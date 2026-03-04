use topiary_core::{formatter, FormatterError, Language, Operation, TopiaryQuery};
use topiary_tree_sitter_facade::Language as TsLanguage;

const QUERY: &str = include_str!("../queries/genexpr.scm");

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Topiary(#[from] FormatterError),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub fn format_str(input: &str) -> Result<String, Error> {
    let grammar: TsLanguage = tree_sitter_genexpr::language().into();
    let query = TopiaryQuery::new(&grammar, QUERY)?;
    let language = Language {
        name: "genexpr".to_string(),
        query,
        grammar,
        indent: Some("    ".to_string()),
    };
    let mut output = Vec::new();
    formatter(
        &mut input.as_bytes(),
        &mut output,
        &language,
        Operation::Format {
            skip_idempotence: false,
            tolerate_parsing_errors: false,
        },
    )?;
    Ok(String::from_utf8(output)?)
}
