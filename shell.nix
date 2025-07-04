# Copyright lowRISC contributors.
#
# SPDX-License-Identifier: MIT OR Apache-2.0

{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    reuse
    rustup
  ];
}
