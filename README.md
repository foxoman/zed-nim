# zed-nim
Nim extension for Zed Editor

- Tree-Sitter Support: https://github.com/alaviss/tree-sitter-nim
- Language Server Support: https://github.com/nim-lang/langserver

# Install

## Prerequisites
Make sure to install [nimlangserver](https://github.com/nim-lang/langserver) before installing this extension

## Editor
Nim support has been officially integrated into the Zed editor through this extension. To enable it, search for "Nim" in the Zed extensions and install it.
![image](https://github.com/user-attachments/assets/11009e16-1c61-49a7-a9eb-77d3aaeb9724)


## For Dev / Manual install

- Download the release source
- Extract it into a folder
- Go to the extension page in Zed and click on Install Dev Extension
![image](https://github.com/user-attachments/assets/5ce69399-e1b2-4b5c-8e9e-93a952c9977d)
- Wait for a time till it builds and you will see nim extension installed
![image](https://github.com/user-attachments/assets/88d7622c-f799-40d9-8c0a-1d99d2529197)




# Screenshot
![image](https://github.com/user-attachments/assets/96cd2df6-11ff-495b-b97f-787ea9b08dc2)

## Inlay hints
![image](https://github.com/user-attachments/assets/ae2e1a14-f923-4b8b-b1b5-1515b9e55697)

## Tasks
Support for current file run, nimble run, nimble build, nimble test

![image](https://github.com/user-attachments/assets/7ae1b533-daef-4ecd-8896-1cbf663ba22a)

## Lsp Error
![image](https://github.com/user-attachments/assets/04bd10b2-d531-4c22-83d3-d570c85d0eda)

![image](https://github.com/user-attachments/assets/e8b3d664-8753-43f6-b690-6140471a2a17)

## Other supported features:
- Go to definition and type definition
- Rename
- Hover
- Outline

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

# Language Server Configuration

To change settings of the nimlangserver add the following to your settings. List of options can be found in nimlangserver's [description](https://github.com/nim-lang/langserver?tab=readme-ov-file#configuration-options).

```
  "lsp": {
    "nim": {
      "settings": {
        // Add your settings here:
        "timeout": 10000,
        "projectMapping": [
          {
            "projectFile": "tests/all\.nim",
            "fileRegex": "tests/.*\.nim"
          }
        ],
        "maxNimsuggestProcesses": 1,
        "notificationVerbosity": "warning"
      }
    }
  }
```

## To-Do List for Project:
- [ ] Fix Project Config options
- [x] Add runnable functionality
- [ ] Implement Debug Task
- [ ] Develop Macros
- [ ] Create code snippets
