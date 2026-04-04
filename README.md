# M0RX Programming Language

> **M**achine **0**ptimized **R**untime e**X**ecution

M0RX is a next-generation compiled programming language built for the AI era.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![LLVM](https://img.shields.io/badge/backend-LLVM%2017-orange)

---

## Why M0RX?

| Feature | M0RX |
|---|---|
| Speed | C-level (LLVM compiled) |
| Safety | Rust-like memory safety |
| Portable | Runs on 15+ platforms |
| Readable | Python-like syntax |
| AI Ready | Built-in AI/ML support |

---

## Quick Start
```mrx
// Hello World
showln("Hello M0RX!")

// Variables
let name: str = "M0RX"
let version: ant = 1
let active: bool = true

// Function
fn greet(name: str) -> nil {
    showln(name)
}

greet("Hello World!")

// AI in one line
use m0rx.ai
let llm: any = llmLoad("llama3")
let reply: str = llmChat(llm, [{"role": "user", "content": "Hi!"}])
showln(reply)
```

---

## Install

**Linux / macOS:**
```bash
curl -sSf https://raw.githubusercontent.com/Morx-code-dev/m0rx-lang/main/install.sh | sh
```

**Windows:**
Download from [Releases](https://github.com/Morx-code-dev/m0rx-lang/releases)

---

## Language Features

### Keywords (85)
`if` `else` `elif` `match` `when` `loop` `while` `each` `break` `skip` `give` `halt` `fn` `class` `trait` `impl` `let` `fix` `bind` `async` `await` `spawn` `try` `catch` `panic` `model` `infer` `embed` `predict` ...

### Data Types (22)
`tiny` `short` `ant` `long` `vast` `utiny` `ushort` `uant` `ulong` `half` `dbl` `precise` `chr` `str` `txt` `bool` `nil` `list` `map` `set` `tensor` `blob`

### Operators (42)
`+` `-` `*` `/` `%` `**` `==` `!=` `>` `<` `>=` `<=` `&&` `||` `!!` `|>` `??` `~>` `::` `->` ...

---

## Standard Libraries (40+)

| Library | Purpose |
|---|---|
| `m0rx.backend` | HTTP, REST, GraphQL, gRPC, WebSocket |
| `m0rx.ui` | 2D/3D graphics, game engine, mobile |
| `m0rx.ai` | LLM, RAG, CV, NLP, ONNX, OpenRouter |
| `m0rx.voice` | TTS, STT, voice cloning |
| `m0rx.db` | SQL, NoSQL, Vector DB |
| `m0rx.sec` | Crypto, JWT, OAuth2, TLS |
| `m0rx.net` | TCP, UDP, HTTP client |
| `m0rx.async` | Async runtime, channels, coroutines |
| `m0rx.cloud` | AWS, GCP, Azure |
| `m0rx.ml` | Neural networks, training, inference |

---

## CLI Tools
```bash
morxc run hello.mrx        # Run a file
morxc build app.mrx        # Compile to binary
morxpkg install m0rx.ai    # Install package
morxfmt myfile.mrx         # Format code
morxlint myfile.mrx        # Lint code
morxtest unit              # Run tests
morxdoc gen .              # Generate docs
```

---

## Editor Support (15+)
VS Code · Neovim · Vim · Emacs · JetBrains · Sublime Text · Helix · Zed · Lapce · Kate · Micro · Nano · Eclipse · Atom · Gedit

---

## Platform Support (15+)
Linux · macOS · Windows · Android · iOS · WebAssembly · AWS · GCP · Azure · Docker · Kubernetes · Raspberry Pi · FreeBSD · Edge · Embedded

---

## License
MIT — Free and open source forever.

---

## Contributing
Pull requests welcome! See [CONTRIBUTING.md](.github/CONTRIBUTING.md)
