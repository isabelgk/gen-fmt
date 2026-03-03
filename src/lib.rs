use anyhow::Result;
use topiary_core::{formatter, Language, Operation, TopiaryQuery};
use topiary_tree_sitter_facade::Language as TsLanguage;

const QUERY: &str = include_str!("../queries/genexpr.scm");

pub fn format_str(input: &str) -> Result<String> {
    let grammar: TsLanguage = tree_sitter_genexpr::language().into();
    let query = TopiaryQuery::new(&grammar, QUERY)
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let language = Language {
        name: "genexpr".to_string(),
        query,
        grammar,
        indent: None,
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
    )
    .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(String::from_utf8(output)?)
}
