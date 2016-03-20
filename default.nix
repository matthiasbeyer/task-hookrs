{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustStable; [
    rustc
    cargo
    pkgs.llvmPackages.lldb
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "task-hookrs";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env ];

}

