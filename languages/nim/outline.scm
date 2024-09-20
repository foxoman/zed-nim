(proc_declaration
    "proc" @context
    name: [
        (identifier) @name
        (accent_quoted
        (identifier) @name)
        (exported_symbol
        (identifier)) @context
        (exported_symbol
        (accent_quoted
            (identifier))) @context
  ]) @item

(method_declaration
  "method" @context
  name: [
    (identifier) @name
    (accent_quoted
      (identifier) @name)
    (exported_symbol
      (identifier)) @context
    (exported_symbol
      (accent_quoted
        (identifier)) @context)
  ]) @item

(func_declaration
  "func" @context
  name: [
    (identifier) @name
    (accent_quoted
      (identifier) @name)
    (exported_symbol
      (identifier)) @context
    (exported_symbol
      (accent_quoted
        (identifier)) @context)
  ]) @item

(iterator_declaration
  "iterator" @context
  name: [
    (identifier) @name
    (accent_quoted
      (identifier) @name)
    (exported_symbol
      (identifier)) @context
    (exported_symbol
      (accent_quoted
        (identifier)) @context)
  ]) @item

(converter_declaration
  "converter" @context
  name: [
    (identifier) @name
    (accent_quoted
      (identifier) @name)
    (exported_symbol
      (identifier)) @context
    (exported_symbol
      (accent_quoted
        (identifier)) @context)
  ]) @item

(template_declaration
  "template" @context
  name: [
    (identifier) @name
    (accent_quoted
      (identifier) @name)
    (exported_symbol
      (identifier)) @context
    (exported_symbol
      (accent_quoted
        (identifier))) @context
  ]) @item

(macro_declaration
  "macro" @context
  name: [
    (identifier) @name
    (accent_quoted
      (identifier) @name)
    (exported_symbol
      (identifier)) @context
    (exported_symbol
      (accent_quoted
        (identifier))) @context
  ]) @item

(const_section
  "const" @context
  (variable_declaration
    (symbol_declaration_list
      (symbol_declaration
        name: [
          (identifier) @name
          (accent_quoted
            (identifier) @name)
          (exported_symbol
            (identifier)) @context
          (exported_symbol
            (accent_quoted
              (identifier))) @context
        ]))) @item
)

(let_section
  "let" @context
  (variable_declaration
    (symbol_declaration_list
      (symbol_declaration
        name: [
          (identifier) @name
          (accent_quoted
            (identifier) @name)
          (exported_symbol
            (identifier)) @context
          (exported_symbol
            (accent_quoted
              (identifier))) @context
        ]))) @item
)

(var_section
  "var" @context
  (variable_declaration
    (symbol_declaration_list
      (symbol_declaration
        name: [
          (identifier) @name
          (accent_quoted
            (identifier) @name)
          (exported_symbol
            (identifier)) @context
          (exported_symbol
            (accent_quoted
              (identifier))) @context
        ]))) @item
)

(type_declaration
  (type_symbol_declaration
    name: [
      (identifier) @name
      (accent_quoted
        (identifier) @name)
      (exported_symbol
        (identifier)) @context
      (exported_symbol
        (accent_quoted
          (identifier))) @context
    ]
  )
) @item

(enum_field_declaration
  (symbol_declaration
    name: [
      (identifier) @name
      (accent_quoted
        (identifier)) @context
    ])) @item

(object_declaration
  (field_declaration_list
    (field_declaration
      (symbol_declaration_list
        (symbol_declaration
          name: [
            (identifier) @name
            (accent_quoted
              (identifier) @name)
            (exported_symbol
              (identifier)) @context
            (exported_symbol
              (accent_quoted
                (identifier))) @context
          ])
      )
    ) @item
  )
)

(documentation_comment) @annotation
(block_documentation_comment) @annotation

(block
  "block" @context
  ((_) @name)?
) @item

(static_statement
  "static" @context
  ":" @item
)

; macro calls (for DSLs)
(
  (
    call
    function: (identifier) @context
    (argument_list
    (_)? @name
    ":"
    )
  ) @item
)

(variant_declaration
  ("case" @context
  (variant_discriminator_declaration
    (symbol_declaration_list
      (symbol_declaration
        _ @name
      ))
  ))) @item

(variant_declaration
  (of_branch
    "of" @context
    values: (expression_list
      (_)
    ) @name
  ) @item
)

(object_declaration
  (field_declaration_list
    (variant_declaration
      (of_branch
        consequence: (field_declaration_list
              (field_declaration
                (symbol_declaration_list
                  (symbol_declaration
                    name: [
                      (identifier) @name
                      (accent_quoted
                        (identifier) @name)
                      (exported_symbol
                        (identifier)) @context
                      (exported_symbol
                        (accent_quoted
                          (identifier))) @context
                    ])
                )
              )
            ) @item
      )
    )
  )
)
