# zed-nim
Nim support for Zed Editor

- Tree-Sitter Support: https://github.com/alaviss/tree-sitter-nim
- Language Server Support: https://github.com/nim-lang/langserver

# Screenshot
![image](https://github.com/user-attachments/assets/96cd2df6-11ff-495b-b97f-787ea9b08dc2)

## Inlay hints
![image](https://github.com/user-attachments/assets/ae2e1a14-f923-4b8b-b1b5-1515b9e55697)

## Tasks
Support for current file run, nimble run, nimble build

![image](https://github.com/user-attachments/assets/7ae1b533-daef-4ecd-8896-1cbf663ba22a)

## Lsp Error
![image](https://github.com/user-attachments/assets/04bd10b2-d531-4c22-83d3-d570c85d0eda)

![image](https://github.com/user-attachments/assets/e8b3d664-8753-43f6-b690-6140471a2a17)



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
- some syntax coloring issues
- Config options 
