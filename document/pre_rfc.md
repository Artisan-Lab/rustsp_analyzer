<img width="567" alt="image" src="https://github.com/user-attachments/assets/ecb22c45-a245-4450-8e76-35b578ed27d7" /><img width="631" alt="image" src="https://github.com/user-attachments/assets/e9824650-3f05-45ea-a4a3-d6cd9234505b" /><img width="421" alt="image" src="https://github.com/user-attachments/assets/e8a2e086-7e2d-4c09-812b-bdcd32e7aaa7" /><img width="511" alt="image" src="https://github.com/user-attachments/assets/39febbef-6ec1-4e6b-8e40-72ed13bbab81" /><img width="525" alt="image" src="https://github.com/user-attachments/assets/bd01dfb4-c3fa-4198-a714-4cfce7a72312" /><img width="648" alt="image" src="https://github.com/user-attachments/assets/6eda9b26-6d36-4d7c-b1f8-0baf63fd60b5" /><img width="498" alt="image" src="https://github.com/user-attachments/assets/438b5eb6-58a5-4434-926f-91a2eab45032" /><img width="644" alt="image" src="https://github.com/user-attachments/assets/f1538166-f7d8-4746-b0e0-c40a3388d0be" /><img width="561" alt="image" src="https://github.com/user-attachments/assets/e41b965b-d42d-422e-9c60-e61712f386b9" /># Privimitive Safety Properties for Rust Contract Design

This document proposes a draft that defines the basic safety properties useful for contract definition. Note that the Rust community is advancing the standardization of contract design, as referenced in the following links. We believe this proposal would be useful to facilitate contract specifications.

[Rust Contracts RFC (draft)](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2022-11-25-contracts.md)  
[MCP759](https://github.com/rust-lang/compiler-team/issues/759)  
[std-contracts-2025h1](https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html)  

## Overall Idea
In contract design, there are two types of safety properties:

**Precondition**: Safety requirements that must be satisfied before calling an unsafe API.  
**Postcondition**: Traditionally, this refers to properties the system must satisfy after the API call. However, in Rust, it signifies that calling an unsafe API may leave the program in a vulnerable state.  

Sometimes, it can be challenging to classify a safety property as either a precondition or a postcondition. To address this, we further break down safety properties into primitives. Each primitive safety property can serve as either a precondition or a postcondition, depending on the context. The idea also addresses the ambiguity of certain high-level or compound safety properties, such as a ``valid pointer.'' In practice, a valid pointer may need to satisfy several primitive conditions, including being non-null, non-dangling, and pointing to an object of type T. We will elaborate on these details in the sections that follow.

## Safety Properties
### I. Layout-related Primitives
Refer to the document of [type-layout](https://doc.rust-lang.org/reference/type-layout.html), we define three primitives: alignment, size, and padding.

#### a) Alignment
Alignment is measured in bytes. It must be at least 1, and is always a power of 2. It can be represented as $2^x, s.t. x\ge 0$. We say the memory address of a Type T is aligned if the address is a multiple of alignment(T). We can formulate an alignment requirement as:

$$\text{addressof}(\text{instance}(T)) \\% \text{alignment}(T) = 0$$

If requiring a pointer $p$ of type T* to be aligned, the property can be formularized as:

$$p \\% \text{alignment}(T) = 0$$

Example API: [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html).

#### b) Size 
The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. It is always a multiple of its alignment (including 0), i.e., $\text{sizeof}(T) \\% \text{alignment}(T)=0$. 

A safety property may require the size to be not ZST. We can formulate the requirement as 

$$\text{sizeof}(T) > 0$$

Example API: [NonNull.offset_from](https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from)

#### c) Padding 
Padding is the unused space required between successive elements in an array, and it will be considered when calculating the size of the element. For example, the following data structure has 1 byte padding, and its size is 4.
```rust
struct MyStruct { a: u16,  b: u8 } // alignment: 2; padding 1
mem::size_of::<MyStruct>(); // size: 4
```

A safety property may require the type T has no padding. We can formulate the requirement as 

$$\text{padding}(T)=0$$

Example API: intrinsic [raw_eq()](https://doc.rust-lang.org/std/intrinsics/fn.raw_eq.html)

### II. Pointer Validity

Refering to the documents about [pointer validity](https://doc.rust-lang.org/std/ptr/index.html#safety), whether a pointer is valid depends on the context of pointer usage, and the criteria varies for different APIs. To better descript the pointer validity and avoid ambiguity, we breakdown the concept related pointer validity into several primitives. 

#### d) Address (Primitive)
The memory address that the pointer points to. A safety property may require the pointer address to be null, namely ``non-null``, because the address of a null pointer is undefined. We can fomulate the property as 

$$ p != null $$

Example API: [NonNull::new_unchecked()](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked), [Box::from_non_null()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_non_null)

#### e) Allocation (Primitive)
To indicate whether the memory address pointed by the pointer is available to use or has been allocated by the system, either on heap or stack. There is a related safety requirements non-dangling, which means the pointer should point to a valid memory address that has not been deallocated in the heap or is valid in the stack. We can fomulate the requirement as 

$$ \text{alloca}(p) \in \lbrace GlobalAllocator, OtherAllocator, stack \rbrace $$

Example API: [ptr::offset()](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset-1)

Besides, some properties may require the allocator to be consistent, i.e., the memory address pointed by the pointer p should be allocated by a specific allocator, like the GlobalAllocator.

$$ \text{alloca}(p) = GlobalAllocator $$

Example API: [Arc::from_raw()](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw), [Arc::from_raw_in()](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw_in), [Box::from_raw_in()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw_in)

#### f) Point-to (Primitive)
A safety property may require the pointer point to a value of a particular type. We can fomulate the property as 

$$ \text{typeof}(*p) = T $$

Point-to implies non-dangling and non-null(not sure, need to be confirmed).

#### Derived Safety Properties
There are two useful derived safety properties based on the primitives.

**Bounded Address (derived)**
$$ \text{typeof}(*(p + \text{sizeof}(T) * offset))  = T $$

Example API: [ptr::offset()](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset-1)

**Overlap (derived)**

A safety property may require the two pointers do not overlap with respect to T: 

$$ |p_{dst} - p_{src}| > \text{sizeof}(T)$$

Example API: [ptr::copy()](https://doc.rust-lang.org/std/ptr/fn.copy.html) 

It may also require the two pointers do not overlap with respect to $T\times n$ : 

$$ |p_{dst} - p_{src}| > \text{sizeof}(T) * n $$

Example API: [ptr::copy_nonoverlapping()](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html)
 
### Content-related Primitives

#### g) Initialization
A memory of type T pointed by a pointer is either initialized or not. This is a binary primitive.

$$init(*p)\in {true, false}$$

Example API: [MaybeUninit.assume_init()](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#method.assume_init), [Box::assume_init()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)

#### h) Integer

$$ {true, false}$$

Example API: [pointer.add()](https://doc.rust-lang.org/std/primitive.pointer.html#method.add)

#### i) String

Example API: [String::from_utf8_unchecked()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked), [CStr::from_ptr()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

#### j) Unwrap

$$enum(T)\in {Ok, Err, Some, None}\$$

Example API: [Option::unwrap_unchecked()](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked), [Result::unwrap_unchecked()](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_unchecked), [Result::unwrap_err_unchecked()](https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_err_unchecked)

### Alias-related Primitives

#### k) Onwership

Example API: [Box::from_raw()](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw), [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html)

#### l) Lifetime

Example API: [CStr::from_ptr()](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

#### m) Alias

Example API: [pointer.as_mut()](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut)

### Advanced Primitives

#### n) Trait

Example API: [ptr::read()](https://doc.rust-lang.org/std/ptr/fn.read.html), [ptr::read_volatile()](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html)

#### o) Thread-Safe

Example API: Auto trait [Send](https://doc.rust-lang.org/std/marker/trait.Send.html), [Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)

#### p) Pin

Example API: [Pin::new_unchecked()](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)

#### q) I/O

Example API: [trait.FromRawFd::from_raw_fd()](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd), [UdpSocket::from_raw_socket()](https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_socket)

