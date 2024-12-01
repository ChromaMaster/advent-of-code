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

        python = pkgs.python313;
        python_packages = python.withPackages (
          ps: with ps; [
            pip
            pytest
          ]
        );
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.stdenv.cc.cc ];

            buildInputs = [
              #Add here dependencies for the project.
              python
              python_packages

              uv
              ruff
            ];
          };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
