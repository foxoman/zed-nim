# zed-nim
Nim support for Zed Editor

- Tree-Sitter Support: https://github.com/alaviss/tree-sitter-nim
- Language Server Support: https://github.com/nim-lang/langserver

# Formating Configuration

To use nph as a formatter, add this to your settings 

```
  "languages": {
    "Nim": {
      "formatter": {
        "external": {
          "command": "nph",
          "arguments": ["-"]
        }
      }
    }
  }
```

# Need to fix:
- Auto indent
- inlay hint
- run and build task
- some syntax coloring issues
- Config options 
