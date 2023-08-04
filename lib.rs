/*
Copyright 2023 Benjamin Richcreek

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
//!# Struct-inator Traits
//!This is a library for implementing a trait, [`SpecifyCreatableStruct`] - which, when implemented, alliows for quick and easy conversion between rust [`Iterators`](Iterator) and
//! types that implement [`SpecifyCreatableStruct`].
//! 
//! More specifically, any type implementing [`SpecifyCreatableStruct`] must implement [`create_struct`](SpecifyCreatableStruct::create_struct), a function converting directly from an [`Iterator`] over items of type [`NamedField<I>`], where 
//! `I` is the same type as [`InnerIteratorType`](SpecifyCreatableStruct)
//! 
//! You can implement this trait yourself, or you can automatically implement it for a user-defined struct using [`structinator`](<https://example.com>)
//! 
//! [structinator]: https://example.com
//! 
//! # Example
//!```rust
//!use structinator_traits::*;
//!use structinator::iter_convertable;
//!use enum_unwrapper::unique_try_froms;
//!#[unique_try_froms()]
//!enum WaffleInfo {
//!    Topping(u8),
//!    Layers(u16),
//!}
//!#[iter_convertable(WaffleInfo)]
//!struct Waffles {
//!    syrup_amount: u8,
//!    butter_amount: u8,
//!    layer_count: u16,
//!}
//!fn main() {
//!    let mut iterator = [NamedField::<WaffleInfo> {
//!        name: String::from("butter_amount"),
//!       wrapped_value: WaffleInfo::Topping(44),
//!    }, NamedField::<WaffleInfo> {
//!        name: String::from("layer_count"),
//!        wrapped_value: WaffleInfo::Layers(444),
//!    }, NamedField::<WaffleInfo> {
//!        name: String::from("syrup_amount"),
//!        wrapped_value: WaffleInfo::Topping(4),
//!    }].into_iter();
//!    let generated_struct = Waffles::create_struct(&mut iterator).unwrap();
//!    assert_eq!(4,generated_struct.syrup_amount);
//!    assert_eq!(44,generated_struct.butter_amount);
//!    assert_eq!(444,generated_struct.layer_count);
//!}
//!```
/// [`SpecifyCreatableStruct`]'s original intended use case was with user-defined [`struct`](https://doc.rust-lang.org/1.58.1/std/keyword.struct.html)s, and this structure was designed for convenience, allowing the implementor to store both a [`String`]ification of the field's name,
/// and the field's value. Note that it is also the type that the argument passed to [`create_struct`](SpecifyCreatableStruct::create_struct) must iterate over.
pub struct NamedField<I> {
    ///Intended to hold the name of the field `wrapped_value` should be assigned to
    pub name: String,
    ///Intended to hold the value to be assigned to a given field in the target [`struct`](https://doc.rust-lang.org/1.58.1/std/keyword.struct.html)
    pub wrapped_value: I,
}
///Any type implementing this trait must be convertable from an [`Iterator`] over elements of type [`NamedField<I>`], where `I` is the same type assigned to [`InnerIteratorType`](SpecifyCreatableStruct::InnerIteratorType).
pub trait SpecifyCreatableStruct: Sized {
    ///The type contained in [`NamedField`]
    type InnerIteratorType;
    ///The type that should be returned if the conversion fails
    type Error;
    /// The function that should be called to attempt a conversion from an [`Iterator`] to the type implementing this trait.
    fn create_struct(seed_iterator: &mut dyn Iterator<Item = NamedField<Self::InnerIteratorType>>) -> Result<Self,Self::Error>;
}