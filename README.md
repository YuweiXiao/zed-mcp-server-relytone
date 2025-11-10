# RelytONE MCP Server for Zed

This extension wires Zed's Model Context Protocol integration up to RelytONE's hosted MCP server. It mirrors the official `@relyt/mcp-server-relytone` implementation and simply launches it via `npx`, so you always get the latest published build without having to manage binaries yourself.

## Configuration

After enabling the extension, add the following snippet to your Zed settings (Command Palette → "Open Settings" → `settings.json`):

```jsonc
{
  "mcpServers": {
    "relytone": {
      "command": "npx",
      "args": ["--yes", "@relyt/mcp-server-relytone"],
      "env": {
        "RELYTONE_BEARER_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

The extension surfaces an inline settings UI so you can store the `RELYTONE_BEARER_TOKEN` securely inside Zed and avoid hard‑coding it in your user settings file.

## Obtaining a token

Sign in at https://relytone.data.cloud, open your RelytONE dashboard, and generate a bearer token. Keep it handy—you'll paste it into the extension settings the first time you connect.
