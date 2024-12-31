# Privimitive Safety Properties for Rust Contract Design

This document proposes a draft that defines the basic safety properties useful for contract definition. Note that the Rust community is advancing the standardization of contract design, as referenced in the following links. We believe this proposal would be useful to facilitate contract specifications.

[Rust Contracts RFC (Draft)](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2022-11-25-contracts.md)  
[MCP759](https://github.com/rust-lang/compiler-team/issues/759)  
[std-contracts-2025h1](https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html)  

## Overall Idea
In contract design, there are two types of safety properties:

**Precondition**: Safety requirements that must be satisfied before calling an unsafe API.  
**Postcondition**: Traditionally, this refers to properties the system must satisfy after the API call. However, in Rust, it signifies that calling an unsafe API may leave the program in a vulnerable state.  

Sometimes, it can be challenging to classify a safety property as either a precondition or a postcondition. To address this, we further break down safety properties into primitives. Each primitive safety property can serve as either a precondition or a postcondition, depending on the context. The idea also addresses the ambiguity of certain high-level or compound safety properties, such as a ``valid pointer.'' In practice, a valid pointer may need to satisfy several primitive conditions, including being non-null, non-dangling, and pointing to an object of type T. We will elaborate on these details in the sections that follow.

## Primitive Safety Properties
### I. Layout-related Primitives
Refer to the document of [type-layout](https://doc.rust-lang.org/reference/type-layout.html), we define three primitives: alignment, size, and padding.

#### Alignment
Alignment is measured in bytes. It must be at least 1, and is always a power of 2. It can be represented as $2^x, s.t. x\ge 0$. We say the memory address of a Type T is aligned if the address is a multiple of alignment(T). We can formulate an alignment requirement as:

$$\text{addressof}(\text{instance}(T)) \\% \text{alignment}(T) = 0$$

If requiring a pointer $p$ of type T* to be aligned, the property can be formularized as:
$$p \\% \text{alignment}(T) = 0$$

An example API is [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html).

#### Size 
The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. It is always a multiple of its alignment (including 0), i.e., $\text{sizeof}(T) \\% \text{alignment}(T)=0$. 

A safety property may require the size to be not ZST. We can formulate the requirement as $\text{sizeof}(T)!=0$

An example API is the [offset_from](https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from) method of NonNull.

#### Padding 
Padding is the unused space required between successive elements in an array, and it will be considered when calculating the size of the element. For example, the following data structure has 1 byte padding, and its size is 4.
```rust
struct MyStruct { a: u16,  b: u8 } // alignment: 2; padding 1
mem::size_of::<MyStruct>(); // size: 4
```

A safety property may require the type T has no padding. We can formulate the requirement as $\text{padding}(T)!=0$

An example API is the intrinsic [raw_eq](https://doc.rust-lang.org/std/intrinsics/fn.raw_eq.html) function.

### II. Pointer-related Primitives

#### Validity
Refering to the documents about [pointer validity](https://doc.rust-lang.org/std/ptr/index.html#safety), whether a pointer is valid depends on the context of pointer usage, and there are several specific attributes related to pointer validity, non-null, non-wild, non-dangling, point-to-T.

- Non-null: The pointer address should not be null, and the address of a null pointer is undefined. This attribute is **confusing** at the current stage, see [pull/134912](https://github.com/rust-lang/rust/pull/134912).
- Non-wild: The pointer address should points to a memory address that has been allocated by the system, either on heap or stack. Accessing a wild pointer may triger segmentation fault.
- Non-dangling: The pointer should point to a valid memory address that has not been deallocated in the heap or is valid in the stack. (TO SOLVE: Whehter a dangling pointer to a zero-sized type is valid?).
- Point-to-T: The pointer must point to a memory unit of type T.

We may design the requirement of a valid pointer for a particular API by combining these attributes. 
 $\text{valid}(p) \subseteq {non-null, non-wild non-dangling, point-to-T}$

#### Bounded

#### Overlap

#### Allocator


### Content-related Primitives
#### Initialization

#### Integer

#### String

#### Unwrap

### Alias-related Primitives

#### Onwership

#### Lifetime

#### Alias

### Advanced Primitives

#### Trait

#### Thread-Safe

#### Pin

#### I/O
opened, volatile

