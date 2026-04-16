# Metal IDE (VS Code-based)

Metal IDE is a VS Code extension package that turns VS Code into a focused IDE for the Metal language.

## What is included

- Metal language registration (`.mt`)
- Syntax highlighting grammar
- Smart brackets/comments/indent behavior
- Starter snippets for functions, loops, maps, and conditionals
- Custom theme **Metal Aurora (Liquid Glass)** inspired by macOS translucency, but with its own visual language

## Design language: Liquid Glass

The interface style combines:

- Deep low-contrast backgrounds
- Soft neon accents for syntax landmarks
- Cool blue hierarchy for UI chrome
- Gentle transparency-like layering
- Low-noise gutters and panels for better focus

## Use locally

1. Open this folder in VS Code:
   - `ide/metal-ide`
2. Install dependencies only if you want to publish/package (optional).
3. Press `F5` in VS Code to launch an Extension Development Host.
4. In that host window:
   - open any `.mt` file,
   - run **Color Theme** and select **Metal Aurora (Liquid Glass)**.

## Next steps

Potential future upgrades:
- Language server (LSP) for diagnostics and autocomplete.
- Formatter integration.
- Runner tasks (`metal check`, `metal run`) as IDE commands.
- Welcome view and custom icon pack.
