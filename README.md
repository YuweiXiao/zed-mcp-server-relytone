# RelytONE MCP Server for Zed

This extension integrates Zed's Model Context Protocol with RelytONE's MCP server. It automatically manages the `@relyt/mcp-server-relytone` npm package, ensuring you always run the latest version without manual updates.

## Configuration

After enabling the extension, you'll be prompted to configure your RelytONE bearer token through Zed's built-in extension settings UI. The extension will handle all package management automatically.

Alternatively, you can manually add the configuration to your Zed settings (Command Palette → "Open Settings" → `settings.json`):

```jsonc
{
  "context_servers": {
    "mcp-server-relytone": {
      "settings": {
        "relytone_bearer_token": "your-bearer-token-here"
      }
    }
  }
}
```

## Obtaining a Bearer Token

1. Sign in at https://relytone.data.cloud
2. Open your RelytONE account dashboard
3. Generate a bearer token
4. Paste it into the extension settings when prompted

The token will be securely stored and passed to the MCP server as the `RELYTONE_BEARER_TOKEN` environment variable.
