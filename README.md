# Tauri + Sycamore

This template should help get you started developing with Tauri and Sycamore.

# To build project

```
cargo tauri build
```

# To run project

```
cargo tauri dev
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

NOTES: 

MAIN_VIEW:
Start: Application Reads source file for .sheet.txt saved files.
  if none exist display none and only show the create button.
If create button is clicked show CREATE_VIEW.
If sheet item is selected show SHEET_VIEW with data loaded from file
If load new file is selected then open up a files window that lets the user select which file they want to load to the source file

CREATE_VIEW: 
Two text fields
One for class the other for race. The user types the name of the class and race they want the character to be.

SHEET_VIEW:
shows all the character sheet data and also allows the character edit all data and update data from there

