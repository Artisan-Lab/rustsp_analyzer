# Safety Property
Their are two types of safety properties:  
**Precondition**: the safety requirements need to be satisfied before calling an unsafe API.  
**Postcondition**: calling the unsafe API leads the program to a vulnerable state.  

## Layout
Refer to the document of [type-layout](https://doc.rust-lang.org/reference/type-layout.html).
### Aligned 
According to the official document [type-layout](https://doc.rust-lang.org/reference/type-layout.html), aligned means the memory address to store a value of alignment n must only be a multiple of n. Alignment is measured in bytes, and must be at least 1, and always a power of 2. 

If requiring a signle pointer $p$ to be alligned, the property can be formularized as:

$$p \text{\\%} \text{sizeof}(*p) = 0 $$

If requiring two pointers $p1$ and $p2$ to be alligned, the property can be formularized as(TO BE FIXED):

$$p1 \text{\\%} \text{sizeof}(*p1) = p2 \text{\\%} \text{sizeof}(*p2) $$

An example api is[swap](https://doc.rust-lang.org/std/ptr/fn.swap.html).

(TO BE FIXED) requirement about **padding**?

### Sized 
According to the official document [Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html) and [type-layout](https://doc.rust-lang.org/reference/type-layout.html), it means the size of the type is known at compile time. The size of a value is always a multiple (including 0) of its alignment. 

In particular, Dynamically Sized Types (DST) are not sized, such as trait objects and slices; Zero Sized Types (ZST) is sized.

If requiring a value $v$ of type T to be sized, the property can be formularized as:

$$\text{Sizeof}(v) = \text{Constant}(c) \text{ and } \text{Ptr}(v) \text{\\%} \text{Sizeof}(v) = 0$$

[API: size_of_raw](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)

## Pointer Validity

### Non-Null
This property requires the pointer address should not be null. A null pointer is undefined. This property is mainly related to the [NonNull](https://doc.rust-lang.org/std/ptr/struct.NonNull.html) struct and the [ptr::null()](https://doc.rust-lang.org/std/ptr/fn.null.html) function.

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

[API: assume_init](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)

### Typed (Precondition or Postcondition)
The content of the object memory must be fully initiated according to the type. This can be either a precondition or a postcondition

For example, the API [read](https://doc.rust-lang.org/std/primitive.pointer.html#method.read) has a precondition that the pointed memory should be typed. On the other hand, the API [alloc_zeored](https://doc.rust-lang.org/std/alloc/fn.alloc_zeroed.html) create a threat that the resulting object is not typed. 

## Dereferencable
Dereferencable implies pointer validity and aligned.
According to the official document [exotically-sized-types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#exotically-sized-types), dereferencable implies nonnull.

The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object.

[API: copy_from](https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_from)

$$ \forall p \in P, n \in \mathbb{N}, \text{Dereferencable}(p, n) \Leftrightarrow \left( \exists O \in Objects \, | \, \text{Allocated}(O) \land \text{WithinBounds}(p, n, O) \right)$$

$\text{Dereferencable}(p, n)$：指针 $p$ 是否指向一块大小为 $n$ 的内存区域，并且该区域是合法的、可以被解引用的。 $\text{Allocated}(O)$ ：对象 $O$ 是否已经分配。 $\text{WithinBounds}(p, n, O)$ ：指针 $p$ 所指向的内存区域从 $p$ 开始，长度为 $n$ ，是否完全位于已分配对象 $O$ 的内存范围内。

## Numbers and Strings
### No Numerical Overflow
The relationship expressions based on numerical operations exhibit clear numerical boundaries. The terms of the expressions can be constants, variables, or the return values of function calls. There are six relational operators including EQ, NE, LT, GT, LE, and GE.

[API: offset_from](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from-1)

$$\forall x, y \in Values, \, \forall O \in \{EQ, NE, LT, GT, LE, GE\}, \text{ValidRelationalExpression}(x, y, O) $$

$\text{ValidRelationalExpression}(x, y, O)$：对于给定的数值（或子表达式） $x$ 和 $y$ ，以及运算符 $O$ ，表达式 $xOy$ 必须是合法且有效的关系表达式。关系运算符 $O$ 只能是上述六个关系运算符之一。关系操作结果必须为布尔值，表示 $x$ 和 $y$ 之间的关系。

### Encoded String:

The encoding format of the string includes UTF-8 string, ASCII string (in bytes), and C-compatible string (nul-terminated trailing with no nul bytes in the middle).
```rust
impl String::from_utf8_unchecked
```
[API: from_utf8_unchecked](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked)

$$\forall s \in Strings, \text{Encoded}(s) \Leftrightarrow \left( \text{UTF8}(s) \lor \text{ASCII}(s) \lor \text{CCompatible}(s) \right)$$

$\text{UTF8}(s)$：字符串 $s$ 是否符合UTF-8编码格式。 $\text{ASCII}(s)$ ：字符串 $s$ 是否仅包含ASCII字符，并以字节数组形式存储。 $\text{CCompatible}(s)$ ：字符串 $s$ 是否符合C兼容字符串格式，即以空字符结尾且不包含中间的空字符。


---
**SystemIO**: The variable is related to the system IO and depends on the target platform, including TCP sockets, handles, and file descriptors.
```rust
trait FromRawFd::from_raw_fd
```
[trait: from_raw_fd](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html)

$$forall v \in Variables, T \in Types, \text{SystemIO}(v, T) \Leftrightarrow \left( \text{TCPSockets}(v, T) \lor \text{Handles}(v, T) \lor \text{FileDescriptors}(v, T) \right)$$

$\text{TCPSockets}(v, T)$：变量 $v$ 是否代表一个TCP套接字，该套接字的行为由操作系统的网络栈管理。 $\text{Handles}(v, T)$ ：变量 $v$ 是否代表一个操作系统句柄，如进程句柄、窗口句柄或其他系统级资源的句柄 $\text{FileDescriptors}(v, T)$ ：变量 $v$ 是否代表一个文件描述符，通常用于表示文件、管道或套接字等文件系统资源。

## Concurrency
### Send
The type can be transferred across threads.

[trati: Send](https://doc.rust-lang.org/std/marker/trait.Send.html)

$$\forall T \in Types, \text{Send}(T) \Leftrightarrow \left( \forall v \in Values, \text{ThreadSafe}(v, T) \right)$$

$\text{ThreadSafe}(v, T)$：类型 $T$ 的值 $v$ 是否在线程间传递时是线程安全的。通常， $v$ 不包含可变的、共享的状态，或采用了适当的同步机制（如锁、原子操作等）来确保线程安全。

### Sync 
The type can be safe to share references between threads.
[trait: Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)

$$\forall T \in Types, \text{Sync}(T) \Leftrightarrow \left( \forall v \in Values, \text{SafeSharedReference}(v, T) \right)$$

$\text{SafeSharedReference}(v, T)$：类型 $T$ 的值 $v$ 是否能够在多个线程之间安全地共享引用而不会出现数据竞争或不一致的状态。

## Pinned 
The value may be moved, although it ought to be pinned.

[API: new_unchecked](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)

$\text{ShouldBePinned}(v, T)$：值 $v$ 是否应该被固定在内存中，不应被移动。类型 $T$ 需要保证固定，例如通过`Pin<T>`来防止移动。 $\text{MovedDespitePinning}(v, T)$ ：值 $v$ 是否被错误地移动，尽管它应该被固定。移动固定值可能导致不可预期的行为，尤其是在涉及到非堆分配的类型时。


## Ownership
### Aliased (post condition)
The value may have multiple mutable references or simultaneously have mutable and shared references.

[API: as_mut](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut)

$$\forall v \in Values, T \in Types, \text{Aliased}(v, T) \Leftrightarrow \left( \text{MultipleMutableReferences}(v, T) \lor \text{MutableAndSharedReferences}(v, T) \right)$$

$\text{MultipleMutableReferences}(v, T)$：值 $v$ 可能有多个可变引用，通常违反Rust强制要求每次只有一个可变引用的别名规则。 $\text{MutableAndSharedReferences}(v, T)$ ：值 $v$ 可能同时有可变引用和共享引用，不允许在同一作用域内同时存在可变引用和共享引用，因为这可能导致数据不一致。

### Mutated (post condition)
The value, which is owned by an immutable binding or pointed by shared reference, may be mutated.

[API: as_ref](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref-1)

$$\forall v \in Values, T \in Types, \text{Mutated}(v, T) \Leftrightarrow \left( \text{OwnedByImmutableBinding}(v, T) \land \text{SharedReferenceToMutableValue}(v, T) \right)$$

$\text{OwnedByImmutableBinding}(v, T)$：值 $v$ 由不可变绑定拥有，即该值的所有权属于不可变变量。 $\text{SharedReferenceToMutableValue}(v, T)$ ：值 $v$ 通过共享引用访问，但其内部状态仍然可能被修改。

### Outlived (post condition) 
The arbitrary lifetime (unbounded) that becomes as big as context demands may outlive the pointed memory.

[API: from_ptr](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

$$\forall v \in Values, T \in Types, \text{Outlived}(v, T) \Leftrightarrow \left( \exists L, \text{ArbitraryLifetime}(L) \land \text{LifetimeExceedsMemory}(v, L) \right)$$

$\text{ArbitraryLifetime}(L)$：表示存在一个任意的生命周期 $L$ ，其长度可以根据上下文的需求而变长或变短，且通常没有具体的上限。 $\text{LifetimeExceedsMemory}(v, L)$ ：值 $v$ 的生命周期 $L$ 是否超出了它所指向内存的生命周期，即在内存被释放之后，生命周期依然被延长，可能导致引用失效。

### DualOwned**
It may create multiple overlapped owners in the ownership system that share the same memory via retaking the owner or creating a bitwise copy.

[API: from_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)

$$\forall v \in Values, \text{DualOwned}(T) \Leftrightarrow \left( \exists v_1, v_2 \in Values, \text{OverlappingOwners}(v_1, v_2, T) \right)$$

$\text{OverlappingOwners}(v_1, v_2, T)$：类型为 $T$ 的两个值 $v_1$ 和 $v_2$ 同时拥有对同一内存块的所有权。通过复原所有权（如通过`from_raw`）或者通过创建位拷贝对象（如`read()`）来实现。

## More
**Unreachable**: The specific value will trigger unreachable data flow, such as enumeration index (variance), boolean value, etc.
```rust
impl<T> Option<T>::unwrap_unchecked
```
[API: unwrap_unchecked](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked)

$$\forall v \in Values, T \in Types, \text{Unreachable}(v, T) \Leftrightarrow \left( \text{TriggersUnreachableFlow}(v, T) \right)$$

$\text{TriggersUnreachableFlow}(v, T)$：值 $v$ 是否会导致不可达的数据流或控制流路径。例如，在枚举类型中，如果一个值代表了一个不可能的枚举成员，或者布尔值总是为`true`或`false`（取决于上下文），则它触发不可达路径。

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
