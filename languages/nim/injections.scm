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

; TODO: this doesn't work dynamically, must be a workaround
; (generalized_string
;   function: (identifier) @injection.language
;   (string_content) @content
;   (#not-any-of? @injection.language "re" "rex" "fmt"))

; SQL, HTML, CSS, XML, GLSL and Markdown
(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "sql")
  (#eq? @_string_prefix "sql"))

(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "html")
  (#eq? @_string_prefix "html"))

(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "css")
  (#eq? @_string_prefix "css"))

(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "xml")
  (#eq? @_string_prefix "xml"))

(generalized_string
  function: (identifier) @_string_prefix
  .
  (string_content) @content
  (#set! "language" "glsl")
  (#eq? @_string_prefix "glsl"))

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
  (comment_content) @_language
  (#eq? @_language "cpp"))
  .
  (pragma_statement
    (pragma_list
      (colon_expression
        left: (identifier) @_emit_keyword
        (#eq? @_emit_keyword "emit")
        right: (_
          (string_content) @content))))
  (#set! "language" "cpp"))

((comment
  (comment_content) @_language
  (#eq? @_language "c"))
  .
  (pragma_statement
    (pragma_list
      (colon_expression
        left: (identifier) @_emit_keyword
        (#eq? @_emit_keyword "emit")
        right: (_
          (string_content) @content))))
  (#set! "language" "c"))

((comment
  (comment_content) @_language
  (#eq? @_language "objc"))
  .
  (pragma_statement
    (pragma_list
      (colon_expression
        left: (identifier) @_emit_keyword
        (#eq? @_emit_keyword "emit")
        right: (_
          (string_content) @content))))
  (#set! "language" "objc"))

((comment
  (comment_content) @_language
  (#eq? @_language "javascript"))
  .
  (pragma_statement
    (pragma_list
      (colon_expression
        left: (identifier) @_emit_keyword
        (#eq? @_emit_keyword "emit")
        right: (_
          (string_content) @content))))
  (#set! "language" "javascript"))


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
