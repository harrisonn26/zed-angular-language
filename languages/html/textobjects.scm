; Control flow "around" selections (include the entire statement)
(if_statement) @function.around
(else_statement) @function.around
(for_statement) @function.around
(switch_statement) @function.around

; Control flow "inside" selections (exclude the curly braces)
; For statement_block: select only the content between { and }
(statement_block
  "{" . (_)* @function.inside . "}")

; For switch_body: select only the content between { and }
(switch_body
  "{" . (_)* @function.inside . "}")
