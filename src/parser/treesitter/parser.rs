use std::{path::Path, str::FromStr};

use tokio::{fs::File, io::AsyncReadExt};
use tree_sitter::StreamingIterator;

use crate::{models, parser, utils::normalise_symbol_name};

/// A source code parser, which can read source code and output a fully parsed
/// [`crate::models::parsed::Index`].
///
/// Under the hood this uses Treesitter, and custom built queries to parse symbols out of source
/// code.
#[derive(Debug, Default, Clone, Copy)]
pub struct Parser {}

impl parser::Parser for Parser {
    type ParseContext = super::Context;

    type ParseOutput = super::Output;

    /// Create an index using Treesitter, for a given file.
    ///
    /// This returns the parsed index, as well as the underlying Treesitter tree,
    /// which can be passed back in on subsequent calls to [`crate::parser::treesitter::Parser`]
    /// in order to enable incremental tree parsing.
    ///
    /// # Errors
    ///
    /// Returns an error if the file could not be parsed successfully into an index using
    /// Treesitter.
    async fn parse(
        &self,
        file: &Path,
        ctx: &Self::ParseContext,
    ) -> parser::Result<Self::ParseOutput> {
        let language = models::parsed::Language::try_from(file)?;

        let parser_language: tree_sitter::Language =
            std::convert::Into::<tree_sitter_language::LanguageFn>::into(language).into();

        let (tree, file_content) =
            Self::parse_file_into_tree(file, &parser_language, ctx.existing_tree.as_ref()).await?;

        let symbols =
            Self::extract_symbols(file, &file_content, &tree, language, &parser_language)?;

        let mut index = models::parsed::Index::new(models::parsed::Type::TreeSitter);

        for symbol in symbols {
            index.append_symbol(symbol);
        }

        Ok(super::Output { index, tree })
    }
}

impl Parser {
    /// Parse a file into a tree, optionally using an existing tree from a previous call, in order
    /// to use incremental tree parsing to optimize speed.
    async fn parse_file_into_tree(
        file: &Path,
        language: &tree_sitter::Language,
        existing_tree: Option<&tree_sitter::Tree>,
    ) -> parser::Result<(tree_sitter::Tree, Vec<u8>)> {
        let mut parser = tree_sitter::Parser::new();

        parser
            .set_language(language)
            .map_err(|e| parser::Error::InvalidLanguage(Some(e)))?;

        let mut file = File::open(file).await.map_err(parser::Error::InvalidFile)?;

        let mut file_content = Vec::new();

        file.read_to_end(&mut file_content)
            .await
            .map_err(parser::Error::InvalidFile)?;

        let tree: tree_sitter::Tree = parser
            .parse(&mut file_content, existing_tree)
            .ok_or(parser::Error::InvalidLanguage(None))?;

        Ok((tree, file_content))
    }

    /// Extract the relevant symbols from the Treesitter tree, so that they can be parsed into
    /// a set of [`models::parsed::Symbol`].
    ///
    /// This relies upon Treesitter queries, which are language-specific but extract nodes in
    /// a language-agnostic way.
    ///
    /// See [`models::parsed::Language::get_symbol_query`] for the underlying Treesitter queries
    /// for supported languages.
    fn extract_symbols(
        file: &Path,
        file_content: &Vec<u8>,
        tree: &tree_sitter::Tree,
        language: models::parsed::Language,
        parser_language: &tree_sitter::Language,
    ) -> parser::Result<impl Iterator<Item = models::parsed::Symbol>> {
        let query = tree_sitter::Query::new(parser_language, language.get_symbol_query())
            .map_err(parser::Error::InvalidQuery)?;

        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), file_content.as_slice());

        let capture_names = query.capture_names();

        let mut symbols = Vec::new();

        while let Some(m) = matches.next() {
            for c in m.captures {
                let Ok(kind) =
                    models::parsed::SymbolKind::from_str(capture_names[c.index as usize])
                else {
                    continue;
                };

                let Ok(name) = c.node.utf8_text(file_content).map(normalise_symbol_name) else {
                    continue;
                };

                if name.is_empty() {
                    // Any normalised symbol names which come out empty (i.e. just whitespace)
                    // can be ignored
                    continue;
                }

                let mut symbol = models::parsed::Symbol::new(kind, &name);

                let start_position = c.node.start_position();
                let end_position = c.node.end_position();

                let occurrence = models::parsed::Occurrence::new(
                    language,
                    file,
                    models::parsed::Range::new(
                        start_position.row + 1,
                        end_position.row + 1,
                        start_position.column + 1,
                        end_position.column + 1,
                    ),
                    models::parsed::Roles(vec![models::parsed::SymbolRole::Definition]),
                );
                symbol.add_occurrence(occurrence);

                symbols.push(symbol);
            }
        }

        Ok(symbols.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use insta::assert_debug_snapshot;
    use itertools::Itertools;

    use crate::parser::{Parser, treesitter::Context};

    #[tokio::test]
    pub async fn test_parsing_rust() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(PathBuf::from("tests/fixtures/rust.rs").as_path(), &ctx)
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_go() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(PathBuf::from("tests/fixtures/go.go").as_path(), &ctx)
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_lua() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(PathBuf::from("tests/fixtures/lua.lua").as_path(), &ctx)
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_typescript() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(
                PathBuf::from("tests/fixtures/typescript.ts").as_path(),
                &ctx,
            )
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_typescript_tsx() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(
                PathBuf::from("tests/fixtures/typescript.tsx").as_path(),
                &ctx,
            )
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_javascript() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(
                PathBuf::from("tests/fixtures/javascript.js").as_path(),
                &ctx,
            )
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_javascript_jsx() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(
                PathBuf::from("tests/fixtures/javascript.jsx").as_path(),
                &ctx,
            )
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }

    #[tokio::test]
    pub async fn test_parsing_clojure() {
        let parser = super::Parser::default();

        let ctx = Context::default();

        let output = parser
            .parse(PathBuf::from("tests/fixtures/clojure.clj").as_path(), &ctx)
            .await;

        let index = output.expect("Index should always be available");

        assert_debug_snapshot!(index.index.symbols.iter().sorted());
    }
}
