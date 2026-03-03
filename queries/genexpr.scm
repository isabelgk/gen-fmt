; Opaque leaves — don't recurse into these
(string_literal) @leaf
(number_literal) @leaf
(comment) @leaf
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

; Semicolons: no space before, newline after
; NOTE: this also applies inside for() headers — fix if for loops are needed
";" @prepend_antispace @append_hardline

; Function declaration bodies: indent between braces
(function_declaration
  "{" @prepend_space @append_hardline @append_indent_start
  "}" @prepend_hardline @prepend_indent_end @append_hardline
)

; Control flow blocks (compound_statement used by if/while/do bodies)
(compound_statement
  "{" @append_hardline @append_indent_start
  "}" @prepend_hardline @prepend_indent_end
)

; Top-level: allow blank lines between top-level items
(translation_unit
  (_) @allow_blank_line_before
)

; Control flow keywords: space before the condition paren
["if" "for" "while" "do"] @append_space

; else: space before and after
"else" @prepend_space @append_space

; Ternary operators
"?" @prepend_space @append_space
":" @prepend_space @append_space

; Parentheses: no space inside (for calls, control flow, etc.)
"(" @append_antispace
")" @prepend_antispace

; require directives: newline after each
(compiler_command) @append_hardline
