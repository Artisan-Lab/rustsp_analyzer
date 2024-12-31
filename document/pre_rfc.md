![image](https://github.com/user-attachments/assets/d6c38c49-e9ca-4ef1-bce1-8d5ea5388e0c)![image](https://github.com/user-attachments/assets/56ce53a2-71a7-4f46-a41b-c47dd3c64c7d)# Privimitive Safety Properties for Rust Contract Design

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

$$\text{addressof}(\text{instance}(T)) \text{ % } \text{alignment(T)} = 0$$

If requiring a pointer $p$ of type T* to be aligned, the property can be formularized as:
$$p \text{ \% } \text{alignment(T)} = 0$$

An example API is [ptr::read()](https://doc.rust-lang.org/nightly/std/ptr/fn.read.html).

#### Size 
The size of a value is the offset in bytes between successive elements in an array with that item type including alignment padding. It is always a multiple of its alignment (including 0), i.e., $\text{sizeof}(T) \text{ \% } \text{alignment}(T)=0$. 

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

Non-Null
This property requires the pointer address should not be null for non zero-sized objects. The address of a null pointer is undefined. Note that accodring to the document of [safety](https://doc.rust-lang.org/std/ptr/index.html#safety), a null pointer to a zero-sized object is valid. This property is mainly related to the [NonNull](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) struct and the [ptr::null()](https://doc.rust-lang.org/std/ptr/fn.null.html) function. 
$$p\text{ is defined and } p! = 0 $$

[API: new_unchecked](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked)

Non-Dangling 
The pointer should point to a valid memory address that has not been deallocated in the heap or is valid in the stack. According to [exotically-sized-types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#exotically-sized-types), a pointer to a zero-sized types should also be non-dangling (not sure, should be confirmed).

$$\text{Memory}(p)\text{ is allocated or } p > \text{Address}(stack pointer) $$

[API: get_unchecked](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html#tymethod.get_unchecked)

Other Pointer Validity Requirements
**Not wild**: the pointer should be initialized and point to an allocated memory space. 

#### Bounded

#### Overlap

#### Allocator


## Content
### Initialized (Pre condition)
The content of the object memory must be initiated, at least partially.

[assume_init](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)

### Typed (Precondition or Postcondition)
The content of the object memory must be fully initiated according to the type. This can be either a precondition or a postcondition

For example, the API [read](https://doc.rust-lang.org/std/primitive.pointer.html#method.read) has a precondition that the pointed memory should be typed. On the other hand, the API [alloc_zeored](https://doc.rust-lang.org/std/alloc/fn.alloc_zeroed.html) create a threat that the resulting object is not typed. 

## Dereferencable
Accodring to the document of [safety](https://doc.rust-lang.org/std/ptr/index.html#safety), dereferencable implies pointer validity and aligned. Besides, it also requires the memory range of the given size starting at the pointer is bounded within a single allocated object.

According to the official document [exotically-sized-types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#exotically-sized-types), dereferencable implies nonnull.

[copy_from](https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_from)

## Numbers and Strings
### No Numerical Overflow
The relationship expressions based on numerical operations exhibit clear numerical boundaries. The terms of the expressions can be constants, variables, or the return values of function calls. There are six relational operators including EQ, NE, LT, GT, LE, and GE.

[offset_from](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from-1)

$$\forall x, y \in Values, \, \forall O \in \{EQ, NE, LT, GT, LE, GE\}, \text{ValidRelationalExpression}(x, y, O) $$

### Encoded String:

The encoding format of the string includes UTF-8 string, ASCII string (in bytes), and C-compatible string (nul-terminated trailing with no nul bytes in the middle).

[from_utf8_unchecked](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked)

## Concurrency
### Send
The type can be transferred across threads.

[trati: Send](https://doc.rust-lang.org/std/marker/trait.Send.html)

### Sync 
The type can be safe to share references between threads.
[trait: Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)

## Pinned (Postcondition? both pre and post?)
A vulnerable state that the value may be moved.

[new_unchecked](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)

## Ownership
### Aliased (post condition)
The API leads to a bad state that an object has multiple mutable references.

[as_mut](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut)

### DualOwned (pre or post condition)
It may create multiple overlapped owners in the ownership system that share the same memory via retaking the owner or creating a bitwise copy.

[from_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)

### Mutated (pre or post condition)
The API leads to a bad state that an object owned by an immutable binding (under certain circumstances) could be be mutated through other mutable bindings.

[as_ref](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref-1)

### Lifetime (pre or post condition) 
the lifetime of the returned reference must be shorter than the object pointed by the ptr.

[from_ptr](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

$$\forall v \in Values, T \in Types, \text{Outlived}(v, T) \Leftrightarrow \left( \exists L, \text{ArbitraryLifetime}(L) \land \text{LifetimeExceedsMemory}(v, L) \right)$$

