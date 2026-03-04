pub mod ffi;

use topiary_core::{formatter, FormatterError, Language, Operation, TopiaryQuery};
use topiary_tree_sitter_facade::Language as TsLanguage;
use tree_sitter::{Node, Parser};

const QUERY: &str = include_str!("../queries/genexpr.scm");

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Topiary(#[from] FormatterError),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub fn format_str(
    input: &str,
    skip_idempotence: bool,
    tolerate_parsing_errors: bool,
) -> Result<String, Error> {
    let braced = insert_braces(input);

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
        &mut braced.as_bytes(),
        &mut output,
        &language,
        Operation::Format {
            skip_idempotence,
            tolerate_parsing_errors,
        },
    )?;
    Ok(String::from_utf8(output)?)
}

// Wrap braceless if/else bodies with braces before Topiary formats.
fn insert_braces(input: &str) -> String {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_genexpr::language())
        .expect("tree-sitter-genexpr language should be valid");
    let Some(tree) = parser.parse(input, None) else {
        return input.to_string();
    };

    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    collect_brace_edits(tree.root_node(), input.as_bytes(), &mut edits);

    if edits.is_empty() {
        return input.to_string();
    }

    // Apply from end to beginning so earlier byte offsets stay valid.
    edits.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result = input.to_string();
    for (start, end, replacement) in edits {
        result.replace_range(start..end, &replacement);
    }
    result
}

fn collect_brace_edits<'tree>(
    node: Node<'tree>,
    src: &[u8],
    edits: &mut Vec<(usize, usize, String)>,
) {
    let children: Vec<Node<'tree>> = {
        let mut cursor = node.walk();
        node.children(&mut cursor).collect()
    };

    if node.kind() == "selection_statement" {
        for child in &children {
            match child.kind() {
                // Braceless consequence: statement whose first named child is not compound_statement.
                "statement" => maybe_wrap(*child, src, edits, false),
                // else_clause contains the alternative (braceless or else-if).
                "else_clause" => {
                    let mut cursor = child.walk();
                    for grandchild in child.children(&mut cursor) {
                        if grandchild.kind() == "statement" {
                            // Don't wrap if it's an else-if chain.
                            maybe_wrap(grandchild, src, edits, true);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    for child in children {
        collect_brace_edits(child, src, edits);
    }
}

// Wrap `node` (a `statement`) in braces if its first named child is not already
// a `compound_statement`. When `skip_selection` is true, also skip wrapping if
// the first named child is a `selection_statement` (preserves `else if` chains).
fn maybe_wrap<'tree>(
    node: Node<'tree>,
    src: &[u8],
    edits: &mut Vec<(usize, usize, String)>,
    skip_selection: bool,
) {
    let first_named = {
        let mut cursor = node.walk();
        let x = node.named_children(&mut cursor).next();
        x
    };
    let inner_kind = first_named.map(|n| n.kind()).unwrap_or("");
    if inner_kind == "compound_statement" {
        return; // already braced
    }
    if skip_selection && inner_kind == "selection_statement" {
        return; // else-if: recurse will handle the inner bodies
    }
    let start = node.start_byte();
    let end = node.end_byte();
    let content = std::str::from_utf8(&src[start..end]).unwrap_or("");
    edits.push((start, end, format!("{{{}}}", content)));
}
