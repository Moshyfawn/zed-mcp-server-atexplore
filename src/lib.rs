use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

#[derive(Deserialize, JsonSchema, Default)]
struct ATExploreSettings {
    server_url: Option<String>,
}

struct ATExploreMcpExtension;

impl zed::Extension for ATExploreMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        if zed::npm_package_installed_version("mcp-remote")?.is_none() {
            zed::npm_install_package("mcp-remote", "latest")?;
        }

        let server_url = ContextServerSettings::for_project("mcp-server-atexplore", project)
            .ok()
            .and_then(|s| s.settings)
            .and_then(|v| serde_json::from_value::<ATExploreSettings>(v).ok())
            .and_then(|s| s.server_url)
            .unwrap_or_else(|| "https://mcp.atexplore.social/mcp".into());

        Ok(Command {
            command: "node_modules/.bin/mcp-remote".into(),
            args: vec![server_url],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!("../configuration/installation_instructions.md").into(),
            default_settings: include_str!("../configuration/default_settings.jsonc").into(),
            settings_schema: serde_json::to_string(&schemars::schema_for!(ATExploreSettings))
                .map_err(|e| e.to_string())?,
        }))
    }
}

zed::register_extension!(ATExploreMcpExtension);
