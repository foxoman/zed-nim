; runnable main module
((when
    condition: (identifier) @run (#eq? @run "isMainModule"))
    (#set! tag nim-main))

((when
    condition: (parenthesized ((identifier) @run (#eq? @run "isMainModule")) ))
    (#set! tag nim-main))

; runnable tests
; suite
((call
    function: (identifier) @run (#eq? @run "suite")
    (argument_list
        ((interpreted_string_literal
               . (string_content) @nim_suite)
        )
        ":"
    )) @_nim-unittest
  (#set! tag nim-unittest))

; single test
(source_file ((call
    function: (identifier) @run (#eq? @run "test")
    (argument_list
        ((interpreted_string_literal
               . (string_content) @nim_test)
        )
        ":"
    )) @_nim-unittest
  (#set! tag nim-unittest-single)))

; test in the suite
(
  (call
    function: (identifier) @_suite_name (#eq? @_suite_name "suite")
    (argument_list
      ((interpreted_string_literal
         . (string_content) @nim_suite)
      )
      ":"
      ; .
      (statement_list
        (_)?
        (call
          function: (identifier) @run (#eq? @run "test")
          (argument_list
            ((interpreted_string_literal
               . (string_content) @nim_test)
            )
            ":"
          )
        )
      )
    )
  ) @_nim-unittest
  (#set! tag nim-unittest-suite)
)
