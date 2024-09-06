//! # Serde IEML
//!
//! A Rust library for using the [Serde](https://crates.io/crates/serde) serialization framework with data in IEML document format.
//!
//! IEML is a data description format whose key differences are the presence of tags and unordered anchors, and the ability to describe one object in multiple documents (files). Anchors are a primitive similar to references and pointers, which allows describing self-referential data structures and avoiding unnecessary copying of document contents.
//!
//! ```ieml
//! - < profession/programmer
//! 	name: > John Doe
//! 	language: Rust
//! 	experience: 2
//!
//! - < profession/programmer
//! 	name: > Juan Pérez
//! 	language: Rust
//! 	experience: 0.5
//! ```
//! profession/programmer.ieml
//! ```ieml
//! profession: Programmer
//! name: @name
//! experience: @experience
//! additional:
//! 	language: @language
//! ```
//!
//! There are three common ways that you might find yourself needing to work with IEML data in Rust.
//! - As text data. An unprocessed string of IEML data that you read from a file.
//! - As an untyped or loosely typed representation. Maybe you want to check that some IEML data is valid before passing it on, but without knowing the structure of what it contains. Or you want to do very basic manipulations like insert a key in a particular spot.
//! - As a strongly typed Rust data structure. When you expect all or most of your data to conform to a particular structure and want to get real work done without IEML's structure tripping you up.
//!
//! ## Progress
//! - [x] Data structures and APIs for working with them
//! - [x] Parsing from raw input to an intermediate data structure
//! - [x] Implementation of Serde traits for deserialization
//! - [ ] Generating a set of documents from an intermediate data structure
//! - [ ] Implementation of Serde traits for serialization
//!
//! ## Dependency
//!
//! ```toml
//! [dependencies]
//! serde = "1"
//! serde_ieml = "0.3"
//! ```
//!
//! ## Operating on loosely typed IEML data
//! IEML data is stored in a [`Data`] structure, the details of its implementation are hidden, but the library provides several structures for viewing this data. The main one is [`View`] from it you can get structures for each of the supported primitives.
//!
//! The document reader can be passed to the function [`de::parse::parse_with_reader`] to get [`Data`], the simplest reader is `&str` containing the top document directly. And using the [`view`](Data::view) method to get [`View`].
//!
//! ```
//! use serde_ieml::{Data, View, de::parse::parse_with_reader};
//!
//! // Some IEML input data as a &str. Maybe this comes
//! // from the user.
//! let input = r#"
//! name: > John Doe
//! age: 43
//! phones:
//! 	- +44 1234567
//! 	- +44 2345678
//! "#;
//!
//! // Parse the string of data into serde_ieml::Data.
//! let data = parse_with_reader(input).unwrap();
// 
//! // Getting reference structure allowing to read
//! // IEML data.
//! let view = data.view();
// 
//! // Getting an object to view the map.
//! let map = view.map().unwrap();
// 
//! // Getting an object to view the list contained
//! // in the map.
//! let list = map.get("phones").unwrap().list().unwrap();
//!
//! // The first `string` call returns a structure
//! // allowing to read string primitive data
//! assert_eq!("John Doe", map.get("name").unwrap().string().unwrap().string());
//! assert_eq!("+44 1234567", list.get(0).unwrap().raw().unwrap().raw());
//! ```
//!
//! ## Parsing IEML as strongly typed data structures
//!
//! Serde provides a powerful way of mapping IEML data into Rust data structures largely automatically.
//!
//! ```
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//!     phones: Vec<String>,
//! }
//!
//! // Some IEML input data as a &str. Maybe this comes from the user.
//! let input = r#"
//! name: > John Doe
//! age: 43
//! phones:
//! 	- > +44 1234567
//! 	- > +44 2345678
//! "#;
//!
//! // Parse the string of data into a Person object.
//! let p: Person = serde_ieml::from_source(input).unwrap();
//!
//! // Do things just like with any other Rust data structure.
//! assert_eq!("John Doe", p.name);
//! assert_eq!("+44 1234567", p.phones[0]);
//! ```
//!
//! Serde will automatically interpret the input data as a `Person` and produce informative error messages if the layout does not conform to what a Person is expected to look like.
//!
//! Any type that implements Serde's [`Deserialize`](serde::Deserialize) trait can be deserialized this way. This includes built-in Rust standard library types like `Vec<T>` and `HashMap<K, V>`, as well as any structs or enums annotated with `#[derive(Deserialize)]`.
//! 
//! Once we have p of type `Person`, our IDE and the Rust compiler can help us use it correctly like they do for any other Rust code. The IDE can autocomplete field names to prevent typos, which was impossible in the [`Data`] representation. And the Rust compiler can check that when we write `p.phones[0]`, then p.phones is guaranteed to be a `Vec<String>` so indexing into it makes sense and produces a `String`.
//!
//! The necessary setup for using Serde's derive macros is explained on the [`Using derive`](https://serde.rs/derive.html) page of the Serde site.
//! 
//! ## Constructing IEML data
//! 
//! Serde IEML provides a [`make`](data::make) module to build [`Data`] objects with a guarantee of a valid structure.
//! 
//! ```
//! use serde_ieml::data::{make, name::Name};
//! use std::convert::Infallible;
//! 
//! // The same position is used for all nodes to simplify the example.
//! let mark = Default::default();
//!
//! // The type of `john` is `serde_ieml::Data`
//! let (john, _) = make::make::<_, Infallible, _>(
//!     mark, 
//!     make::map(mark, |token| {
//!         let (token, _) = token.add(
//!             mark, 
//!             Name::new("name").unwrap(), 
//!             make::string(mark, (), "John Doe")
//!         )?;
//!         let (token, _) = token.add(
//!             mark, 
//!             Name::new("age").unwrap(), 
//!             make::raw(mark, (), "43")
//!         )?;
//!         let (token, _) = token.add(
//!             mark, 
//!             Name::new("phones").unwrap(), 
//!             make::list(
//!                 mark, 
//!                 |token| {
//!                     let (token, _) = token.add(
//!                         make::raw(mark, (), "+44 1234567")
//!                     )?;
//!                     let (token, _) = token.add(
//!                         make::raw(mark, (), "+44 1234567")
//!                     )?;
//!                     Ok((token, ()))
//!                 }
//!             )
//!         )?;
//!         Ok((token, ()))
//!     })
//! ).unwrap();
//!
//! // Getting reference structure allowing to read
//! // IEML data.
//! let view = john.view();
//!
//! // Getting an object to view the map.
//! let map = view.map().unwrap();
//!
//! // Getting an object to view the list contained
//! // in the map.
//! let list = map.get("phones").unwrap().list().unwrap();
//!
//! assert_eq!("John Doe", map.get("name").unwrap().string().unwrap().string());
//! ```
//! 
//! All [`make`](data::make) module functions except [`make`](data::make::make) and [`make_document`](data::make::make_document) return a lambda accepting a value of type [`Token`](data::make::Token), all token types describe the possibility or impossibility (if it is a used or error token) to add exactly one or more nodes to a particular IEML location. 
//! 
//! The main purpose of this system is to allow you to write your own deserializers of IEML from other formats. This system avoids a number of errors, such as cyclic nodes or closed sections of a structure to which access is lost. The need for this system arises because IEML differs from other formats by its non-tree structure.

pub mod data;
pub mod de;
pub mod error;

pub use de::{from_source, from_source_advanced};

pub use data::Data;
pub use data::View;
