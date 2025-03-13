# Zed Nim
Nim extension for Zed Editor
<img align="right" width="500" src="https://github.com/user-attachments/assets/0674bec5-0e43-4756-bc38-3cd82d91316b">

### Features
* Syntax highlighting
* Code completion
* Runnable buttons
* Inlay hints
* Tasks
* Diagnostics
* Go to definition and references
* Hover
* Outline
* Code actions (restart nimsuggest)
<br clear="right"/>


## Install

### Prerequisites
By default this extension uses [nimlangserver](https://github.com/nim-lang/langserver). Alternatively you can configure it to use [nimlsp](https://github.com/PMunch/nimlsp) (see below), though support for it is limited. Make sure to install one of the language servers.

### Editor
Nim support has been officially integrated into the Zed editor through this extension. To enable it, search for "Nim" in the Zed extensions and install it.

<img width="400" alt="Extensions" src="https://github.com/user-attachments/assets/25f3ce66-ed75-4d1b-8b89-9f5b10f94834" />


### Dev / Manual install
Download the source and follow the instructions in the [Zed documentation](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally).

## Configuration

### LSP Parameters

To change settings of the nimlangserver add the following to your settings. List of options can be found in nimlangserver's [description](https://github.com/nim-lang/langserver?tab=readme-ov-file#configuration-options).

```json
  "lsp": {
    "nim": {
      "settings": {
        // Add your settings here:
        "timeout": 10000,
        "projectMapping": [
          {
            "projectFile": "tests/all\\.nim",
            "fileRegex": "tests/.*\\.nim"
          }
        ],
        "maxNimsuggestProcesses": 1,
        "notificationVerbosity": "warning"
      }
    }
  }
```

### Switching LSPs

Zed allows to specify the binary to use as a language server (such as nimlsp):

```json
  "lsp": {
    "nim": {
      "binary": {
        "path": "/your/path/to/.nimble/bin/nimlsp",
        "args": []
      },
      ...
    }
  }
```


### Formatting
To use [nph](https://github.com/arnetheduck/nph) as a formatter, add this to your settings:

```json
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

### Jupyter

Zed provides support for [notebooks as scripts](https://zed.dev/docs/repl). To use [jupyternim](https://github.com/stisa/jupyternim) as a kernel, add it in your settings:

```json
  "jupyter": {
    "kernel_selections": {
      "nim": "jupyternimspec"
    }
  }
```


## Screenshots

| <img width="540" alt="Diagnostics" src="https://github.com/user-attachments/assets/f81eeb39-f530-48e3-a4e7-e6af3babe412" /> | <img width="360" alt="Inlay hints" src="https://github.com/user-attachments/assets/d21f083c-3332-4e5b-ad9a-e5f439606624" /> |
|:---:|:---:|
| Diagnostics | Inlay Hints |

| <img width="500" alt="Tasks" src="https://github.com/user-attachments/assets/4f802c5f-a710-4ed0-87a3-9b71795e2f85" /> | <img width="400" alt="Buttons" src="https://github.com/user-attachments/assets/b8afe1a4-8ca1-4bcd-83fa-bdb258577e01" /> |
 |:---:|:---:|
| Tasks | Runnables |
