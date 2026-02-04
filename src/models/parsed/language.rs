use std::{ops::Deref, path::Path};

use strum_macros::EnumIter;
use tree_sitter_language::LanguageFn;

use crate::parser;

/// The supported languages.
#[derive(Debug, Clone, Copy, EnumIter, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Language {
    /// Golang
    Go,

    /// Rust
    Rust,

    /// Lua
    Lua,

    /// TypeScript
    TypeScript,

    /// TypeScript JSX
    TypeScriptJsx,

    ///  Javascript
    Javascript,

    /// Javascript JSX
    JavascriptJsx,

    /// Clojure
    Clojure,
}

/// A particular file extension for a supported language ([`Language`]).
#[derive(Debug, Clone, Copy)]
pub struct FileExtension<'a>(&'a str);

impl<'a> Deref for FileExtension<'a> {
    type Target = str;

    fn deref(&self) -> &'a Self::Target {
        self.0
    }
}

impl TryFrom<&FileExtension<'_>> for Language {
    type Error = parser::Error;

    fn try_from(value: &FileExtension<'_>) -> Result<Self, Self::Error> {
        match value.0.to_lowercase().as_str() {
            "go" => Ok(Self::Go),
            "rs" => Ok(Self::Rust),
            "lua" => Ok(Self::Lua),
            "ts" => Ok(Self::TypeScript),
            "tsx" => Ok(Self::TypeScriptJsx),
            "js" => Ok(Self::Javascript),
            "jsx" => Ok(Self::JavascriptJsx),
            "clj" => Ok(Self::Clojure),
            _ => Err(parser::Error::InvalidUri(value.0.to_string())),
        }
    }
}

impl From<Language> for FileExtension<'_> {
    fn from(value: Language) -> Self {
        FileExtension(match value {
            Language::Go => "go",
            Language::Rust => "rs",
            Language::Lua => "lua",
            Language::TypeScript => "ts",
            Language::TypeScriptJsx => "tsx",
            Language::Javascript => "js",
            Language::JavascriptJsx => "jsx",
            Language::Clojure => "clj",
        })
    }
}

impl TryFrom<&Path> for Language {
    type Error = parser::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let ext = match path.extension().and_then(|ext| ext.to_str()) {
            Some(e) => e.trim_start_matches('.').to_ascii_lowercase(),
            None => {
                return Err(parser::Error::InvalidUri(
                    path.to_string_lossy().to_string(),
                ));
            }
        };

        Self::try_from(&FileExtension(&ext))
    }
}

impl From<Language> for LanguageFn {
    fn from(value: Language) -> Self {
        match value {
            Language::Go => tree_sitter_go::LANGUAGE,
            Language::Rust => tree_sitter_rust::LANGUAGE,
            Language::Lua => tree_sitter_lua::LANGUAGE,
            Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            Language::TypeScriptJsx => tree_sitter_typescript::LANGUAGE_TSX,
            Language::Javascript | Language::JavascriptJsx => tree_sitter_javascript::LANGUAGE,
            Language::Clojure => tree_sitter_clojure_orchard::LANGUAGE,
        }
    }
}

impl Language {
    /// Get the language-specific Treesitter symbol query, in order
    /// to exact all the symbols from a particular source file.
    #[must_use]
    pub const fn get_symbol_query(&self) -> &'static str {
        match self {
            Self::Go => include_str!("./../../parser/treesitter/scm/golang_symbols.scm"),
            Self::Rust => include_str!("./../../parser/treesitter/scm/rust_symbols.scm"),
            Self::Lua => include_str!("./../../parser/treesitter/scm/lua_symbols.scm"),
            Self::TypeScript => {
                include_str!("./../../parser/treesitter/scm/typescript_symbols.scm")
            }
            Self::TypeScriptJsx => {
                include_str!("./../../parser/treesitter/scm/typescript_jsx_symbols.scm")
            }
            Self::Javascript => {
                include_str!("./../../parser/treesitter/scm/javascript_symbols.scm")
            }
            Self::JavascriptJsx => {
                include_str!("./../../parser/treesitter/scm/javascript_jsx_symbols.scm")
            }
            Self::Clojure => include_str!("./../../parser/treesitter/scm/clojure_symbols.scm"),
        }
    }
}
