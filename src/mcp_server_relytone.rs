use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

#[derive(Debug, Deserialize, JsonSchema)]
struct RelytONEContextServerSettings {
    relytone_bearer_token: String,
}

struct RelytONEModelContextExtension;

impl zed::Extension for RelytONEModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("mcp-server-relytone", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `relytone_bearer_token` setting".into());
        };
        let settings: RelytONEContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: "npx".into(),
            args: vec!["--yes".into(), "@relyt/mcp-server-relytone".into()],
            env: vec![
                (
                    "RELYTONE_BEARER_TOKEN".into(),
                    settings.relytone_bearer_token,
                ),
            ],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(RelytONEContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(RelytONEModelContextExtension);
