use zed::settings::LspSettings;
use zed_extension_api::{self as zed, Result};
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
}

zed::register_extension!(NimExtension);
