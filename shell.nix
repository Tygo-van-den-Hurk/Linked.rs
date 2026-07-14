with import <nixpkgs> {};
mkShell {
  buildInputs = [
    rust-analyzer
    rustfmt
    rustc
    cargo
  ];
}
