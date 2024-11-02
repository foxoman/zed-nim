; =============================================================================
; generalized_strings
; regex in generalized_strings
(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "regex")
  (#any-of? @_string_prefix "re" "rex"))

; format string in generalized_strings
(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "nim_format_string")
  (#eq? @_string_prefix "fmt"))

; format string in normal strings with & prefix
(prefix_expression
  operator: (operator) @_string_prefix
  .
  (_
    (string_content) @content)
  (#set! "language" "nim_format_string")
  (#eq? @_string_prefix "&"))

; sql in generalized_strings
; and anything you like as long as the function name is the same as the injected language's parser

(generalized_string
  function: (identifier) @language
  (string_content) @content
  (#not-any-of? @language "re" "rex" "fmt" "md"))

(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "markdown")
  (#eq? @_string_prefix "md"))

; =============================================================================
; emit pragma
; C / CPP / OBJC / JAVASCRIPT
; a `#<no whitespace><language name>`
; has to directly precede the {.emit: "<language code>".} pragma
; eg.:
; #objc
; {.emit: "<objc code>".}
; OR
; #javascript
; {.emit: "<javascript code>".}
; normal strings
((comment
  (comment_content) @language)
  .
  (pragma_statement
    (pragma_list
      (colon_expression
        left: (identifier) @_emit_keyword
        (#eq? @_emit_keyword "emit")
        right: (_
          (string_content) @content)))))


; =============================================================================
; asm statement
((assembly_statement
    (_
      (string_content) @content))
  (#set! "language" "asm"))

; =============================================================================
; comments
; NOTE: ts "comment" parser heavily impacts performance
; markdown parser in documentation_comment
(documentation_comment
  (comment_content) @content
  (#set! "language" "markdown_inline"))

; markdown parser in block_documentation_comment
(block_documentation_comment
  (comment_content) @content
  (#set! "language" "markdown"))
