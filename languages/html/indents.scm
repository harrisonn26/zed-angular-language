(start_tag ">" @end) @indent
(self_closing_tag "/>" @end) @indent

(element
  (start_tag) @start
  (end_tag)? @end) @indent
[
  (statement_block)
  (switch_statement)
] @indent.begin

(statement_block
  "{" @indent.branch)

(statement_block
  "}" @indent.end)

"}" @indent.branch

"}" @indent.end
