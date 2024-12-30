# Privimitive Safety Properties for Rust Contract Design

This document proposes a draft that defines the basic safety properties useful for contract definition. Note that the Rust community is advancing the standardization of contract design, as referenced in the following links. We believe this proposal would be useful to facilitate contract specifications.

[Rust Contracts RFC (Draft)](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2022-11-25-contracts.md).  
[MCP759](https://github.com/rust-lang/compiler-team/issues/759)  
[std-contracts-2025h1](https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html)

## Overall Idea
In contract design, there are two types of safety properties:

**Precondition**: Safety requirements that must be satisfied before calling an unsafe API.  
**Postcondition**: Traditionally, this refers to properties the system must satisfy after the API call. However, in Rust, it signifies that calling an unsafe API may leave the program in a vulnerable state.  

Sometimes, it can be challenging to classify a safety property as either a precondition or a postcondition. To address this, we further break down safety properties into primitives. Each primitive safety property can serve as either a precondition or a postcondition, depending on the context. The idea also addresses the ambiguity of certain high-level or compound safety properties, such as a "valid pointer." In practice, a valid pointer must satisfy several primitive conditions, including being non-null, non-dangling, and pointing to an object of type T. We will elaborate on these details in the sections that follow.

## Primitive Safety Properties
### Layout-related Primitives
Refer to the document of [type-layout](https://doc.rust-lang.org/reference/type-layout.html).
### Aligned 
According to the official document [type-layout](https://doc.rust-lang.org/reference/type-layout.html), aligned means the memory address to store a value of alignment n must only be a multiple of n. Alignment is measured in bytes, and must be at least 1, and always a power of 2. 

If requiring a signle pointer $p$ to be alligned, the property can be formularized as:

$$p \text{\\%} \text{sizeof}(*p) = 0 $$

If requiring two pointers $p1$ and $p2$ to be alligned, the property can be formularized as(TO BE FIXED):

$$p1 \text{\\%} \text{sizeof}(*p1) = p2 \text{\\%} \text{sizeof}(*p2) $$

An example api is[swap](https://doc.rust-lang.org/std/ptr/fn.swap.html).

### Sized 
According to the official document [Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html) and [type-layout](https://doc.rust-lang.org/reference/type-layout.html), it means the size of the type is known at compile time. The size of a value is always a multiple (including 0) of its alignment. 

In particular, Dynamically Sized Types (DST) are not sized, such as trait objects and slices; Zero Sized Types (ZST) is sized.

If requiring a value $v$ of type T to be sized, the property can be formularized as:

$$\text{Sizeof}(v) = \text{Constant}(c) \text{ and } \text{Ptr}(v) \text{\\%} \text{Sizeof}(v) = 0$$

[API: size_of_raw](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)

## Pointer Validity

### Non-Null
This property requires the pointer address should not be null for non zero-sized objects. The address of a null pointer is undefined. Note that accodring to the document of [safety](https://doc.rust-lang.org/std/ptr/index.html#safety), a null pointer to a zero-sized object is valid. This property is mainly related to the [NonNull](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) struct and the [ptr::null()](https://doc.rust-lang.org/std/ptr/fn.null.html) function. 
$$p\text{ is defined and } p! = 0 $$

[API: new_unchecked](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked)

### Non-Dangling 
The pointer should point to a valid memory address that has not been deallocated in the heap or is valid in the stack. According to [exotically-sized-types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#exotically-sized-types), a pointer to a zero-sized types should also be non-dangling (not sure, should be confirmed).

$$\text{Memory}(p)\text{ is allocated or } p > \text{Address}(stack pointer) $$

[API: get_unchecked](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html#tymethod.get_unchecked)

### Other Pointer Validity Requirements
**Not wild**: the pointer should be initialized and point to an allocated memory space. 

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

## More
**Unreachable**: The specific value will trigger unreachable data flow, such as enumeration index (variance), boolean value, etc.
```rust
impl<T> Option<T>::unwrap_unchecked
```
[unwrap_unchecked](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked)

$$\forall v \in Values, T \in Types, \text{Unreachable}(v, T) \Leftrightarrow \left( \text{TriggersUnreachableFlow}(v, T) \right)$$

$\text{TriggersUnreachableFlow}(v, T)$：值 $v$ 是否会导致不可达的数据流或控制流路径。例如，在枚举类型中，如果一个值代表了一个不可能的枚举成员，或者布尔值总是为`true`或`false`（取决于上下文），则它触发不可达路径。

---
**SystemIO**: The variable is related to the system IO and depends on the target platform, including TCP sockets, handles, and file descriptors.
```rust
trait FromRawFd::from_raw_fd
```
[trait: from_raw_fd](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html)

$$forall v \in Variables, T \in Types, \text{SystemIO}(v, T) \Leftrightarrow \left( \text{TCPSockets}(v, T) \lor \text{Handles}(v, T) \lor \text{FileDescriptors}(v, T) \right)$$

$\text{TCPSockets}(v, T)$：变量 $v$ 是否代表一个TCP套接字，该套接字的行为由操作系统的网络栈管理。 $\text{Handles}(v, T)$ ：变量 $v$ 是否代表一个操作系统句柄，如进程句柄、窗口句柄或其他系统级资源的句柄 $\text{FileDescriptors}(v, T)$ ：变量 $v$ 是否代表一个文件描述符，通常用于表示文件、管道或套接字等文件系统资源。

**Freed**:The value may be manually freed or automated dropped.
```rust
impl<T: ?Sized> ManuallyDrop<T>::drop
```
[API: drop](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html#method.drop)

$$\forall v \in Values, T \in Types, \text{Freed}(v, T) \Leftrightarrow \left( \text{ManuallyFreed}(v, T) \lor \text{AutomaticallyDropped}(v, T) \right)$$

$\text{ManuallyFreed}(v, T)$：值 $v$ 是否已经通过手动释放导致内存被回收，内存区域已经不再有效。 $\text{AutomaticallyDropped}(v, T)$ ：值 $v$ 是否已经通过自动释放机制`drop()`被销毁，自动内存管理系统会在所有权转移或作用域结束时自动丢弃值。

---
**Leaked**: The value may be leaked or escaped from the ownership system.
```rust
impl<T: ?Sized> *mut T::write
```
[API: write](https://doc.rust-lang.org/std/primitive.pointer.html#method.write)

$$\forall v \in Values, T \in Types, \text{Leaked}(v, T) \Leftrightarrow \left( \text{EscapedFromOwnership}(v, T) \lor \text{MemoryLeak}(v, T) \right)$$

$\text{EscapedFromOwnership}(v, T)$：值 $v$ 是否逃逸了所有权系统，意味着该值的所有权被转移或丢失，导致无法再正确管理。可能的情况包括值被传递给外部代码、存储在全局或静态变量中，或者被错误地从所有权管理中移除。 
 $\text{MemoryLeak}(v, T)$ ：值 $v$ 是否导致了内存泄漏，意味着占用的内存没有在预期的时间内释放，导致系统资源无法有效回收。
