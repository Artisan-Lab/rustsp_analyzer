**Non-Null**: A null pointer is never valid, not even for accesses of size zero.
```rust
impl<T: Sized> NonNull<T>::new_unchecked
```
[API: new_unchecked](https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.new_unchecked)

---
**Non-Dangling**: The value must not be pointing to the deallocated memory even for operations of size zero, including data stored in the stack frame and heap chunk.
```rust
trait SliceIndex<T: ?Sized>::get_unchecked
```
[API: get_unchecked](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html#tymethod.get_unchecked)

---
**Numerical**: The relationship expressions based on numerical operations exhibit clear numerical boundaries. The terms of the expressions can be constants, variables, or the return values of function calls. There are six relational operators including EQ, NE, LT, GT, LE, and GE.
```rust
impl<T: ?Sized> *mut T::offset_from {}
```
[API: offset_from](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from-1)

---
**Dereferencable**: The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object.
```rust
impl<T: ?Sized> *mut T::copy_from
```
[API: copy_from](https://doc.rust-lang.org/std/primitive.pointer.html#method.copy_from)

---
**Initialized**: The value that has been initialized can be divided into two scenarios: fully initialized and partially initialized.
```rust
impl<T> MaybeUninit<T>::assume_init
```
[API: assume_init](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init)

---
**Typed**: The bit pattern of the initialized value must be valid at the given type and uphold additional invariants for generics.
```rust
impl<T: ?Sized> *mut T::read
```
[API: read](https://doc.rust-lang.org/std/primitive.pointer.html#method.read)

---
**Encoded**: The encoding format of the string includes UTF-8 string, ASCII string (in bytes), and C-compatible string (nul-terminated trailing with no nul bytes in the middle).
```rust
impl String::from_utf8_unchecked
```
[API: from_utf8_unchecked](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_unchecked)

---
**Sized**: The restrictions on Exotically Sized Types (EST), including Dynamically Sized Types (DST) that lack a statically known size, such as trait objects and slices; and Zero Sized Types (ZST) that occupy no space.
```rust
core::mem::size_of_raw
```
[API: size_of_raw](https://doc.rust-lang.org/std/mem/fn.size_of_val.html)

---
**Aligned**: The value is properly aligned via a specific allocator or the attribute #[repr], including the alignment and the padding.
```rust
impl<T: ?Sized> *mut T::swap
```
[API: swap](https://doc.rust-lang.org/std/ptr/fn.swap.html)

---
**Fitted**: The layout (including size and alignment) must be the same layout that was used to allocate that block of memory.
```rust
trait GlobalAlloc::dealloc
```
[API: dealloc](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#tymethod.dealloc)

---
**SystemIO**: The variable is related to the system IO and depends on the target platform, including TCP sockets, handles, and file descriptors.
```rust
trait FromRawFd::from_raw_fd
```
[trait: from_raw_fd](https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html)

---
**Send**: The type can be transferred across threads.
```rust
core::marker::Send
```
[trati: Send](https://doc.rust-lang.org/std/marker/trait.Send.html)


---
**Sync**: The type can be safe to share references between threads.
```rust
core::marker::Sync
```
[trait: Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)

---
**Unreachable**: The specific value will trigger unreachable data flow, such as enumeration index (variance), boolean value, etc.
```rust
impl<T> Option<T>::unwrap_unchecked
```
[API: unwrap_unchecked](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_unchecked)

---
**Aliased**: The value may have multiple mutable references or simultaneously have mutable and shared references.
```rust
impl<T: ?Sized> *mut T::as_mut
```
[API: as_mut](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut)

---
**Mutated**: The value, which is owned by an immutable binding or pointed by shared reference, may be mutated.
```rust
impl<T: ?Sized> *const T::as_ref
```
[API: as_ref](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref-1)

---
**Outlived**: The arbitrary lifetime (unbounded) that becomes as big as context demands may outlive the pointed memory.
```rust
impl CStr::from_ptr
```
[API: from_ptr](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_ptr)

---
**DualOwned**: It may create multiple overlapped owners in the ownership system that share the same memory via retaking the owner or creating a bitwise copy.
```rust
impl<T: ?Sized> Box<T>::from_raw
```
[API: from_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)

---
**Untyped**: The value may not be in the initialized state, or the byte pattern represents an invalid value of its type.
```rust
core::alloc::alloc_zeored
```
[API: alloc_zeored](https://doc.rust-lang.org/std/alloc/fn.alloc_zeroed.html)

---
**Freed**:The value may be manually freed or automated dropped.
```rust
impl<T: ?Sized> ManuallyDrop<T>::drop
```
[API: drop](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html#method.drop)

---
**Leaked**: The value may be leaked or escaped from the ownership system.
```rust
impl<T: ?Sized> *mut T::write
```
[API: write](https://doc.rust-lang.org/std/primitive.pointer.html#method.write)

---
**Pinned**: The value may be moved, although it ought to be pinned.
```rust
impl<P: Deref> Pin<P>::new_unchecked
```
[API: new_unchecked](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked)