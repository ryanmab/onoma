#![crate_name = "onoma"]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_debug_implementations, rust_2018_idioms, rustdoc::all)]
#![allow(rustdoc::private_doc_tests)]
#![forbid(unsafe_code)]

//! # Onoma
//!
//! > **ὄνομα** — *Onoma* (pronounced `OH-no-ma`) is Greek for **“name”**, signifying not just a
//! > label, but the *essence* or character of a thing.
//!
//! Onoma is a fast, language-agnostic semantic symbol indexer and fuzzy finder, which supports real-time
//! cross-language symbol matching, without needing a full language server and without applying limits
//! to workspace-wide queries.
//!
//! It achieves this through incremental indexing with Tree-sitter and filesystem events,
//! and includes typo-resistant fuzzy matching with a scoring system to prioritise relevant results.
//!
//! While Onoma can be used as a standalone crate, its primary goal is to act as an editor-agnostic
//! indexer and resolver which can be cross-compiled and integrated into text editors and IDEs.
//!
//! ## Supported Languages
//!
//! - Rust (`.rs`)
//! - Go (`.go`)
//! - Lua (`.lua`)
//! - Clojure (`.clj`)
//! - TypeScript (`.ts` and `.tsx`) / JavaScript (`.js` and `.jsx`)
//!
//! ## Usage
//!
//! ### 1. Editor Integrations
//!
//! > _Feel free to [open an issue](https://github.com/ryanmab/onoma/issues) with ideas for additional
//! > editor integrations._
//!
//! ![onoma.nvim](https://github.com/user-attachments/assets/cadc6d39-2491-4ce9-9f61-8a4f8598d62a)
//!
//! Currently, Onoma is integrated with:
//!
//! 1. **Neovim**, using [onoma.nvim](https://github.com/ryanmab/onoma.nvim) with Snacks Picker
//!
//! ### 2. Standalone Crate
//!
//! ```toml
//! [dependencies]
//! onoma = "0.0.4"
//! ```
//!
//! #### Documentation
//!
//! Full documentation is available on [docs.rs](https://docs.rs/onoma/latest/onoma/).
//!
//! ## Contributing
//!
//! Contributions are welcome!
//!
//! The core Onoma backend should contain all editor-agnostic functionality, including improvements
//! to indexing and fuzzy matching.
//!
//! For editor-specific features or changes to bindings for a particular editor, please submit pull requests
//! in the respective editor repositories listed above.
//!
//! ### Testing
//!
//! The tests can be run with:
//!
//! ```sh
//! cargo test
//! ```
//!
//! ## Acknowledgments
//!
//! - [fff.nvim](https://github.com/dmtrKovalenko/fff.nvim/tree/main) for inspiring the semantic fuzzy finder design in Onoma.
//! - [snacks.nvim](https://github.com/folke/snacks.nvim/tree/main) for the excellent picker frontend.
//! - [frizbee](https://github.com/saghen/frizbee) for the high-performance SIMD implementation of fuzzy matching.

mod utils;

pub mod indexer;
pub mod models;
pub mod parser;
pub mod resolver;
pub mod watcher;
