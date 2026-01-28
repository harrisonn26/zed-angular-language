# Zed Angular Language Extension

An alternative to the official Angular language extension for Zed.

Differences:
 - Intended to override Zed's HTML syntax highlighting with Angular syntax highlighting
 - Language server seems to be more reliable than the official extension
 - Does not download the Angular language server on every startup
 - Allows for sytax highlighting alongside other language servers such as html, emmet, tailwind, eslint, etc.
 - Differences in syntax highlighting. This implementation is based on nvim-treesitter's Angular support
 
 This seems to work well when setup, but requires modification of Zed internals.
 
## Features

- Angular Language Server integration
- Angular syntax highlighting on the HTML file type
- Vim text objects for templates ('daf' deletes around a control flow block)

## Installation

1. Uninstall the official Angular extension
2. Ensure HTML extension is installed
3. Clone this repository
4. Install using 'Install Dev Extension' in the Zed extensions pane
5. identify your extension install location (~/Library/Application Support/Zed/extensions/installed on MacOS)
6. Run these commands (update paths as needed):

```
cp -r ~/Library/Application\ Support/Zed/extensions/installed/angular/languages/html ~/Library/Application\ Support/Zed/extensions/installed/html/languages/
```
```
cp ~/Library/Application\ Support/Zed/extensions/installed/angular/grammars/angular.wasm ~/Library/Application\ Support/Zed/extensions/installed/html/grammars/angular.wasm
```
7. Restart Zed

You may be prompted to install the html extension again, ignore the prompt.

## License

MIT License
