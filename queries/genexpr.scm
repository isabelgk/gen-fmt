; Opaque leaves — don't recurse into these
(string_literal) @leaf
(number_literal) @leaf
(comment) @leaf @append_hardline
(inlet_outlet) @leaf
(identifier) @leaf
(type_specifier) @leaf
(assignment_operator) @leaf

; Declarations: space between type and identifier
(declaration (type_specifier) @append_space)

; Binary expressions: spaces around operator
(binary_expression operator: _ @prepend_space @append_space)

; Assignment statements: spaces around operator
(single_assignment operator: _ @prepend_space @append_space)
(multiple_assignment operator: _ @prepend_space @append_space)

; Commas: no space before, one space after
"," @prepend_antispace @append_space

; for() header semicolons: no space before, space after
(for_statement ";" @prepend_antispace @append_space)

; Other emicolons: no space before, newline after
(declaration ";" @prepend_antispace @append_hardline)
(expr_statement_list ";" @prepend_antispace @append_hardline)
(statement ";" @prepend_antispace @append_hardline)
(continue_statement ";" @prepend_antispace @append_hardline)
(break_statement ";" @prepend_antispace @append_hardline)
(return_statement ";" @prepend_antispace @append_hardline)
(do_statement ";" @prepend_antispace @append_hardline)

; Function declaration bodies: indent between braces
(function_declaration
  "{" @prepend_space @append_hardline @append_indent_start
  "}" @prepend_hardline @prepend_indent_end @append_hardline
)

; Control flow blocks (compound_statement used by if/while/do bodies)
(compound_statement
  "{" @prepend_space @append_hardline @append_indent_start
  "}" @prepend_hardline @prepend_indent_end @append_hardline
)

; Top-level: allow blank lines between top-level items
(translation_unit
  (_) @allow_blank_line_before
)

; Function bodies: allow blank lines between declarations and statements
(function_declaration
  (_) @allow_blank_line_before
)

; Allow blank lines between statements within a function body
(expr_statement_list
  (_) @allow_blank_line_before
)

; return keyword: space before the expression
"return" @append_space

; Control flow keywords: space before the condition paren
["if" "for" "while" "do"] @append_space

; else: space after (newline before comes from preceding "}" @append_hardline)
"else" @append_space

; Ternary operators
"?" @prepend_space @append_space
":" @prepend_space @append_space

; Parentheses: no space inside (for calls, control flow, etc.)
"(" @append_antispace
")" @prepend_antispace

; require directives: newline after each
(compiler_command) @append_hardline
