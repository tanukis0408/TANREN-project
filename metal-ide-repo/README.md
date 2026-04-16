# Metal IDE (Standalone Repository)

This folder is a standalone-ready repository for the Metal VS Code extension.

## Quick start

```bash
cd metal-ide-repo
code .
# Press F5 to launch Extension Development Host
```

## Publish to VS Code Marketplace

1. Install tooling:
   ```bash
   npm i -g @vscode/vsce
   ```
2. Package extension:
   ```bash
   vsce package
   ```
3. Publish (requires publisher token):
   ```bash
   vsce publish
   ```
