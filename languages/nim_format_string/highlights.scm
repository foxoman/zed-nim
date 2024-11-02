(string_literal) @string
(matching_curlies
  opening_curly: (opening_curly) @punctuation.special
  equals: (equals)? @punctuation.special
  closing_curly: (closing_curly) @punctuation.special)

(format_specifiers
  colon: (colon) @punctuation.delimiter
  fill_align: (fill_align)? @conditional.ternary
  sign: (sign)? @operator
  hash: (hash)? @punctuation.special
  zero: (zero)? @field
  min_width: (min_width)? @constant
  precision: (precision)? @constant
  type: (type)? @type)

(matching_curlies
  nim_expression: (nim_expression
    escaped_curly: (escaped_curly)+ @string.escape)
)
