# HWID Spoofer

![Rust](https://img.shields.io/badge/Rust-1.65.0-orange?style=flat-square&logo=rust)  
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

**HWID Spoofer** is a Rust-based tool designed to spoof various system identifiers such as installation ID, PC name, MAC address, hardware profile GUID, hostname, and machine GUID. It also provides a feature to clean temporary files and traces left behind on the system. This tool is primarily for research and testing purposes, providing an easy-to-use command-line interface.

> **Disclaimer**: This project is intended for educational purposes only. Please use it responsibly and with full respect for the law and ethical guidelines.

## Features

- **Spoof Installation ID**: Changes the system's installation ID in the registry.
- **Spoof PC Name**: Randomizes and updates the computer name in the registry.
- **Spoof MAC Address**: Generates and applies a new MAC address to the network interface using `netsh`.
- **Spoof GUID**: Updates the hardware profile GUID in the registry.
- **Spoof Hostname**: Renames the system's hostname using `wmic`.
- **Spoof Machine GUID**: Replaces the machine GUID in the registry.
- **Clean Traces**: Deletes temporary files from common system directories.
- **Admin Privileges**: Automatically requests elevation to administrator when needed.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Menu Options](#menu-options)
- [Contributing](#contributing)
- [License](#license)

## Installation

### Prerequisites

Ensure you have the following installed:

- **Rust**: You can install Rust by visiting [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Git**: Clone the repository with Git by visiting [git-scm.com](https://git-scm.com).

### Building the Project

1. Clone this repository:

```bash
git clone https://github.com/havokzero/HWID_Spoofer.git  
cd HWID_Spoofer
```
2. Build the project in release mode:
```ps
cargo build --release
```
3. The compiled executable can be found in the target/release directory:
```ps
cd target/release
```
4. Run the executable (ensure it is launched with administrator privileges):
```ps
./HWID_Spoofer.exe
```
---

## Usage

The HWID Spoofer provides a simple text-based menu interface for interacting with the spoofing options.

Upon running the program, you'll be presented with the following menu:

less

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

Select a number corresponding to the spoofing action you want to perform. The tool will handle the rest.

---
# Example

To spoof the Installation ID, enter 1 at the menu prompt. The tool will spoof the Installation ID and display a success message.
# Menu Options

    [1] Spoof Installation ID: Changes the system's installation ID in the registry.
    [2] Spoof PC Name: Randomizes and updates the computer name in the registry.
    [3] Spoof MAC Address: Generates and applies a new MAC address to the network interface.
    [4] Spoof GUID: Spoofs the hardware profile GUID.
    [5] Spoof Hostname: Renames the system's hostname.
    [6] Spoof Machine GUID: Replaces the machine GUID in the registry.
    [7] Clean Traces: Deletes temporary files from common directories.
    [exit] Exit: Exits the program.
---
# Contributing

If you'd like to contribute to this project, feel free to open an issue or submit a pull request.

    Fork the repository.
    Create your feature branch (git checkout -b feature/your-feature).
    Commit your changes (git commit -am 'Add some feature').
    Push to the branch (git push origin feature/your-feature).
    Open a pull request.
---

# License


This project is licensed under the MIT License - see the LICENSE file for details.

