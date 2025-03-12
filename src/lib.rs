use zed::settings::LspSettings;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, CompletionKind},
    CodeLabel, CodeLabelSpan, LanguageServerId, Result,
};

struct NimExtension {}

impl zed::Extension for NimExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        // Get configuration from settings
        let binary_settings = LspSettings::for_worktree("nim", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(zed::Command {
                command: path,
                args: binary_args.unwrap_or_default(),
                env: Default::default(),
            });
        }

        // If no setting specified, use the default
        let path = worktree
            .which("nimlangserver")
            .ok_or_else(|| "nim-langserver not installed.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("nim", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(zed::serde_json::json!({
            "nim": settings
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Function
            | CompletionKind::Snippet
            | CompletionKind::Method
            | CompletionKind::Keyword
            | CompletionKind::Property => {
                let detail = completion.detail?;
                let kws = vec!["proc", "template", "iterator", "converter"];
                // Get the first word from the detail string
                let first = detail.split_whitespace().next();
                let is_keyword = first.map_or(false, |word| kws.contains(&word));

                let decl = if is_keyword {
                    format!("{} ", first.unwrap())
                } else {
                    detail.clone()
                };
                let decl_len = decl.len();

                // Parse the function signature from detail
                if is_keyword {
                    // Extract the function signature and return type
                    let signature_end = detail.rfind("{.").unwrap_or(detail.len());
                    let signature = &detail[decl_len..signature_end];

                    // Split into parts: "(): string"
                    let parts: Vec<&str> = signature.rsplitn(2, ')').collect();
                    if parts.len() >= 2 {
                        let params = parts[1].to_string() + ")"; // "():"
                        let return_type = parts[0].trim().to_string(); // "string"

                        let code = format!("{decl}{}{}{}", completion.label, params, return_type);
                        let code_len = code.len();

                        // Templates and iterators are distictly highlighted from procs and converters
                        // TODO: Maybe converters also need to be distinct?
                        if decl.starts_with("template") {
                            return Some(CodeLabel {
                                code,
                                spans: vec![
                                    CodeLabelSpan::literal(
                                        format!("{}", completion.label),
                                        Some("constructor".into()), // Same as macros, but LSP provides params only for templates
                                    ),
                                    CodeLabelSpan::code_range(
                                        decl_len + completion.label.len()
                                            ..decl_len + completion.label.len() + params.len(),
                                    ),
                                    CodeLabelSpan::code_range(
                                        decl_len + completion.label.len() + params.len()..code_len,
                                    ),
                                ],
                                filter_range: (0..completion.label.len()).into(),
                            });
                        } else if decl.starts_with("iterator") {
                            return Some(CodeLabel {
                                code,
                                spans: vec![
                                    CodeLabelSpan::literal(
                                        format!("{}", completion.label),
                                        Some("string.escape".into()), // <- TODO: this is very contraversial, but most themes appear to not have that many colors
                                    ),
                                    CodeLabelSpan::code_range(
                                        decl_len + completion.label.len()
                                            ..decl_len + completion.label.len() + params.len(),
                                    ),
                                    CodeLabelSpan::code_range(
                                        decl_len + completion.label.len() + params.len()..code_len,
                                    ),
                                ],
                                filter_range: (0..completion.label.len()).into(),
                            });
                        }

                        return Some(CodeLabel {
                            code,
                            spans: vec![
                                CodeLabelSpan::code_range(
                                    decl_len..decl_len + completion.label.len(),
                                ),
                                CodeLabelSpan::code_range(
                                    decl_len + completion.label.len()
                                        ..decl_len + completion.label.len() + params.len(),
                                ),
                                CodeLabelSpan::code_range(
                                    decl_len + completion.label.len() + params.len()..code_len,
                                ),
                            ],
                            filter_range: (0..completion.label.len()).into(),
                        });
                    }
                } else {
                    if decl.starts_with("macro") {
                        // Macros
                        let code = format!("macro {}", completion.label);
                        return Some(CodeLabel {
                            code,
                            spans: vec![
                                CodeLabelSpan::literal(
                                    format!("{}", completion.label),
                                    Some("constructor".into()),
                                ),
                                CodeLabelSpan::literal(" macro", Some("keyword".into())),
                            ],
                            filter_range: (0..completion.label.len()).into(),
                        });
                    } else {
                        // Object fields
                        // For some reason both converters and object fields are kind 10 (Property)
                        let code = format!("var {}: {}", completion.label, decl);
                        let code_len = code.len();

                        return Some(CodeLabel {
                            code,
                            spans: vec![CodeLabelSpan::code_range(4..code_len)],
                            filter_range: (0..completion.label.len()).into(),
                        });
                    }
                }

                let code = format!("{decl}{}() = discard", completion.label);
                let code_len = code.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(decl_len..code_len - 10)],
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Class => {
                let detail = completion.detail?;
                if detail.starts_with("type ") {
                    let code = format!("type {} = object", completion.label);

                    return Some(CodeLabel {
                        code,
                        spans: vec![
                            CodeLabelSpan::code_range(5..5 + completion.label.len()),
                            CodeLabelSpan::literal(" type", Some("keyword".into())),
                        ],
                        filter_range: (0..completion.label.len()).into(),
                    });
                }
                None
            }
            CompletionKind::Value => {
                let detail = completion.detail?;
                let delimeter = if detail.starts_with("const") {
                    ":"
                } else {
                    "of"
                };
                let decl_parts: Vec<&str> = detail.rsplitn(2, delimeter).collect();

                if decl_parts.len() >= 2 {
                    let value_type_parts: Vec<&str> =
                        decl_parts[0].trim().rsplitn(2, "literal").collect();
                    let has_literal: bool = value_type_parts.len() >= 2;

                    if detail.starts_with("const") {
                        if has_literal {
                            let value_type = value_type_parts[1].trim();
                            let value_literal = value_type_parts[0]
                                .trim()
                                .strip_prefix('(')?
                                .strip_suffix(')')?;
                            let code = format!("const {}: {}", completion.label, value_type);
                            let code_len = code.len();

                            return Some(CodeLabel {
                                code,
                                spans: vec![
                                    CodeLabelSpan::code_range(6..6 + completion.label.len()),
                                    CodeLabelSpan::literal(": const ", Some("keyword".into())),
                                    CodeLabelSpan::code_range(
                                        6 + completion.label.len() + 2..code_len,
                                    ),
                                    CodeLabelSpan::literal(" = ", Some("operator".into())),
                                    CodeLabelSpan::literal(value_literal, Some("constant".into())),
                                ],

                                filter_range: (0..completion.label.len()).into(),
                            });
                        }
                        let value_type = value_type_parts[0].trim();
                        let code = format!("const {}: {}", completion.label, value_type);
                        let code_len = code.len();

                        return Some(CodeLabel {
                            code,
                            spans: vec![
                                CodeLabelSpan::code_range(6..6 + completion.label.len()),
                                CodeLabelSpan::literal(": const ", Some("keyword".into())),
                                CodeLabelSpan::code_range(6 + completion.label.len() + 2..code_len),
                            ],

                            filter_range: (0..completion.label.len()).into(),
                        });
                    } else {
                        let value_type = value_type_parts[0].trim();
                        let code = format!("let {}: {}", completion.label, value_type);
                        let code_len = code.len();

                        return Some(CodeLabel {
                            code,
                            spans: vec![
                                CodeLabelSpan::code_range(4..4 + completion.label.len()),
                                CodeLabelSpan::literal(": let ", Some("keyword".into())),
                                CodeLabelSpan::code_range(4 + completion.label.len() + 2..code_len),
                            ],
                            filter_range: (0..completion.label.len()).into(),
                        });
                    }
                }
                None
            }
            CompletionKind::Field => {
                let detail = completion.detail?;
                let decl_parts: Vec<&str> = detail.rsplitn(2, "of").collect();

                if decl_parts.len() >= 2 {
                    let field_type = decl_parts[0].trim();
                    let code = format!("var {}: {}", completion.label, field_type);
                    let code_len = code.len();

                    return Some(CodeLabel {
                        code,
                        spans: vec![
                            CodeLabelSpan::code_range(4..4 + completion.label.len()),
                            CodeLabelSpan::literal(": var ", Some("keyword".into())),
                            CodeLabelSpan::code_range(4 + completion.label.len() + 2..code_len),
                        ],
                        filter_range: (0..completion.label.len()).into(),
                    });
                }
                None
            }
            CompletionKind::Variable => {
                let detail = completion.detail?;
                let decl_parts: Vec<&str> = detail.rsplitn(2, "of").collect();

                if decl_parts.len() >= 2 {
                    let field_type_parts: Vec<&str> = decl_parts[0].trim().splitn(2, ':').collect();
                    let field_type = field_type_parts[0].trim();
                    let code = format!(
                        "var a: {}; for {} in []: discard",
                        field_type, completion.label
                    );
                    let code_len = code.len();

                    return Some(CodeLabel {
                        code,
                        spans: vec![
                            CodeLabelSpan::code_range(7 + field_type.len() + 6..code_len - 15),
                            CodeLabelSpan::literal(": ", Some("keyword".into())),
                            CodeLabelSpan::code_range(7..7 + field_type.len()),
                        ],
                        filter_range: (0..completion.label.len()).into(),
                    });
                }
                None
            }
            CompletionKind::Enum => {
                let detail = completion.detail?;
                let enum_type = detail.strip_prefix("enum ").unwrap_or("");
                let code = format!("type {} = enum", enum_type);

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::literal(
                            completion.label.clone(),
                            Some("local.definition.enum".into()),
                        ),
                        CodeLabelSpan::literal(" enum ", Some("keyword".into())),
                        CodeLabelSpan::code_range(5..5 + enum_type.len()),
                    ],
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            _ => None,
        }
    }
}

zed::register_extension!(NimExtension);
