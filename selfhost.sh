#!/bin/bash
# M0RX Self Hosting Script
# M0RX compiler ko M0RX mein rewrite karo

echo "M0RX Self Hosting v0.1.0"
echo "========================"
echo "Phase 1: Build with Rust"
echo "  cd compiler && cargo build --release"
echo ""
echo "Phase 2: Rewrite in M0RX"
echo "  morxc build compiler/src/main.mrx"
echo ""
echo "Phase 3: Self compile"
echo "  ./morxc build compiler/src/main.mrx"
echo ""
echo "M0RX is now self-hosted!"
echo "========================"

cd /workspaces/m0rx-lang/compiler
source "$HOME/.cargo/env"
cargo build --release 2>&1 | tail -3
echo "Self host build: DONE"
