#!/usr/bin/env bash
set -e

nixpkgs="$1"

rm -f /tmp/j /tmp/j2

nix-instantiate ./list-pkgs.nix --eval --strict --json --read-write-mode --arg nixpkgs "$nixpkgs" > /tmp/j
nix-store -q --xml $(cat /tmp/j | jq -r '.[] | .[1]') > /tmp/j2
