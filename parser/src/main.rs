use serde::{Deserialize, Serialize};
use serde_json::{Value};

use std::collections::HashMap;
use std::fs;

type Identifier = String;
type Filed = String;
type Sp = String;
type Description = Vec<String>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SpTri {
   sp: Sp,
   f: Filed,
   des: Description,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SpMapping {
   map: HashMap<Identifier, Vec<SpTri>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SpMappingMerge {
   map: HashMap<Identifier, String>,
}

fn remove_prefix(s: String) -> String {
   if let Some(index) = s.find("core/") {
       // 包含"core"的部分及其之前的字符长度
       let length_to_remove = index + "core/".len();
       // 返回从length_to_remove位置开始到字符串末尾的切片
       s[length_to_remove..].to_owned()
   } else if let Some(index) = s.find("std/") {
      let length_to_remove = index + "std/".len();
      // 返回从length_to_remove位置开始到字符串末尾的切片
      s[length_to_remove..].to_owned()
   } else if let Some(index) = s.find("alloc/") {
      let length_to_remove = index + "alloc/".len();
      // 返回从length_to_remove位置开始到字符串末尾的切片
      s[length_to_remove..].to_owned()
   } else {
      // 如果没有找到"core"，则返回原始字符串
      unreachable!()
   }
}

use core::ptr::read;

fn main() {
   let j:Value = serde_json::from_str(JSON).unwrap();
   let mut sp_mapping = SpMapping::default();
   let mut sp_mapping_merge = SpMappingMerge::default();

   let j_funcs = &j["unsafe_fn"];
   match j_funcs {
      Value::Object(ref j_map) => {
         
         // iterate all identifer (weblinks)
         for func in j_map {
            let id = remove_prefix(func.0.to_string());
            //sp4func records all field - sp - doc pairs
            let sp4func = func.1.as_object().unwrap();
            let mut sp_tris:Vec<SpTri> = Vec::default();
            
            // iterate all fields
            for triplet in sp4func {
               let field = triplet.0;
               if field == "row" { continue; }
               // sps records all sp - doc for one field
               let sps = triplet.1.as_object().unwrap();

               for pair in sps {
                  let sp = pair.0;
                  let docs = pair.1.as_array().unwrap();

                  let mut sp_tri = SpTri::default();
                  sp_tri.f = field.to_string();
                  sp_tri.sp = sp.to_string();
                  for doc in docs {
                     sp_tri.des.push(doc.as_str().unwrap().to_string());
                  }
                  sp_tris.push(sp_tri);
               }
            }

            sp_mapping.map.insert(id, sp_tris);
         }
      },
      _ => panic!(),
   }

   for elem in &sp_mapping.map {
      let mut ans = String::new();
      for tri in elem.1 {
         ans.push_str(&format!("‼️`{}` **{}**:\n ", &tri.f, &tri.sp) );
         for d in &tri.des {
            ans.push_str(&format!("> _{}_\n\n ", d));
         }
         ans.push_str("\n");
      }
      ans.push_str("\n");
      sp_mapping_merge.map.insert(elem.0.to_owned(), ans);
   }

   let output = serde_json::to_string(&sp_mapping_merge).unwrap();
   fs::write("spmap.json", output).unwrap();
}

static JSON:&'static str = r#"{
   "unsafe_fn": {
      "https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html#tymethod.alloc": {
         "row": 3,
         "layout": {
            "Layout": [
               "`layout` has non-zero size."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory may or may not be initialized."
            ],
            "Freed": [
               "Returning a null pointer indicates that either memory is exhausted."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html#tymethod.dealloc": {
         "row": 4,
         "ptr": {
            "Allocated": [
               "`ptr` must denote a block of memory currently allocated via this allocator."
            ]
         },
         "layout": {
            "Layout": [
               "`layout` must be the same layout that was used to allocate that block of memory."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html#method.alloc_zeroed": {
         "row": 5,
         "layout": {
            "Layout": [
               "`layout` has non-zero size."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory is guaranteed to be initialized but may be untyped."
            ],
            "Freed": [
               "Returning a null pointer indicates that either memory is exhausted."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html#method.realloc": {
         "row": 6,
         "ptr": {
            "Allocated": [
               "`ptr` must be currently allocated via this allocator."
            ],
            "Freed": [
               "If this returns a non-null pointer, then ownership of the memory block referenced by `ptr` has been transferred to this allocator."
            ]
         },
         "layout": {
            "Layout": [
               "`layout` has non-zero size.",
               "`layout` must be the same layout that was used to allocate that block of memory."
            ]
         },
         "new_size": {
            "Bounded": [
               "`new_size` must be greater than zero.",
               "`new_size`, when rounded up to the nearest multiple of `layout.align()`, must not overflow `isize::MAX`."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory may or may not be initialized."
            ],
            "Freed": [
               "Returning a null pointer indicates that either memory is exhausted."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/struct.Layout.html#method.for_value_raw": {
         "row": 7,
         "t": {
            "Bounded": [
               "If the unsized tail of `T` is a slice, then the length of the slice tail must be an initialized integer, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`.",
               "If the unsized tail of `T` is a trait object, then the vtable part of the pointer must point to a valid vtable for the type `T` acquired by an unsizing coercion, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`."
            ],
            "Layout": [
               "If the unsized tail of `T` is a slice, then the length of the slice tail must be an initialized integer, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`.",
               "If the unsized tail of `T` is a trait object, then the vtable part of the pointer must point to a valid vtable for the type `T` acquired by an unsizing coercion, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/struct.Layout.html#method.from_size_align_unchecked": {
         "row": 8,
         "align": {
            "Bounded": [
               "`align` must not be zero.",
               "`align` must be a power of two."
            ]
         },
         "size": {
            "Bounded": [
               "`size`, when rounded up to the nearest multiple of `align`, must not overflow `isize::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.Allocator.html#tymethod.deallocate": {
         "row": 9,
         "ptr": {
            "Allocated": [
               "`ptr` must denote a block of memory currently allocated via this allocator."
            ]
         },
         "layout": {
            "Layout": [
               "`layout` must fit that block of memory."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.Allocator.html#method.grow": {
         "row": 10,
         "ptr": {
            "Allocated": [
               "`ptr` must denote a block of memory currently allocated via this allocator."
            ],
            "Freed": [
               "If this returns `Ok`, then ownership of the memory block referenced by `ptr` has been transferred to this allocator. "
            ]
         },
         "old_layout": {
            "Layout": [
               "`old_layout` must fit that block of memory."
            ]
         },
         "new_layout": {
            "Bounded": [
               "`new_layout.size()` must be greater than or equal to `old_layout.size()`."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory may or may not be initialized."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.Allocator.html#method.grow_zeroed": {
         "row": 11,
         "ptr": {
            "Allocated": [
               "`ptr` must denote a block of memory currently allocated via this allocator."
            ],
            "Freed": [
               "If this returns `Ok`, then ownership of the memory block referenced by `ptr` has been transferred to this allocator. "
            ]
         },
         "old_layout": {
            "Layout": [
               "`old_layout` must fit that block of memory."
            ]
         },
         "new_layout": {
            "Bounded": [
               "`new_layout.size()` must be greater than or equal to `old_layout.size()`."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory is guaranteed to be initialized but may be untyped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/alloc/trait.Allocator.html#method.shrink": {
         "row": 12,
         "ptr": {
            "Allocated": [
               "`ptr` must denote a block of memory currently allocated via this allocator."
            ],
            "Freed": [
               "If this returns `Ok`, then ownership of the memory block referenced by `ptr` has been transferred to this allocator. "
            ]
         },
         "old_layout": {
            "Layout": [
               "`old_layout` must fit that block of memory."
            ]
         },
         "new_layout": {
            "Bounded": [
               "`new_layout.size()` must be smaller than or equal to `old_layout.size()`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/array/struct.IntoIter.html#method.new_unchecked": {
         "row": 13,
         "buffer": {
            "Initialized": [
               "The `buffer[initialized]` elements must all be initialized."
            ]
         },
         "initialized": {
            "Bounded": [
               "The range must be in-bounds for the buffer, with `initialized.end <= N`.",
               "The range must be canonical, with `initialized.start` <= `initialized.end`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/char/fn.from_u32_unchecked.html": {
         "row": 14,
         "i": {
            "Initialized": [
               "Not all valid `u32`s are valid `char`s, it may construct invalid `char` values."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.char.html#method.from_u32_unchecked": {
         "row": 15,
         "i": {
            "Initialized": [
               "Not all valid `u32`s are valid `char`s, it may construct invalid `char` values."
            ]
         }
      },
      "https://doc.rust-lang.org/core/convert/trait.FloatToInt.html": {
         "row": 16,
         "self": {
            "Bounded": [
               "The value must not be `NaN`.",
               "The value must not be infinite.",
               "The value must be representable in the return type `Int`, after truncating off its fractional part."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ffi/struct.CStr.html#method.from_ptr": {
         "row": 17,
         "ptr": {
            "Bounded": [
               "The nul terminator must be within `isize::MAX` from `ptr`."
            ],
            "Allocated": [
               "`ptr` must be non-null even for a zero-length cstr."
            ],
            "Initialized": [
               "The memory pointed to by `ptr` must contain a valid nul terminator at the end of the string."
            ],
            "Dereferencable": [
               "The entire memory range of this `CStr` must be contained within a single allocated object!"
            ]
         },
         "retval": {
            "Aliased": [
               "The memory referenced by the returned `CStr` must not be mutated for the duration of lifetime `'a`.",
               "The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the context."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ffi/struct.CStr.html#method.from_bytes_with_nul_unchecked": {
         "row": 18,
         "bytes": {
            "Initialized": [
               "The provided slice must be nul-terminated and not contain any interior nul bytes."
            ]
         }
      },
      "https://doc.rust-lang.org/core/iter/trait.Step.html#method.forward_unchecked": {
         "row": 21,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `Self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/iter/trait.Step.html#method.backward_unchecked": {
         "row": 22,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `Self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.char.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `char::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `i8::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `i16::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `i32::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `i64::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `i128::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `u8::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `u16::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `u16::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `u64::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `u128::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `isize::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.forward_unchecked": {
         "row": 23,
         "start": {
            "Bounded": [
               "`start` + `count` may overflow the range of values supported by `usize::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.char.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `char::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `i8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `i16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `i32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `i64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `i128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `u8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `u16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `u32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `u64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `u128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `isize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.backward_unchecked": {
         "row": 24,
         "start": {
            "Bounded": [
               "`start` - `count` may overflow the range of values supported by `usize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/struct.ManuallyDrop.html#method.take": {
         "row": 25,
         "retval": {
            "DualOwned": [
               "This function semantically moves out the contained value without preventing further usage, leaving the state of this container unchanged. It is your responsibility to ensure that this `ManuallyDrop` is not used again."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/struct.ManuallyDrop.html#method.drop": {
         "row": 26,
         "slot": {
            "Freed": [
               "This function runs the destructor of the contained value. The zombie value should not be exposed to safe code after been dropped, and this function should not be called more than once."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.assume_init": {
         "row": 27,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized state."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.assume_init_read": {
         "row": 28,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized state."
            ]
         },
         "retval": {
            "DualOwned": [
               "This function creates a bitwise copy of the contents, regardless whether the contained type implements the `Copy` trait or not. When using multiple copies of the data (by calling `assume_init_read` multiple times, or first calling `assume_init_read` and then `assume_init`), it is your responsibility to ensure that data may indeed be duplicated."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.assume_init_drop": {
         "row": 29,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized state.",
               "All additional invariants of the type `T` must be satisfied, as the `Drop` implementation of `T` (or its members) may rely on this."
            ],
            "Freed": [
               "Drops the contained value in place."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.assume_init_ref": {
         "row": 30,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized state."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.assume_init_mut": {
         "row": 31,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` really is in an initialized state."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.array_assume_init": {
         "row": 32,
         "array": {
            "Initialized": [
               "It is up to the caller to guarantee that all elements of the array are in an initialized state."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.slice_assume_init_ref": {
         "row": 33,
         "slice": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` elements really are in an initialized state."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#method.slice_assume_init_mut": {
         "row": 34,
         "slice": {
            "Initialized": [
               "It is up to the caller to guarantee that the `MaybeUninit<T>` elements really are in an initialized state."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/fn.size_of_val_raw.html": {
         "row": 35,
         "val": {
            "Bounded": [
               "If the unsized tail of `T` is a slice, then the length of the slice tail must be an initialized integer, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`.",
               "If the unsized tail of `T` is a trait object, then the vtable part of the pointer must point to a valid vtable for the type `T` acquired by an unsizing coercion, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`."
            ],
            "Layout": [
               "If the unsized tail of `T` is a slice, then the length of the slice tail must be an initialized integer, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`.",
               "If the unsized tail of `T` is a trait object, then the vtable part of the pointer must point to a valid vtable for the type `T` acquired by an unsizing coercion, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/fn.align_of_val_raw.html": {
         "row": 36,
         "val": {
            "Bounded": [
               "If the unsized tail of `T` is a slice, then the length of the slice tail must be an initialized integer, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`.",
               "If the unsized tail of `T` is a trait object, then the vtable part of the pointer must point to a valid vtable for the type `T` acquired by an unsizing coercion, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`."
            ],
            "Layout": [
               "If the unsized tail of `T` is a slice, then the length of the slice tail must be an initialized integer, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`.",
               "If the unsized tail of `T` is a trait object, then the vtable part of the pointer must point to a valid vtable for the type `T` acquired by an unsizing coercion, and the size of the entire value (dynamic tail length + statically sized prefix) must fit in `isize`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/fn.zeroed.html": {
         "row": 37,
         "retval": {
            "Untyped": [
               "The padding byte is not necessarily zeroed. There is no guarantee that an all-zero byte-pattern represents a valid value of some type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/fn.transmute.html": {
         "row": 38,
         "src": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type.",
               "To transmute the inner type of the contents of a container, you must make sure to not violate any of the container's invariants."
            ],
            "Layout": [
               "Both types must have the same size.",
               "Note that source and destination are passed by-value, which means if `Src` or `Dst` contain padding, that padding is not guaranteed to be preserved by transmute.",
               "When transmuting values that point elsewhere (such as pointers, references, boxes…), the caller has to ensure proper alignment of the pointed-to values."
            ]
         },
         "retval": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type."
            ],
            "Aliased": [
               "It can turn a `*mut T` into an `&mut T`.",
               "It can extend a lifetime, or shorten an invariant lifetime."
            ],
            "Untyped": [
               "It is therefore your responsibility to guarantee that every value passed to transmute is valid at both types `Src` and `Dst`. Failing to uphold this condition may lead to unexpected and unstable compilation results."
            ]
         }
      },
      "https://doc.rust-lang.org/core/mem/fn.transmute_copy.html": {
         "row": 39,
         "src": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type.",
               "To transmute the inner type of the contents of a container, you must make sure to not violate any of the container's invariants."
            ],
            "Layout": [
               "Both types must have the same size.",
               "Note that source and destination are passed by-value, which means if `Src` or `Dst` contain padding, that padding is not guaranteed to be preserved by transmute.",
               "When transmuting values that point elsewhere (such as pointers, references, boxes…), the caller has to ensure proper alignment of the pointed-to values."
            ]
         },
         "retval": {
            "DualOwned": [
               "It will also unsafely create a copy of the contained value instead of moving out of `src`."
            ],
            "Aliased": [
               "It can turn a `*mut T` into an `&mut T`.",
               "It can extend a lifetime, or shorten an invariant lifetime."
            ],
            "Untyped": [
               "It is therefore your responsibility to guarantee that every value passed to transmute is valid at both types `Src` and `Dst`. Failing to uphold this condition may lead to unexpected and unstable compilation results."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.f32.html#method.to_int_unchecked": {
         "row": 40,
         "self": {
            "Bounded": [
               "The value must not be `NaN`.",
               "The value must not be infinite.",
               "The value must be representable in the return type `Int`, after truncating off its fractional part."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.f64.html#method.to_int_unchecked": {
         "row": 41,
         "self": {
            "Bounded": [
               "The value must not be `NaN`.",
               "The value must not be infinite.",
               "The value must be representable in the return type `Int`, after truncating off its fractional part."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > i8::MAX` or `self + rhs < i8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > i16::MAX` or `self + rhs < i16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > i32::MAX` or `self + rhs < i32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > i64::MAX` or `self + rhs < i64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > i128::MAX` or `self + rhs < i128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > u8::MAX` or `self + rhs < u8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > u16::MAX` or `self + rhs < u16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > u32::MAX` or `self + rhs < u32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > u64::MAX` or `self + rhs < u64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > u128::MAX` or `self + rhs < u128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > isize::MAX` or `self + rhs < isize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_add": {
         "row": 42,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self + rhs > usize::MAX` or `self + rhs < usize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > i8::MAX` or `self - rhs < i8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > i16::MAX` or `self - rhs < i16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > i32::MAX` or `self - rhs < i32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > i64::MAX` or `self - rhs < i64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > i128::MAX` or `self - rhs < i128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > u8::MAX` or `self - rhs < u8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > u16::MAX` or `self - rhs < u16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > u32::MAX` or `self - rhs < u32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > u64::MAX` or `self - rhs < u64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > u128::MAX` or `self - rhs < u128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > isize::MAX` or `self - rhs < isize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_sub": {
         "row": 43,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self - rhs > usize::MAX` or `self - rhs < usize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > i8::MAX` or `self * rhs < i8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > i16::MAX` or `self * rhs < i16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > i32::MAX` or `self * rhs < i32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > i64::MAX` or `self * rhs < i64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > i128::MAX` or `self * rhs < i128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > u8::MAX` or `self * rhs < u8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > u16::MAX` or `self * rhs < u16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > u32::MAX` or `self * rhs < u32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > u64::MAX` or `self * rhs < u64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > u128::MAX` or `self * rhs < u128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > isize::MAX` or `self * rhs < isize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_mul": {
         "row": 44,
         "self": {
            "Bounded": [
               "This results in undefined behavior when `self * rhs > usize::MAX` or `self * rhs < usize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_shl": {
         "row": 45,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i8.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i16.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i32.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i64.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.i128.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u8.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u16.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u32.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u64.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.u128.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.isize.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.unchecked_shr": {
         "row": 46,
         "self": {
            "Bounded": [
               "This results in undefined behavior if `rhs` is larger than or equal to the number of bits in `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU8.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU16.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU32.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU64.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU128.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroUsize.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI8.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI16.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI32.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI64.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI128.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroIsize.html#method.new_unchecked": {
         "row": 47,
         "n": {
            "Bounded": [
               "The value must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU8.html#method.unchecked_add": {
         "row": 48,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self + rhs > u8::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU16.html#method.unchecked_add": {
         "row": 48,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self + rhs > u16::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU32.html#method.unchecked_add": {
         "row": 48,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self + rhs > u32::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU64.html#method.unchecked_add": {
         "row": 48,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self + rhs > u64::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU128.html#method.unchecked_add": {
         "row": 48,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self + rhs > u128::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroUsize.html#method.unchecked_add": {
         "row": 48,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self + rhs > usize::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU8.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > u8::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU16.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > u16::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU32.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > u32::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU64.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > u64::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroU128.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > u128::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroUsize.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > usize::MAX`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI8.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > i8::MAX`, or `self * rhs < i8::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI16.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > i16::MAX`, or `self * rhs < i16::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI32.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > i32::MAX`, or `self * rhs < i32::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI64.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > i64::MAX`, or `self * rhs < i64::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroI128.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > i128::MAX`, or `self * rhs < i128::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/num/struct.NonZeroIsize.html#method.unchecked_mul": {
         "row": 49,
         "self": {
            "Bounded": [
               "The behaviour is undefined as soon as `self * rhs > isize::MAX`, or `self * rhs < isize::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_ref": {
         "row": 50,
         "self": {
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Initialized": [
               "The pointer must point to an initialized instance of `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_uninit_ref": {
         "row": 51,
         "self": {
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.offset": {
         "row": 52,
         "self": {
            "Bounded": [
               "The computed offset, in bytes, cannot overflow an `isize`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_offset": {
         "row": 53,
         "self": {
            "Bounded": [
               "The computed offset, in bytes, cannot overflow an `isize`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.offset_from": {
         "row": 54,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_offset_from": {
         "row": 55,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.sub_ptr": {
         "row": 56,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`.",
               "The distance between the pointers must be non-negative (`self` >= `origin`)."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.add": {
         "row": 57,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_add": {
         "row": 58,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.sub": {
         "row": 59,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_sub": {
         "row": 60,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.read": {
         "row": 61,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.read_volatile": {
         "row": 62,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`.",
               "However, storing non-`Copy` types in volatile memory is almost certainly incorrect."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.read_unaligned": {
         "row": 63,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_to": {
         "row": 64,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "dest": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dest` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dest` can violate memory safety. Note that assigning to `*dest` counts as a use because it will attempt to drop the value at `*dest`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_to_nonoverlapping": {
         "row": 65,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dest` with the same size."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "dest": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dest` with the same size."
            ],
            "Layout": [
               "`dest` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dest` can violate memory safety. Note that assigning to `*dest` counts as a use because it will attempt to drop the value at `*dest`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.get_unchecked": {
         "row": 66,
         "self": {
            "Dereferencable": [
               "When `self` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         },
         "index": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_uninit_slice": {
         "row": 67,
         "self": {
            "Bounded": [
               "The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Dereferencable": [
               "The entire memory range of this slice (`ptr.len() * mem::size_of::<T>()` bytes) must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The pointer must be aligned even for zero-length slices."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_ref-1": {
         "row": 68,
         "self": {
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Initialized": [
               "The pointer must point to an initialized instance of `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_uninit_ref-1": {
         "row": 69,
         "self": {
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.offset-1": {
         "row": 70,
         "self": {
            "Bounded": [
               "The computed offset, in bytes, cannot overflow an `isize`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_offset-1": {
         "row": 71,
         "self": {
            "Bounded": [
               "The computed offset, in bytes, cannot overflow an `isize`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_mut": {
         "row": 72,
         "self": {
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Initialized": [
               "The pointer must point to an initialized instance of `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_uninit_mut": {
         "row": 73,
         "self": {
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.offset_from-1": {
         "row": 74,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_offset_from-1": {
         "row": 75,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.sub_ptr-1": {
         "row": 76,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`.",
               "The distance between the pointers must be non-negative (`self` >= `origin`)."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.add-1": {
         "row": 77,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_add-1": {
         "row": 78,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.sub-1": {
         "row": 79,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.byte_sub-1": {
         "row": 80,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.read-1": {
         "row": 81,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.read_volatile-1": {
         "row": 82,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`.",
               "However, storing non-`Copy` types in volatile memory is almost certainly incorrect."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.read_unaligned-1": {
         "row": 83,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_to-1": {
         "row": 84,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "dest": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dest` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dest` can violate memory safety. Note that assigning to `*dest` counts as a use because it will attempt to drop the value at `*dest`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_to_nonoverlapping-1": {
         "row": 85,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dest` with the same size."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "dest": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dest` with the same size."
            ],
            "Layout": [
               "`dest` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dest` can violate memory safety. Note that assigning to `*dest` counts as a use because it will attempt to drop the value at `*dest`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_from": {
         "row": 86,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               ""
            ]
         },
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.copy_from_nonoverlapping": {
         "row": 87,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `src` with the same size."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               ""
            ]
         },
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `src` with the same size."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.drop_in_place": {
         "row": 88,
         "self": {
            "Allocated": [
               "`self` must be nonnull, even if T has size 0.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "The value `self` points to must be valid for dropping, which may mean it must uphold additional invariants. These invariants depend on the type of the value being dropped."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned, even if `T` has size 0.",
               "Unaligned values cannot be dropped in place, they must be copied to an aligned location first using `ptr::read_unaligned`."
            ],
            "Freed": [
               "Executes the destructor (if any) of the pointed-to value."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.write": {
         "row": 89,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.write_bytes": {
         "row": 90,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Untyped": [
               "Additionally, note that changing `self` in this way can easily lead to undefined behavior (UB) later if the written bytes are not a valid representation of some `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.write_volatile": {
         "row": 91,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.write_unaligned": {
         "row": 92,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.replace": {
         "row": 93,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "src": {
            "Leaked": [
               "Neither value is dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.swap": {
         "row": 94,
         "self": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         },
         "selfx": {
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "with": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`with` must be properly aligned."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.split_at_mut": {
         "row": 95,
         "self": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be dereferenceable and span a single allocation that is at least `mid * size_of::<T>()` bytes long."
            ]
         },
         "mid": {
            "Bounded": [
               "Panics if `mid` > `len`.",
               "`mid` must be in-bounds of the underlying allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.split_at_mut_unchecked": {
         "row": 96,
         "self": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`self` must be dereferenceable and span a single allocation that is at least `mid * size_of::<T>()` bytes long."
            ]
         },
         "mid": {
            "Bounded": [
               "Panics if `mid` > `len`.",
               "`mid` must be in-bounds of the underlying allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.get_unchecked_mut": {
         "row": 97,
         "self": {
            "Dereferencable": [
               "When `self` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         },
         "index": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_uninit_slice-1": {
         "row": 98,
         "self": {
            "Bounded": [
               "The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Dereferencable": [
               "The entire memory range of this slice (`ptr.len() * mem::size_of::<T>()` bytes) must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The pointer must be aligned even for zero-length slices."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.pointer.html#method.as_uninit_slice_mut": {
         "row": 99,
         "self": {
            "Bounded": [
               "The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "Either the pointer is null or all of the following is true."
            ],
            "Dereferencable": [
               "The entire memory range of this slice (`ptr.len() * mem::size_of::<T>()` bytes) must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The pointer must be aligned even for zero-length slices."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.Alignment.html#method.new_unchecked": {
         "row": 100,
         "align": {
            "Bounded": [
               "`align` must be a power of two.",
               "Equivalently, it must be `1 << exp` for some exp in `0..usize::BITS`. It must not be zero."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.drop_in_place.html": {
         "row": 101,
         "to_drop": {
            "Allocated": [
               "`to_drop` must be nonnull, even if T has size 0.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "The value `to_drop` points to must be valid for dropping, which may mean it must uphold additional invariants. These invariants depend on the type of the value being dropped."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`to_drop` must be properly aligned, even if `T` has size 0.",
               "Unaligned values cannot be dropped in place, they must be copied to an aligned location first using `ptr::read_unaligned`."
            ],
            "Freed": [
               "Executes the destructor (if any) of the pointed-to value."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.swap.html": {
         "row": 102,
         "x": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`x` must be properly aligned."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         },
         "y": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`y` must be properly aligned."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.swap_nonoverlapping.html": {
         "row": 103,
         "x": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `x` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `y` with the same size."
            ],
            "Layout": [
               "`x` must be properly aligned."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         },
         "y": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `x` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `y` with the same size."
            ],
            "Layout": [
               "`y` must be properly aligned."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.replace.html": {
         "row": 104,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`dst` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ]
         },
         "src": {
            "Leaked": [
               "Neither value is dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.read.html": {
         "row": 105,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`src` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.read_unaligned.html": {
         "row": 106,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`src` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.write.html": {
         "row": 107,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.write_unaligned.html": {
         "row": 108,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.write_bytes.html": {
         "row": 109,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "Untyped": [
               "Additionally, note that changing `dst` in this way can easily lead to undefined behavior (UB) later if the written bytes are not a valid representation of some `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.copy.html": {
         "row": 110,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dst` can violate memory safety. Note that assigning to `*dst` counts as a use because it will attempt to drop the value at `*dst`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.copy_nonoverlapping.html": {
         "row": 111,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dst` with the same size."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dst` with the same size."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dst` can violate memory safety. Note that assigning to `*dst` counts as a use because it will attempt to drop the value at `*dst`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.read_volatile.html": {
         "row": 112,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`src` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`.",
               "However, storing non-`Copy` types in volatile memory is almost certainly incorrect."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/fn.write_volatile.html": {
         "row": 113,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_uninit_ref": {
         "row": 114,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_uninit_mut": {
         "row": 115,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.new_unchecked": {
         "row": 116,
         "ptr": {
            "Allocated": [
               "`ptr` must be non-null."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_ref": {
         "row": 117,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "The pointer must point to an initialized instance of `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_mut": {
         "row": 118,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "The pointer must point to an initialized instance of `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "The pointer must be properly aligned."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset": {
         "row": 119,
         "self": {
            "Bounded": [
               "The computed offset, in bytes, cannot overflow an `isize`."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.byte_offset": {
         "row": 120,
         "self": {
            "Bounded": [
               "The computed offset, in bytes, cannot overflow an `isize`."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.add": {
         "row": 121,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.byte_add": {
         "row": 122,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.sub": {
         "row": 123,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.byte_sub": {
         "row": 124,
         "self": {
            "Bounded": [
               "The computed offset cannot exceed `isize::MAX` bytes."
            ],
            "Dereferencable": [
               "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.offset_from": {
         "row": 125,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.byte_offset_from": {
         "row": 126,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "`origin` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.sub_ptr": {
         "row": 127,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`.",
               "The distance between the pointers must be non-negative (`self` >= `subtracted`)."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "`self` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "Layout": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "subtracted": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "`subtracted` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.read": {
         "row": 128,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.read_volatile": {
         "row": 129,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`.",
               "However, storing non-`Copy` types in volatile memory is almost certainly incorrect."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.read_unaligned": {
         "row": 130,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `self` can violate memory safety. Note that assigning to `self` counts as a use because it will attempt to drop the value at `self`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.copy_to": {
         "row": 131,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "dest": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dest` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dest` can violate memory safety. Note that assigning to `*dest` counts as a use because it will attempt to drop the value at `*dest`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.copy_to_nonoverlapping": {
         "row": 132,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dest` with the same size."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "dest": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dest` with the same size."
            ],
            "Layout": [
               "`dest` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dest` can violate memory safety. Note that assigning to `*dest` counts as a use because it will attempt to drop the value at `*dest`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.copy_from": {
         "row": 133,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               ""
            ]
         },
         "src": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.copy_from_nonoverlapping": {
         "row": 134,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `src` with the same size."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               ""
            ]
         },
         "src": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `self` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `src` with the same size."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*src` can violate memory safety. Note that assigning to `*src` counts as a use because it will attempt to drop the value at `*src`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.drop_in_place": {
         "row": 135,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "The value `self` points to must be valid for dropping, which may mean it must uphold additional invariants. These invariants depend on the type of the value being dropped."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned, even if `T` has size 0.",
               "Unaligned values cannot be dropped in place, they must be copied to an aligned location first using `ptr::read_unaligned`."
            ],
            "Freed": [
               "Executes the destructor (if any) of the pointed-to value."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.write": {
         "row": 136,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.write_bytes": {
         "row": 137,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Untyped": [
               "Additionally, note that changing `self` in this way can easily lead to undefined behavior (UB) later if the written bytes are not a valid representation of some `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.write_volatile": {
         "row": 138,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.write_unaligned": {
         "row": 139,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.replace": {
         "row": 140,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`self` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "src": {
            "Leaked": [
               "Neither value is dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.swap": {
         "row": 141,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         },
         "selfx": {
            "Layout": [
               "`self` must be properly aligned."
            ]
         },
         "with": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`with` must be properly aligned."
            ],
            "Untyped": [
               "The operation is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_uninit_slice": {
         "row": 142,
         "self": {
            "Bounded": [
               "The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The entire memory range of this slice (`ptr.len() * mem::size_of::<T>()` bytes) must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The pointer must be aligned even for zero-length slices."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.as_uninit_slice_mut": {
         "row": 143,
         "self": {
            "Bounded": [
               "The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The entire memory range of this slice (`ptr.len() * mem::size_of::<T>()` bytes) must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The pointer must be aligned even for zero-length slices."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get accessed (read or written) through any other pointer.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ptr/struct.NonNull.html#method.get_unchecked_mut": {
         "row": 144,
         "self": {
            "Allocated": [
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "When `self` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         },
         "index": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/slice/fn.from_raw_parts.html#": {
         "row": 145,
         "data": {
            "Allocated": [
               "`data` must be non-null even for zero-length slices."
            ],
            "Initialized": [
               "`data` must point to `len` consecutive properly initialized values of type `T`."
            ],
            "Dereferencable": [
               "The entire memory range (`len * mem::size_of::<T>()` bytes) of this slice must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "`data` must be aligned even for zero-length slices."
            ]
         },
         "len": {
            "Bounded": [
               "The total size `len * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ]
         },
         "retval": {
            "Aliased": [
               "The memory referenced by the returned slice must not be mutated for the duration of lifetime `'a`.",
               "The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the context."
            ]
         }
      },
      "https://doc.rust-lang.org/core/slice/fn.from_raw_parts_mut.html#": {
         "row": 146,
         "data": {
            "Allocated": [
               "`data` must be non-null even for zero-length slices."
            ],
            "Initialized": [
               "`data` must point to `len` consecutive properly initialized values of type `T`."
            ],
            "Dereferencable": [
               "The entire memory range (`len * mem::size_of::<T>()` bytes) of this slice must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "`data` must be aligned even for zero-length slices."
            ]
         },
         "len": {
            "Bounded": [
               "The total size `len * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`."
            ]
         },
         "retval": {
            "Aliased": [
               "The memory referenced by the returned slice must not be mutated for the duration of lifetime `'a`.",
               "The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the context."
            ]
         }
      },
      "https://doc.rust-lang.org/core/slice/fn.from_ptr_range.html#": {
         "row": 147,
         "range": {
            "Bounded": [
               "The total length of the `range` must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "The `start` pointer of the range must be non-null even for zero-length slices.",
               "The `end` pointer of the range must be non-null even for zero-length slices."
            ],
            "Initialized": [
               "The range must contain `N` consecutive properly initialized values of type `T`."
            ],
            "Dereferencable": [
               "The entire memory range of this slice must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The `start` pointer of the range must be properly aligned to the first element of a slice.",
               "The `end` pointer of the range must be to one past the last element.",
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "retval": {
            "Aliased": [
               "The memory referenced by the returned slice must not be mutated for the duration of lifetime `'a`.",
               "The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the context."
            ]
         }
      },
      "https://doc.rust-lang.org/core/slice/fn.from_mut_ptr_range.html#": {
         "row": 148,
         "range": {
            "Bounded": [
               "The total length of the `range` must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "The `start` pointer of the range must be non-null even for zero-length slices.",
               "The `end` pointer of the range must be non-null even for zero-length slices."
            ],
            "Initialized": [
               "The range must contain `N` consecutive properly initialized values of type `T`."
            ],
            "Dereferencable": [
               "The entire memory range of this slice must be contained within a single allocated object! Slices can never span across multiple allocated objects."
            ],
            "Layout": [
               "The `start` pointer of the range must be properly aligned to the first element of a slice.",
               "The `end` pointer of the range must be to one past the last element.",
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "retval": {
            "Aliased": [
               "The memory referenced by the returned slice must not be mutated for the duration of lifetime `'a`.",
               "The lifetime for the returned slice is inferred from its usage. To prevent accidental misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the context."
            ]
         }
      },
      "https://doc.rust-lang.org/core/slice/trait.SliceIndex.html#tymethod.get_unchecked": {
         "row": 149,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/slice/trait.SliceIndex.html#tymethod.get_unchecked_mut": {
         "row": 150,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.get_unchecked": {
         "row": 151,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.usize.html#method.get_unchecked_mut": {
         "row": 152,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.Range.html#method.get_unchecked-1": {
         "row": 153,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.Range.html#method.get_unchecked_mut-1": {
         "row": 154,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeTo.html#method.get_unchecked": {
         "row": 155,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeTo.html#method.get_unchecked_mut": {
         "row": 156,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFrom.html#method.get_unchecked": {
         "row": 157,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFrom.html#method.get_unchecked_mut": {
         "row": 158,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFull.html#method.get_unchecked-1": {
         "row": 159,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFull.html#method.get_unchecked_mut-1": {
         "row": 160,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html#method.get_unchecked-1": {
         "row": 161,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html#method.get_unchecked_mut-1": {
         "row": 162,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html#method.get_unchecked-1": {
         "row": 163,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html#method.get_unchecked_mut-1": {
         "row": 164,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.tuple.html#method.get_unchecked": {
         "row": 165,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.tuple.html#method.get_unchecked_mut": {
         "row": 166,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.get_unchecked": {
         "row": 167,
         "index": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.get_many_unchecked_mut": {
         "row": 168,
         "index": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.swap_unchecked": {
         "row": 169,
         "a": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior. The caller has to ensure that `a < self.len()`."
            ]
         },
         "b": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior. The caller has to ensure that `b < self.len()`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.as_chunks_unchecked": {
         "row": 170,
         "self": {
            "Bounded": [
               "The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).",
               "`N != 0`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.as_chunks_unchecked_mut": {
         "row": 171,
         "self": {
            "Bounded": [
               "The slice splits exactly into `N`-element chunks (aka `self.len() % N == 0`).",
               "`N != 0`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.split_at_unchecked": {
         "row": 172,
         "mid": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.split_at_mut_unchecked": {
         "row": 173,
         "mid": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used. The caller has to ensure that `0 <= mid <= self.len()`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.align_to": {
         "row": 174,
         "self": {
            "Initialized": [
               "Both the `T` and the `U` must be valid at their given type.",
               "To transmute the inner type of the contents of a container, you must make sure to not violate any of the container's invariants."
            ],
            "Layout": [
               "Both types must have the same size.",
               "Note that source and destination are passed by-value, which means if `T` or `U` contain padding, that padding is not guaranteed to be preserved by transmute.",
               "This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.",
               "When transmuting values that point elsewhere (such as pointers, references, boxes…), the caller has to ensure proper alignment of the pointed-to values."
            ]
         },
         "retval": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type."
            ],
            "Aliased": [
               "It can turn a `*mut T` into an `&mut T`.",
               "It can extend a lifetime, or shorten an invariant lifetime."
            ],
            "Untyped": [
               "It is therefore your responsibility to guarantee that every value passed to transmute is valid at both types `T` and `U`. Failing to uphold this condition may lead to unexpected and unstable compilation results."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.slice.html#method.align_to_mut": {
         "row": 175,
         "self": {
            "Initialized": [
               "Both the `T` and the `U` must be valid at their given type.",
               "To transmute the inner type of the contents of a container, you must make sure to not violate any of the container's invariants."
            ],
            "Layout": [
               "Both types must have the same size.",
               "Note that source and destination are passed by-value, which means if `T` or `U` contain padding, that padding is not guaranteed to be preserved by transmute.",
               "This method has no purpose when either input element `T` or output element `U` are zero-sized and will return the original slice without splitting anything.",
               "When transmuting values that point elsewhere (such as pointers, references, boxes…), the caller has to ensure proper alignment of the pointed-to values."
            ]
         },
         "retval": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type."
            ],
            "Aliased": [
               "It can turn a `*mut T` into an `&mut T`.",
               "It can extend a lifetime, or shorten an invariant lifetime."
            ],
            "Untyped": [
               "It is therefore your responsibility to guarantee that every value passed to transmute is valid at both types `T` and `U`. Failing to uphold this condition may lead to unexpected and unstable compilation results."
            ]
         }
      },
      "https://doc.rust-lang.org/core/str/fn.from_utf8_unchecked.html": {
         "row": 176,
         "v": {
            "Initialized": [
               "The bytes passed in must be valid UTF-8."
            ]
         }
      },
      "https://doc.rust-lang.org/core/str/fn.from_utf8_unchecked_mut.html": {
         "row": 177,
         "v": {
            "Initialized": [
               "The bytes passed in must be valid UTF-8."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.str.html#method.as_bytes_mut": {
         "row": 178,
         "self": {
            "Initialized": [
               "The caller must ensure that the content of the slice is valid UTF-8 before the borrow ends and the underlying `str` is used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.str.html#method.get_unchecked": {
         "row": 179,
         "i": {
            "Bounded": [
               "The starting index must not exceed the ending index.",
               "Indexes must be within bounds of the original slice."
            ],
            "Initialized": [
               "Indexes must lie on UTF-8 sequence boundaries."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.str.html#method.get_unchecked_mut": {
         "row": 180,
         "i": {
            "Bounded": [
               "The starting index must not exceed the ending index.",
               "Indexes must be within bounds of the original slice."
            ],
            "Initialized": [
               "Indexes must lie on UTF-8 sequence boundaries."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.str.html#method.slice_unchecked": {
         "row": 181,
         "begin": {
            "Bounded": [
               "`begin` must not exceed `end`.",
               "`begin` must be byte positions within the string slice."
            ],
            "Initialized": [
               "`begin` must lie on UTF-8 sequence boundaries."
            ]
         },
         "end": {
            "Bounded": [
               "`end` must be byte positions within the string slice."
            ],
            "Initialized": [
               "`end` must lie on UTF-8 sequence boundaries."
            ]
         }
      },
      "https://doc.rust-lang.org/core/primitive.str.html#method.slice_mut_unchecked": {
         "row": 182,
         "begin": {
            "Bounded": [
               "`begin` must not exceed `end`.",
               "`begin` must be byte positions within the string slice."
            ],
            "Initialized": [
               "`begin` must lie on UTF-8 sequence boundaries."
            ]
         },
         "end": {
            "Bounded": [
               "`end` must be byte positions within the string slice."
            ],
            "Initialized": [
               "`end` must lie on UTF-8 sequence boundaries."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFull.html#method.get_unchecked": {
         "row": 183,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFull.html#method.get_unchecked_mut": {
         "row": 184,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.Range.html#method.get_unchecked": {
         "row": 185,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.Range.html#method.get_unchecked_mut": {
         "row": 186,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeTo.html#method.get_unchecked-1": {
         "row": 187,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeTo.html#method.get_unchecked_mut-1": {
         "row": 188,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFrom.html#method.get_unchecked-1": {
         "row": 189,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeFrom.html#method.get_unchecked_mut-1": {
         "row": 190,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html#method.get_unchecked": {
         "row": 191,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeInclusive.html#method.get_unchecked_mut": {
         "row": 192,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html#method.get_unchecked": {
         "row": 193,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/ops/struct.RangeToInclusive.html#method.get_unchecked_mut": {
         "row": 194,
         "self": {
            "Bounded": [
               "Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used."
            ]
         },
         "slice": {
            "Allocated": [
               "A dangling slice pointer is undefined behavior even if the resulting reference is not used."
            ],
            "Dereferencable": [
               "When `slice` is not dereferenceable is undefined behavior even if the resulting pointer is not used."
            ]
         }
      },
      "https://doc.rust-lang.org/core/task/struct.Waker.html#method.from_raw": {
         "row": 195,
         "waker": {
            "Thread": [
               "These functions must all be thread-safe (even though `RawWaker` is `!Send + !Sync`) because `Waker` is `Send + Sync`, and thus wakers may be moved to arbitrary threads or invoked by `&` reference."
            ]
         }
      },
      "https://doc.rust-lang.org/core/hint/fn.unreachable_unchecked.html": {
         "row": 196,
         "NULL": {
            "Unreachable": [
               "Reaching this function is Undefined Behavior.",
               "It will safely panic in case it is actually reached at runtime."
            ]
         }
      },
      "https://doc.rust-lang.org/core/any/trait.Any.html#method.downcast_ref_unchecked": {
         "row": 197,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/any/trait.Any.html#method.downcast_mut_unchecked": {
         "row": 198,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/any/trait.Any.html#method.downcast_ref_unchecked-1": {
         "row": 199,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/any/trait.Any.html#method.downcast_mut_unchecked-1": {
         "row": 200,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/any/trait.Any.html#method.downcast_ref_unchecked-2": {
         "row": 201,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/any/trait.Any.html#method.downcast_mut_unchecked-2": {
         "row": 202,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/cell/struct.RefCell.html#method.try_borrow_unguarded": {
         "row": 203,
         "retval": {
            "Aliased": [
               "Mutably borrowing the `RefCell` while the reference returned by this method is alive is undefined behaviour."
            ]
         }
      },
      "https://doc.rust-lang.org/core/option/enum.Option.html#method.unwrap_unchecked": {
         "row": 204,
         "self": {
            "Unreachable": [
               "Calling this method on `None` is undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/core/pin/struct.Pin.html#method.new_unchecked": {
         "row": 205,
         "retval": {
            "Pinned": [
               "If the constructed `Pin<P>` does not guarantee that the data `P` points to is pinned, that is a violation of the API contract and may lead to undefined behavior in later (safe) operations.",
               "`P::Deref` and `P::DerefMut` implementations must not move out of their self arguments: `Pin::as_mut` and `Pin::as_ref` will call `DerefMut::deref_mut` and `Deref::deref` on the pinned pointer and expect these methods to uphold the pinning invariants.",
               "The reference `P` dereferences to will not be moved out of again; in particular, it must not be possible to obtain a `&mut P::Target` and then move out of that reference."
            ]
         }
      },
      "https://doc.rust-lang.org/core/pin/struct.Pin.html#method.into_inner_unchecked": {
         "row": 206,
         "retval": {
            "Pinned": [
               "You will continue to treat the pointer `P` as pinned after you call this function, so that the invariants on the Pin type can be upheld. If the code using the resulting `P` does not continue to maintain the pinning invariants that is a violation of the API contract and may lead to undefined behavior in later (safe) operations."
            ]
         }
      },
      "https://doc.rust-lang.org/core/pin/struct.Pin.html#method.map_unchecked": {
         "row": 207,
         "retval": {
            "Pinned": [
               "The data you return will not move so long as the argument value does not move (for example, because it is one of the fields of that value), and also that you do not move out of the argument you receive to the interior function."
            ]
         }
      },
      "https://doc.rust-lang.org/core/pin/struct.Pin.html#method.get_unchecked_mut": {
         "row": 208,
         "retval": {
            "Pinned": [
               "You will never move the data out of the mutable reference you receive when you call this function, so that the invariants on the `Pin` type can be upheld."
            ]
         }
      },
      "https://doc.rust-lang.org/core/pin/struct.Pin.html#method.map_unchecked_mut": {
         "row": 209,
         "retval": {
            "Pinned": [
               "The data you return will not move so long as the argument value does not move (for example, because it is one of the fields of that value), and also that you do not move out of the argument you receive to the interior function."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicBool>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicU8.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicU8>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicU16.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicU16>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicU32.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicU32>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicU64.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicU64>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicUsize>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicI8.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicI8>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicI16.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicI16>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicI32.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicI32>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicI64.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicI64>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicIsize.html#method.from_ptr": {
         "row": 210,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicIsize>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicPtr.html#method.from_ptr": {
         "row": 211,
         "ptr": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`ptr` must be aligned to `align_of::<AtomicPtr<T>>()`."
            ],
            "Thread": [
               "You must adhere to the Memory model for atomic accesses. In particular, it is not allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes, without synchronization."
            ]
         },
         "retval": {
            "Aliased": [
               "You must enforce Rust's aliasing rules. In particular, while this reference exists, the memory the pointer points to must not get mutated.",
               "The returned lifetime `'a` is arbitrarily chosen and does not necessarily reflect the actual lifetime of the data."
            ]
         }
      },
      "https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_unchecked": {
         "row": 212,
         "self": {
            "Unreachable": [
               "Calling this method on an `Err` is undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/core/result/enum.Result.html#method.unwrap_err_unchecked": {
         "row": 213,
         "self": {
            "Unreachable": [
               "Calling this method on an `Ok` is undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/core/io/struct.BorrowedCursor.html#method.as_mut": {
         "row": 214,
         "self": {
            "Initialized": [
               "The caller must not uninitialize any bytes in the initialized portion of the cursor."
            ]
         }
      },
      "https://doc.rust-lang.org/core/io/struct.BorrowedCursor.html#method.advance": {
         "row": 215,
         "self": {
            "Initialized": [
               "The caller must ensure that the first `n` bytes of the cursor have been properly initialised."
            ]
         }
      },
      "https://doc.rust-lang.org/core/io/struct.BorrowedCursor.html#method.set_init": {
         "row": 216,
         "self": {
            "Initialized": [
               "The caller must ensure that the first `n` bytes of the buffer have already been initialized."
            ]
         }
      },
      "https://doc.rust-lang.org/core/io/struct.BorrowedBuf.html#method.set_init": {
         "row": 217,
         "self": {
            "Initialized": [
               "The caller must ensure that the first `n` unfilled bytes of the buffer have already been initialized."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/ffi/struct.CString.html#method.from_vec_unchecked": {
         "row": 218,
         "v": {
            "Initialized": [
               "This method is equivalent to `CString::new` except that no runtime assertion is made that `v` contains no 0 bytes, and it requires an actual byte vector, not anything that can be converted to one with `Into`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/ffi/struct.CString.html#method.from_raw": {
         "row": 219,
         "ptr": {
            "Allocated": [
               "The raw pointer must point to a block of memory allocated by the global allocator.",
               "Other usage (trying to take ownership of a string that was allocated by foreign code) is likely to lead to undefined behavior or allocator corruption."
            ],
            "Initialized": [
               "Owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
               "This should only ever be called with a pointer that was earlier obtained by calling `CString::into_raw`.",
               "The recomputed length must match the original length from the `CString::into_raw` call. This means the `CString::into_raw/from_raw` methods should not be used when passing the string to C functions that can modify the string's length."
            ]
         },
         "retval": {
            "DualOwned": [
               "Retakes ownership of a `CString` that was transferred to C via `CString::into_raw`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/ffi/struct.CString.html#method.from_vec_with_nul_unchecked": {
         "row": 220,
         "v": {
            "Initialized": [
               "The given `Vec` must have one nul byte as its last element. This means it cannot be empty nor have any other nul byte anywhere else."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.from_raw_parts": {
         "row": 221,
         "ptr": {
            "Bounded": [
               "The allocated size in bytes must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "`ptr` must have been allocated using the global allocator, such as via the `alloc::alloc` function."
            ],
            "Initialized": [
               "The first `length` values must be properly initialized values of type `T`."
            ],
            "Layout": [
               "`T` needs to have the same alignment as what `ptr` was allocated with."
            ]
         },
         "capacity": {
            "Bounded": [
               "The size of `T` times the `capacity` (ie. the allocated size in bytes) needs to be the same size as the pointer was allocated with.",
               "`capacity` needs to be the capacity that the pointer was allocated with.",
               "`length` needs to be less than or equal to `capacity`."
            ]
         },
         "retval": {
            "DualOwned": [
               "The ownership of `ptr` is effectively transferred to the `Vec<T>` which may then deallocate, reallocate or change the contents of memory pointed to by the pointer at will. Ensure that nothing else uses the pointer after calling this function."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.from_raw_parts_in": {
         "row": 222,
         "ptr": {
            "Bounded": [
               "The allocated size in bytes must be no larger than `isize::MAX`."
            ],
            "Allocated": [
               "`ptr` must be currently allocated via the given allocator `alloc`."
            ],
            "Initialized": [
               "The first `length` values must be properly initialized values of type `T`."
            ],
            "Layout": [
               "`T` needs to have the same alignment as what `ptr` was allocated with."
            ]
         },
         "capacity": {
            "Bounded": [
               "The size of `T` times the `capacity` (ie. the allocated size in bytes) needs to be the same size as the pointer was allocated with.",
               "`capacity` needs to be the capacity that the pointer was allocated with.",
               "`length` needs to be less than or equal to `capacity`."
            ]
         },
         "retval": {
            "DualOwned": [
               "The ownership of `ptr` is effectively transferred to the `Vec<T>` which may then deallocate, reallocate or change the contents of memory pointed to by the pointer at will. Ensure that nothing else uses the pointer after calling this function."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.set_len": {
         "row": 223,
         "new_len": {
            "Bounded": [
               "`new_len` must be less than or equal to `capacity()`."
            ]
         },
         "self": {
            "Initialized": [
               "The elements at `old_len..new_len` must be initialized."
            ],
            "Leaked": [
               "There maybe a memory leak since the inner vectors were not freed prior to the `set_len` call."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.assume_init": {
         "row": 224,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.assume_init-1": {
         "row": 225,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.from_raw": {
         "row": 226,
         "self": {
            "Allocated": [
               "For non-zero-sized values, a `Box` will use the Global allocator for its allocation.",
               "For zero-sized values, the `Box` pointer still has to be valid for reads and writes (always be non-null pointers)."
            ],
            "Initialized": [
               "Recreate a `Box` which was previously converted to a raw pointer using `Box::into_raw`.",
               "It is valid to convert both ways between a `Box` and a raw pointer allocated with the Global allocator, given that the `Layout` used with the allocator is correct for the type."
            ],
            "Layout": [
               "The `Box` pointer has to be sufficiently aligned (always be fully aligned)."
            ]
         },
         "retval": {
            "DualOwned": [
               "After calling this function, the raw pointer is owned by the resulting `Box`. A double-free may occur if the function is called twice on the same raw pointer."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.from_raw_in": {
         "row": 227,
         "self": {
            "Allocated": [
               "For non-zero-sized values, a `Box` will use the in the given allocator for its allocation.",
               "For zero-sized values, the `Box` pointer still has to be valid for reads and writes (always be non-null pointers)."
            ],
            "Initialized": [
               "Recreate a `Box` which was previously converted to a raw pointer using `Box::into_raw_with_allocator.",
               "It is valid to convert both ways between a `Box` and a raw pointer allocated with the given allocator, given that the `Layout` used with the allocator is correct for the type."
            ],
            "Layout": [
               "The `Box` pointer has to be sufficiently aligned (always be fully aligned)."
            ]
         },
         "retval": {
            "DualOwned": [
               "After calling this function, the raw pointer is owned by the resulting `Box`. A double-free may occur if the function is called twice on the same raw pointer."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.downcast_unchecked": {
         "row": 228,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.downcast_unchecked-1": {
         "row": 229,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/boxed/struct.Box.html#method.downcast_unchecked-2": {
         "row": 230,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.assume_init": {
         "row": 231,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the inner value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.assume_init-1": {
         "row": 232,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the inner value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.from_raw": {
         "row": 233,
         "ptr": {
            "Allocated": [
               "The raw pointer must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The raw pointer must have been previously returned by a call to `Rc<U>::into_raw`.",
               "Note that if `U` is not `T` but has the same size and alignment, this is basically like transmuting references of different types."
            ],
            "Layout": [
               "`Rc<U>::into_raw` where `U` must have the same size and alignment as `T`."
            ]
         },
         "retval": {
            "DualOwned": [
               "The user of `from_raw` has to make sure a specific value of `T` is only dropped once."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.from_raw_in": {
         "row": 234,
         "ptr": {
            "Allocated": [
               "The raw pointer must point to a block of memory allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The raw pointer must have been previously returned by a call to `Rc<U>::into_raw`.",
               "Note that if `U` is not `T` but has the same size and alignment, this is basically like transmuting references of different types."
            ],
            "Layout": [
               "`Rc<U>::into_raw` where `U` must have the same size and alignment as `T`."
            ]
         },
         "retval": {
            "DualOwned": [
               "The user of `from_raw` has to make sure a specific value of `T` is only dropped once."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.increment_strong_count": {
         "row": 235,
         "ptr": {
            "Bounded": [
               "The associated `Rc` instance must be valid (i.e. the strong count must be at least 1) for the duration of this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Rc::into_raw`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.increment_strong_count_in": {
         "row": 236,
         "ptr": {
            "Bounded": [
               "The associated `Rc` instance must be valid (i.e. the strong count must be at least 1) for the duration of this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Rc::into_raw`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.decrement_strong_count": {
         "row": 237,
         "ptr": {
            "Bounded": [
               "The associated `Rc` instance must be valid (i.e. the strong count must be at least 1) when invoking this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Rc::into_raw`."
            ],
            "Freed": [
               "This method can be used to release the final `Rc` and backing storage, but should not be called after the final `Rc` has been released."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.decrement_strong_count_in": {
         "row": 238,
         "ptr": {
            "Bounded": [
               "The associated `Rc` instance must be valid (i.e. the strong count must be at least 1) when invoking this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Rc::into_raw`."
            ],
            "Freed": [
               "This method can be used to release the final `Rc` and backing storage, but should not be called after the final `Rc` has been released."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.get_mut_unchecked": {
         "row": 239,
         "retval": {
            "Aliased": [
               "If any other `Rc` or `Weak` pointers to the same allocation exist, then they must not be dereferenced or have active borrows for the duration of the returned borrow, and their inner type must be exactly the same as the inner type of this `Rc` (including lifetimes)."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Rc.html#method.downcast_unchecked": {
         "row": 240,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Weak.html#method.from_raw": {
         "row": 241,
         "ptr": {
            "Allocated": [
               "`ptr` must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The pointer must have originated from the `Weak::into_raw` and must still own its potential weak reference."
            ]
         },
         "retval": {
            "DualOwned": [
               "It takes ownership of one weak reference. This can be used to deallocate the weak count by dropping the `Weak<T>`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/rc/struct.Weak.html#method.from_raw_in": {
         "row": 242,
         "ptr": {
            "Allocated": [
               "`ptr` must point to a block of memory allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The pointer must have originated from the `Weak::into_raw` and must still own its potential weak reference."
            ]
         },
         "retval": {
            "DualOwned": [
               "It takes ownership of one weak reference. This can be used to deallocate the weak count by dropping the `Weak<T>`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/str/fn.from_boxed_utf8_unchecked.html": {
         "row": 243,
         "v": {
            "Initialized": [
               "Converts a boxed slice of bytes to a boxed string slice without checking that the string contains valid UTF-8."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/string/struct.String.html#method.from_raw_parts": {
         "row": 244,
         "buf": {
            "Allocated": [
               "The memory at `buf` needs to have been previously allocated by the same allocator the standard library uses."
            ],
            "Initialized": [
               "The first length bytes at `buf` need to be valid UTF-8."
            ],
            "Layout": [
               "The memory at buf is required alignment of exactly 1."
            ]
         },
         "length": {
            "Bounded": [
               "`length` needs to be less than or equal to `capacity`."
            ]
         },
         "capacity": {
            "Bounded": [
               "`capacity` needs to be the correct value."
            ]
         },
         "retval": {
            "DualOwned": [
               "The ownership of `buf` is effectively transferred to the `String` which may then deallocate, reallocate or change the contents of memory pointed to by the pointer at will. Ensure that nothing else uses the pointer after calling this function."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/string/struct.String.html#method.from_utf8_unchecked": {
         "row": 245,
         "bytes": {
            "Initialized": [
               "It does not check that the bytes passed to it are valid UTF-8."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/string/struct.String.html#method.as_mut_vec": {
         "row": 246,
         "self": {
            "Untyped": [
               "The returned &mut Vec allows writing bytes which are not valid UTF-8."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.assume_init": {
         "row": 247,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the inner value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.assume_init-1": {
         "row": 248,
         "self": {
            "Initialized": [
               "It is up to the caller to guarantee that the inner value really is in an initialized state. Calling this when the content is not yet fully initialized causes immediate undefined behavior."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.from_raw": {
         "row": 249,
         "ptr": {
            "Allocated": [
               "The raw pointer must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The raw pointer must have been previously returned by a call to `Arc<U>::into_raw`.",
               "Note that if `U` is not `T` but has the same size and alignment, this is basically like transmuting references of different types."
            ],
            "Layout": [
               "`Arc<U>::into_raw` where `U` must have the same size and alignment as `T`."
            ]
         },
         "retval": {
            "DualOwned": [
               "The user of `from_raw` has to make sure a specific value of `T` is only dropped once."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.from_raw_in": {
         "row": 250,
         "ptr": {
            "Allocated": [
               "The raw pointer must point to a block of memory allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The raw pointer must have been previously returned by a call to `Arc<U>::into_raw`.",
               "Note that if `U` is not `T` but has the same size and alignment, this is basically like transmuting references of different types."
            ],
            "Layout": [
               "`Arc<U>::into_raw` where `U` must have the same size and alignment as `T`."
            ]
         },
         "retval": {
            "DualOwned": [
               "The user of `from_raw` has to make sure a specific value of `T` is only dropped once."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.increment_strong_count": {
         "row": 251,
         "ptr": {
            "Bounded": [
               "The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) for the duration of this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Arc::into_raw`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.increment_strong_count_in": {
         "row": 252,
         "ptr": {
            "Bounded": [
               "The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) for the duration of this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Arc::into_raw`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.decrement_strong_count": {
         "row": 253,
         "ptr": {
            "Bounded": [
               "The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) when invoking this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Arc::into_raw`."
            ],
            "Freed": [
               "This method can be used to release the final `Arc` and backing storage, but should not be called after the final `Arc` has been released."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.decrement_strong_count_in": {
         "row": 254,
         "ptr": {
            "Bounded": [
               "The associated `Arc` instance must be valid (i.e. the strong count must be at least 1) when invoking this method."
            ],
            "Allocated": [
               "`ptr` must point to a block of memory allocated by allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The pointer must have been obtained through `Arc::into_raw`."
            ],
            "Freed": [
               "This method can be used to release the final `Arc` and backing storage, but should not be called after the final `Arc` has been released."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.get_mut_unchecked": {
         "row": 255,
         "retval": {
            "Aliased": [
               "If any other `Arc` or `Weak` pointers to the same allocation exist, then they must not be dereferenced or have active borrows for the duration of the returned borrow, and their inner type must be exactly the same as the inner type of this `Arc` (including lifetimes)."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Arc.html#method.downcast_unchecked": {
         "row": 256,
         "self": {
            "Initialized": [
               "The contained value must be of type `T`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Weak.html#method.from_raw": {
         "row": 257,
         "ptr": {
            "Allocated": [
               "`ptr` must point to a block of memory allocated by the global allocator."
            ],
            "Initialized": [
               "The pointer must have originated from the `Weak::into_raw` and must still own its potential weak reference."
            ]
         },
         "retval": {
            "DualOwned": [
               "It takes ownership of one weak reference. This can be used to deallocate the weak count by dropping the `Weak<T>`."
            ]
         }
      },
      "https://doc.rust-lang.org/alloc/sync/struct.Weak.html#method.from_raw_in": {
         "row": 258,
         "ptr": {
            "Allocated": [
               "`ptr` must point to a block of memory allocated by `alloc` in the provided allocator."
            ],
            "Initialized": [
               "The pointer must have originated from the `Weak::into_raw` and must still own its potential weak reference."
            ]
         },
         "retval": {
            "DualOwned": [
               "It takes ownership of one weak reference. This can be used to deallocate the weak count by dropping the `Weak<T>`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.get_many_unchecked_mut": {
         "row": 259,
         "retval": {
            "Aliased": [
               "Calling this method with overlapping keys is undefined behavior even if the resulting references are not used."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd": {
         "row": 260,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.from_raw_fd": {
         "row": 261,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.from_raw_fd": {
         "row": 261,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_fd": {
         "row": 261,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/fd/struct.BorrowedFd.html#method.borrow_raw": {
         "row": 262,
         "fd": {
            "Bounded": [
               "The resource pointed to by `fd` must not have the value `-1`."
            ],
            "SystemIO": [
               "The resource pointed to by `fd` must remain open for the duration of the returned `BorrowedFd`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/fd/struct.OwnedFd.html#method.from_raw_fd": {
         "row": 263,
         "fd": {
            "SystemIO": [
               "The resource pointed to by `fd` must be open and suitable for assuming ownership. The resource must not require any cleanup other than `close`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/fd/type.RawFd.html#method.from_raw_fd": {
         "row": 264,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/fs/struct.File.html#method.from_raw_fd": {
         "row": 265,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/linux/process/struct.PidFd.html#method.from_raw_fd": {
         "row": 266,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.from_raw_fd": {
         "row": 267,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html#method.from_raw_fd": {
         "row": 268,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.from_raw_fd": {
         "row": 269,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/process/struct.Stdio.html#method.from_raw_fd": {
         "row": 270,
         "fd": {
            "SystemIO": [
               "The `fd` passed in must be an owned file descriptor; in particular, it must be open."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedHandle.html#method.borrow_raw": {
         "row": 271,
         "handle": {
            "SystemIO": [
               "The resource pointed to by handle must be a valid open handle, it must remain open for the duration of the returned `BorrowedHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html#method.from_raw_handle": {
         "row": 272,
         "handle": {
            "SystemIO": [
               "`handle` must be an owned handle; in particular, it must be open.",
               "`handle` must be a handle for a resource that may be freed via `CloseHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrNull.html#method.from_raw_handle": {
         "row": 273,
         "handle": {
            "SystemIO": [
               "The passed handle value must either satisfy the safety requirements of `FromRawHandle::from_raw_handle` (below), or be null.",
               "`handle` must be an owned handle; in particular, it must be open.",
               "`handle` must be a handle for a resource that may be freed via `CloseHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrInvalid.html#method.from_raw_handle": {
         "row": 274,
         "handle": {
            "SystemIO": [
               "The passed handle value must either satisfy the safety requirements of `FromRawHandle::from_raw_handle` (below), or be `INVALID_HANDLE_VALUE` (-1).",
               "`handle` must be an owned handle; in particular, it must be open.",
               "`handle` must be a handle for a resource that may be freed via `CloseHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/trait.FromRawHandle.html#tymethod.from_raw_handle": {
         "row": 275,
         "handle": {
            "SystemIO": [
               "`handle` must be an owned handle; in particular, it must be open.",
               "`handle` must be a handle for a resource that may be freed via `CloseHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/fs/struct.File.html#method.from_raw_handle": {
         "row": 276,
         "handle": {
            "SystemIO": [
               "`handle` must be an owned handle; in particular, it must be open.",
               "`handle` must be a handle for a resource that may be freed via `CloseHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/trait.FromRawSocket.html#tymethod.from_raw_socket": {
         "row": 277,
         "sock": {
            "SystemIO": [
               "`sock` must be an owned socket; in particular, it must be open.",
               "`sock` must be a socket that may be freed via `closesocket`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.from_raw_socket": {
         "row": 278,
         "sock": {
            "SystemIO": [
               "`sock` must be an owned socket; in particular, it must be open.",
               "`sock` must be a socket that may be freed via `closesocket`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.from_raw_socket": {
         "row": 279,
         "sock": {
            "SystemIO": [
               "`sock` must be an owned socket; in particular, it must be open.",
               "`sock` must be a socket that may be freed via `closesocket`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.from_raw_socket": {
         "row": 280,
         "sock": {
            "SystemIO": [
               "`sock` must be an owned socket; in particular, it must be open.",
               "`sock` must be a socket that may be freed via `closesocket`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedSocket.html#method.borrow_raw": {
         "row": 281,
         "socket": {
            "Bounded": [
               "The resource pointed to by raw must not have the value `INVALID_SOCKET`."
            ],
            "SystemIO": [
               "The resource pointed to by `raw` must remain open for the duration of the returned `BorrowedSocket`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedSocket.html#method.from_raw_socket": {
         "row": 282,
         "socket": {
            "SystemIO": [
               "`socket` must be an owned socket; in particular, it must be open.",
               "`socket` must be a socket that may be freed via `closesocket`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/process/struct.Stdio.html#method.from_raw_handle": {
         "row": 283,
         "handle": {
            "SystemIO": [
               "`handle` must be an owned handle; in particular, it must be open.",
               "`handle` must be a handle for a resource that may be freed via `CloseHandle`."
            ]
         }
      },
      "https://doc.rust-lang.org/std/alloc/struct.System.html#method.alloc": {
         "row": 284,
         "layout": {
            "Layout": [
               "`layout` has non-zero size."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory may or may not be initialized."
            ],
            "Freed": [
               "Returning a null pointer indicates that either memory is exhausted."
            ]
         }
      },
      "https://doc.rust-lang.org/std/alloc/struct.System.html#method.alloc_zeroed": {
         "row": 285,
         "layout": {
            "Layout": [
               "`layout` has non-zero size."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory is guaranteed to be initialized but may be untyped."
            ],
            "Freed": [
               "Returning a null pointer indicates that either memory is exhausted."
            ]
         }
      },
      "https://doc.rust-lang.org/std/alloc/struct.System.html#method.dealloc": {
         "row": 286,
         "ptr": {
            "Allocated": [
               "`ptr` must denote a block of memory currently allocated via this allocator."
            ]
         },
         "layout": {
            "Layout": [
               "`layout` must be the same layout that was used to allocate that block of memory."
            ]
         }
      },
      "https://doc.rust-lang.org/std/alloc/struct.System.html#method.realloc": {
         "row": 287,
         "ptr": {
            "Allocated": [
               "`ptr` must be currently allocated via this allocator."
            ],
            "Freed": [
               "If this returns a non-null pointer, then ownership of the memory block referenced by `ptr` has been transferred to this allocator."
            ]
         },
         "layout": {
            "Layout": [
               "`layout` has non-zero size.",
               "`layout` must be the same layout that was used to allocate that block of memory."
            ]
         },
         "new_size": {
            "Bounded": [
               "`new_size` must be greater than zero.",
               "`new_size`, when rounded up to the nearest multiple of `layout.align()`, must not overflow `isize::MAX`."
            ]
         },
         "retval": {
            "Untyped": [
               "The allocated block of memory may or may not be initialized."
            ],
            "Freed": [
               "Returning a null pointer indicates that either memory is exhausted."
            ]
         }
      },
      "https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn_unchecked": {
         "row": 288,
         "self": {
            "Initialized": [
               "Panics if a thread name was set and it contained null bytes."
            ]
         },
         "retval": {
            "Aliased": [
               "The spawned thread may outlive the caller. The caller has to ensure that the spawned thread does not outlive any references in the supplied thread closure and its return type."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.assume.html": {
         "row": 289,
         "b": {
            "Bounded": [
               "If the condition is false, the behavior is undefined."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.const_allocate.html": {
         "row": 291,
         "align": {
            "Bounded": [
               "`align` argument must be a power of two."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.const_deallocate.html": {
         "row": 292,
         "align": {
            "Bounded": [
               "`align` argument must be a power of two."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.const_eval_select.html": {
         "row": 293,
         "called_in_const": {
            "Bounded": [
               "The two functions must behave observably equivalent. Safe code in other crates may assume that calling a `const fn` at compile-time and at run-time produces the same result. A function that produces a different result when evaluated at run-time, or has any other observable side-effects, is unsound."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.ctlz_nonzero.html": {
         "row": 294,
         "x": {
            "Bounded": [
               "Like `ctlz`, but extra-unsafe as it returns `undef` when given an `x` with value `0`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.cttz_nonzero.html": {
         "row": 295,
         "x": {
            "Bounded": [
               "Like `cttz`, but extra-unsafe as it returns `undef` when given an `x` with value `0`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.exact_div.html": {
         "row": 296,
         "x": {
            "Bounded": [
               "Resulting in undefined behavior where `x % y != 0`.",
               "Resulting in undefined behavior where `x == T::MIN && y == -1`."
            ]
         },
         "y": {
            "Bounded": [
               "Resulting in undefined behavior where `y == 0`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.fadd_fast.html": {
         "row": 297,
         "x": {
            "Bounded": [
               "Float addition that allows optimizations based on algebraic rules. The calculated result cannot overflow `T::MAX` or `T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.fdiv_fast.html": {
         "row": 298,
         "x": {
            "Bounded": [
               "Float division that allows optimizations based on algebraic rules. The calculated result cannot overflow `T::MAX` or `T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.fmul_fast.html": {
         "row": 299,
         "x": {
            "Bounded": [
               "Float multiplication that allows optimizations based on algebraic rules. The calculated result cannot overflow `T::MAX` or `T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.frem_fast.html": {
         "row": 300,
         "x": {
            "Bounded": [
               "Float remainder that allows optimizations based on algebraic rules. The calculated result cannot overflow `T::MAX` or `T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.fsub_fast.html": {
         "row": 301,
         "x": {
            "Bounded": [
               "Float subtraction that allows optimizations based on algebraic rules. The calculated result cannot overflow `T::MAX` or `T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.ptr_offset_from.html": {
         "row": 311,
         "self": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`ptr` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "EST": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "origin": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`base` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.ptr_offset_from_unsigned.html": {
         "row": 312,
         "ptr": {
            "Bounded": [
               "The distance between the pointers, in bytes, cannot overflow an `isize`.",
               "The distance between the pointers, in bytes, must be an exact multiple of the size of `T`.",
               "The distance between the pointers must be non-negative (`ptr` >= `base`)."
            ],
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`ptr` must be either in bounds or one byte past the end of the same allocated object.",
               "Both pointers must be derived from a pointer to the same object."
            ],
            "EST": [
               "This function panics if `T` is a Zero-Sized Type (ZST)."
            ]
         },
         "base": {
            "Allocated": [
               ""
            ],
            "Dereferencable": [
               "`base` must be either in bounds or one byte past the end of the same allocated object."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.raw_eq.html": {
         "row": 313,
         "a": {
            "Initialized": [
               "It's UB to call this if any of the bytes in `*a` are uninitialized or carry a pointer value."
            ],
            "Layout": [
               "Note that this is a stricter criterion than just the values being fully-initialized: if `T` has padding, it's UB to call this intrinsic."
            ]
         },
         "b": {
            "Initialized": [
               "It's UB to call this if any of the bytes in `*b` are uninitialized or carry a pointer value."
            ],
            "Layout": [
               "Note that this is a stricter criterion than just the values being fully-initialized: if `T` has padding, it's UB to call this intrinsic."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.unaligned_volatile_load.html": {
         "row": 318,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "`src` must point to a properly initialized value of type `T`."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ]
         },
         "retval": {
            "DualOwned": [
               "If `T` is not `Copy`, using both the returned value and the value at `src` can violate memory safety. Note that assigning to `src` counts as a use because it will attempt to drop the value at `src`.",
               "However, storing non-`Copy` types in volatile memory is almost certainly incorrect."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.unaligned_volatile_store.html": {
         "row": 319,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Leaked": [
               "This is safe, but it could leak allocations or resources, so care should be taken not to overwrite an object that should be dropped."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.unchecked_add.html": {
         "row": 320,
         "x": {
            "Bounded": [
               "Returns the result of an unchecked addition, resulting in undefined behavior when `x + y > T::MAX` or `x + y < T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.unchecked_mul.html": {
         "row": 321,
         "x": {
            "Bounded": [
               "Returns the result of an unchecked multiplication, resulting in undefined behavior when `x * y > T::MAX` or `x * y < T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.unchecked_sub.html": {
         "row": 322,
         "x": {
            "Bounded": [
               "Returns the result of an unchecked subtraction, resulting in undefined behavior when `x - y > T::MAX` or `x - y < T::MIN`."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.volatile_copy_memory.html": {
         "row": 323,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dst` can violate memory safety. Note that assigning to `*dst` counts as a use because it will attempt to drop the value at `*dst`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.volatile_copy_nonoverlapping_memory.html": {
         "row": 324,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dst` with the same size."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dst` with the same size."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dst` can violate memory safety. Note that assigning to `*dst` counts as a use because it will attempt to drop the value at `*dst`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.volatile_set_memory.html": {
         "row": 325,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned ('min_align_of::<T>()')."
            ],
            "Untyped": [
               "Additionally, note that changing `dst` in this way can easily lead to undefined behavior (UB) later if the written bytes are not a valid representation of some `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.vtable_align.html": {
         "row": 326,
         "ptr": {
            "EST": [
               "`ptr` must point to a vtable."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.vtable_size.html": {
         "row": 327,
         "ptr": {
            "EST": [
               "`ptr` must point to a vtable."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.copy.html": {
         "row": 328,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dst` can violate memory safety. Note that assigning to `*dst` counts as a use because it will attempt to drop the value at `*dst`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.copy_nonoverlapping.html": {
         "row": 329,
         "src": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dst` with the same size."
            ],
            "Layout": [
               "`src` must be properly aligned."
            ]
         },
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object.",
               "The region of memory beginning at `src` with a size of `count * size_of::<T>()` bytes must not overlap with the region of memory beginning at `dst` with the same size."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "DualOwned": [
               "If `T` is not `Copy`, using both the values in the region beginning at `self` and the region beginning at `*dst` can violate memory safety. Note that assigning to `*dst` counts as a use because it will attempt to drop the value at `*dst`."
            ],
            "Untyped": [
               "The copy is untyped in the sense that data may be uninitialized or otherwise violate the requirements of `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.drop_in_place.html": {
         "row": 330,
         "to_drop": {
            "Allocated": [
               "`to_drop` must be nonnull, even if T has size 0.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Initialized": [
               "The value `to_drop` points to must be valid for dropping, which may mean it must uphold additional invariants. These invariants depend on the type of the value being dropped."
            ],
            "Dereferencable": [
               "The memory range of the given size starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`to_drop` must be properly aligned, even if `T` has size 0.",
               "Unaligned values cannot be dropped in place, they must be copied to an aligned location first using `ptr::read_unaligned`."
            ],
            "Freed": [
               "Executes the destructor (if any) of the pointed-to value."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.transmute.html": {
         "row": 331,
         "src": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type.",
               "To transmute the inner type of the contents of a container, you must make sure to not violate any of the container's invariants."
            ],
            "Layout": [
               "Both types must have the same size.",
               "Note that source and destination are passed by-value, which means if `Src` or `Dst` contain padding, that padding is not guaranteed to be preserved by transmute.",
               "When transmuting values that point elsewhere (such as pointers, references, boxes…), the caller has to ensure proper alignment of the pointed-to values."
            ]
         },
         "retval": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type."
            ],
            "Aliased": [
               "It can turn a `*mut T` into an `&mut T`.",
               "It can extend a lifetime, or shorten an invariant lifetime."
            ],
            "Untyped": [
               "It is therefore your responsibility to guarantee that every value passed to transmute is valid at both types `Src` and `Dst`. Failing to uphold this condition may lead to unexpected and unstable compilation results."
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.write_bytes.html": {
         "row": 332,
         "dst": {
            "Allocated": [
               "A null pointer is never valid, not even for accesses of size zero.",
               "Even for operations of size zero, the pointer must not be pointing to deallocated memory."
            ],
            "Dereferencable": [
               "The memory range of the given size (`count * size_of::<T>()` bytes) starting at the pointer must all be within the bounds of a single allocated object."
            ],
            "Layout": [
               "`dst` must be properly aligned."
            ],
            "Untyped": [
               "Additionally, note that changing `dst` in this way can easily lead to undefined behavior (UB) later if the written bytes are not a valid representation of some `T`."
            ],
            "Leaked": [
               ""
            ]
         }
      },
      "https://doc.rust-lang.org/core/intrinsics/fn.transmute_unchecked.html": {
         "row": 333,
         "src": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type.",
               "To transmute the inner type of the contents of a container, you must make sure to not violate any of the container's invariants."
            ],
            "Layout": [
               "Both types must have the same size.",
               "Note that source and destination are passed by-value, which means if `Src` or `Dst` contain padding, that padding is not guaranteed to be preserved by transmute.",
               "When transmuting values that point elsewhere (such as pointers, references, boxes…), the caller has to ensure proper alignment of the pointed-to values."
            ]
         },
         "retval": {
            "Initialized": [
               "Both the argument and the result must be valid at their given type."
            ],
            "Aliased": [
               "It can turn a `*mut T` into an `&mut T`.",
               "It can extend a lifetime, or shorten an invariant lifetime."
            ],
            "Untyped": [
               "It is therefore your responsibility to guarantee that every value passed to transmute is valid at both types `Src` and `Dst`. Failing to uphold this condition may lead to unexpected and unstable compilation results."
            ]
         }
      }
   }
}"#;