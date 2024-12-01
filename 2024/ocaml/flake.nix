{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        ocaml = pkgs.ocaml;
        ocamlPackages = pkgs.ocamlPackages;
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              #Add here dependencies for the project.
              ocaml

              dune_3 # Build system for OCaml projects
              opam # OCaml package manager
              ocamlPackages.utop # OCaml CLI
              ocamlPackages.ocamlformat # OCaml formatter
              ocamlPackages.ocaml-lsp # OCaml Language server protocol
              ocamlPackages.findlib # OCaml library manager
            ];
          };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
