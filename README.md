# HWID Spoofer

![Rust](https://img.shields.io/badge/Rust-1.81.0-orange?style=flat-square&logo=rust)  
![License](https://img.shields.io/badge/license-MIT-green)
![GitHub stars](https://img.shields.io/github/stars/havokzero/HWID_Spoofer?style=flat-square)
![GitHub forks](https://img.shields.io/github/forks/havokzero/HWID_Spoofer?style=flat-square)
![GitHub last commit](https://img.shields.io/github/last-commit/havokzero/HWID_Spoofer?style=flat-square)

## 🚀 Overview

**HWID Spoofer** is a Rust-based tool that enables you to spoof system identifiers such as installation ID, PC name, MAC address, hardware profile GUID, hostname, and machine GUID. It's perfect for research, testing, and more, offering a simple yet powerful command-line interface.

> **Disclaimer**: This project is intended for **educational purposes only**. Use it responsibly and in compliance with applicable laws.

## ✨ Features

- **Spoof Installation ID**: Changes the system's installation ID in the registry.
- **Spoof PC Name**: Randomizes and updates the computer name in the registry.
- **Spoof MAC Address**: Generates and applies a new MAC address using `netsh`.
- **Spoof GUID**: Updates the hardware profile GUID in the registry.
- **Spoof Hostname**: Renames the system's hostname using `wmic`.
- **Spoof Machine GUID**: Replaces the machine GUID in the registry.
- **Clean Traces**: Deletes temporary files and other traces from the system.
- **Admin Privileges**: Automatically requests administrator rights when needed.

## 🛠️ Installation

### Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install).
- **Git**: [Install Git](https://git-scm.com).

### Build Instructions

1. Clone the repository:

    ```bash
    git clone https://github.com/havokzero/HWID_Spoofer.git  
    cd HWID_Spoofer
    ```

2. Build the project in release mode:

    ```bash
    cargo build --release
    ```

3. The compiled executable will be in the `target/release` directory:

    ```bash
    cd target/release
    ```

4. Run the executable with administrator privileges:

    ```bash
    ./HWID_Spoofer.exe
    ```

---

## 💻 Usage

HWID Spoofer provides a simple text-based menu interface for interacting with the spoofing options.

Upon running the program, you'll see:

```
=== HWID Spoofer ===

  Select an option:
  [1] Spoof Installation ID
  [2] Spoof PC Name
  [3] Spoof MAC Address
  [4] Spoof GUID
  [5] Spoof Hostname
  [6] Spoof Machine GUID
  [7] Clean Traces
  [exit] Exit
```

Simply choose the number of the action you'd like to perform.

---

### Example

To spoof the Installation ID, simply type `1` and press `Enter`. HWID Spoofer will handle the rest and display a success message.

---

## 🧰 Menu Options

- **[1] Spoof Installation ID**: Changes the system's installation ID in the registry.
- **[2] Spoof PC Name**: Randomizes and updates the computer name in the registry.
- **[3] Spoof MAC Address**: Generates and applies a new MAC address.
- **[4] Spoof GUID**: Spoofs the hardware profile GUID.
- **[5] Spoof Hostname**: Renames the system's hostname.
- **[6] Spoof Machine GUID**: Replaces the machine GUID in the registry.
- **[7] Clean Traces**: Deletes temporary files from common directories.
- **[exit] Exit**: Exits the program.

---

## 🤝 Contributing

If you'd like to contribute to this project, feel free to open an issue or submit a pull request.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
