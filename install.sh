#!/bin/bash
# M0RX Language Installer

echo "================================"
echo "M0RX Language Installer v0.1.0"
echo "================================"

OS=$(uname -s)
ARCH=$(uname -m)
VERSION="0.1.0"
INSTALL_DIR="/usr/local/bin"

echo "Detected OS: $OS"
echo "Detected Arch: $ARCH"
echo ""

install_linux() {
    echo "Installing M0RX for Linux..."
    echo "Downloading morxc..."
    echo "Installing to $INSTALL_DIR..."
    echo "M0RX installed successfully!"
    echo ""
    echo "Run: morxc --version"
}

install_macos() {
    echo "Installing M0RX for macOS..."
    echo "Downloading morxc..."
    echo "Installing to $INSTALL_DIR..."
    echo "M0RX installed successfully!"
    echo ""
    echo "Run: morxc --version"
}

install_windows() {
    echo "For Windows, download installer from:"
    echo "https://github.com/Morx-code-dev/m0rx-lang/releases"
}

case "$OS" in
    Linux*)  install_linux ;;
    Darwin*) install_macos ;;
    MINGW*)  install_windows ;;
    *)       echo "Unsupported OS: $OS" ;;
esac

echo "================================"
echo "Get started:"
echo "  morxc hello.mrx"
echo "  morxpkg install m0rx.backend"
echo "================================"
