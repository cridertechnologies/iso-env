# iso-env

🚀 **iso-env** is a lightweight **project isolation tool** that manages multiple programming environments, ensuring each project uses the correct versions of **Python, Node.js, Flutter, .NET**, and more.

## ✨ Features
- **Automatic environment isolation** per project
- **Supports multiple languages**: Python, Node.js, Flutter, .NET, and more
- **Cross-platform support**: Windows, macOS, and Linux
- **Detects and installs missing tools** (`pyenv`, `nvm`, `fvm`, etc.)
- **Supports new and existing projects**
- **Auto-switching between environments**

---

## 📦 Installation

### **Windows**
```sh
winget install iso-env
```

### **MacOS (Homebrew)**
```sh
brew install iso-env
```

### **Linux (Debian-based)**
```sh
sudo apt install iso-env
```

### Or, build from source:
```sh
git clone https://github.com/cridertechnologies/iso-env.git
cd iso-env
cargo build --release
mv target/release/iso-env /usr/local/bin/
```

---

## ⚡ Usage

### 1️⃣ Create a New Isolated Project
```sh
iso-env create my-project python 3.9.13
```

- Creates `my-project/`
- Sets up a Python virtual environment (venv)
- Installs the correct Python version via `pyenv`
- Writes `.envconfig.json` for project tracking

---

### 2️⃣ Add Isolation to an Existing Project
Run inside the project directory:
```sh
iso-env create . node 18.17.1
```

- Detects the current folder
- Installs and applies the correct Node.js version
- Creates `.envconfig.json`

---

### 3️⃣ Use an Existing Project
```sh
iso-env use my-project
```

- Reads `.envconfig.json`
- Detects language and version
- Switches to the correct environment

---

### 4️⃣ List All Managed Projects
```sh
iso-env list
```

Example Output:
```sh
Managed Projects:
- my-python-project (python 3.9.13)
- my-react-app (node 16.20.2)
- my-flutter-app (flutter 3.16.2)
```

---

## 🌎 Supported Languages

|Language|Isolation Tool|Version Management|
|--------|--------------|------------------|
| Python | `venv`       | `pyenv`          |
| Node.js| `nvm`        | `nvm`            |
| React  | `nvm`        | `npx create-react-app`|
| Angular| `nvm`        | `ng new`         |
| Flutter| `fvm`        | `fvm`            |
| .NET   | `.NET SDK`   | `global.json`    |

---

## 🔧 How It Works
1. Automatically Detects & Installs Tools
 - If `pyenv`, `nvm`, or `fvm` is missing, `iso-env` installs is automatically based on the OS.
2. Cross-platform Compatibility
 - Windows → Uses `winget install`
 - MacOS → Uses `brew install`
 - Linux → Uses `apt install -y`
3. Dynamic Language Management
 - No hardcoded checks!
 - Easily extendable via `/languages` folder.

---

## 🛠️ Contributing
We welcome contributions! To add support for a new language:

 1. Fork the repo & create a new branch.
 2. Add a file in `src/languages/` (e.g., `ruby.rs`).
 3. Implement the `LanguageManager` trait.
 4. Register it in `src/languages/mod.rs`.
 5. Submit a PR!

---

## 📝 License
MIT License. See `LICENSE` for details.

---

## 🎯 TODO / Future Improvements

 - [ ] Add project templates for common stacks
 - [ ] Add support for additional languages