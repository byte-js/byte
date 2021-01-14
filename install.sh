#!/bin/sh
# Copyright 2019 the Byte authors. All rights reserved. MIT license.
# TODO(everyone): Keep this script simple and easily auditable.

set -e

if ! command -v unzip >/dev/null; then
	echo "Error: unzip is required to install Byte (see: https://github.com/byte-js/byte)." 1>&2
	exit 1
fi

byte_uri="https://github.com/byte-js/byte/releases/download/0.0.2/byte.zip"

byte_install="${BYTE_INSTALL:-$HOME/.byte}"
bin_dir="$byte_install/bin"
exe="$bin_dir/byte"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.zip" "$byte_uri"
unzip -d "$bin_dir" -o "$exe.zip"
chmod +x "$exe"
rm "$exe.zip"
echo "export PATH=$HOME/.byte/bin:$PATH" >>$HOME/.bash_profile
echo "export PATH=$HOME/.byte/bin:$PATH" >>$HOME/.zshrc
echo "export PATH=$HOME/.byte/bin:$PATH" >>$HOME/.bashrc

echo "Byte was installed successfully to $exe"
if command -v byte >/dev/null; then
	echo "Run 'byte help' to get started"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bash_profile" ;;
	esac
	echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
	echo "  export BYTE_INSTALL=\"$byte_install\""
	echo "  export PATH=\"\$BYTE_INSTALL/bin:\$PATH\""
	echo "Run '$exe --help' to get started"
fi