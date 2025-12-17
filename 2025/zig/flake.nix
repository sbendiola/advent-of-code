{
  description = "Zig development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    alejandra = {
      url = "github:kamadorueda/alejandra/3.0.0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zig-overlay.url = "github:mitchellh/zig-overlay";
    # Keep in sync with zigVersion below.
    zls-overlay.url = "github:zigtools/zls/0.14.0";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    alejandra,
    ...
  } @ inputs:
    flake-utils.lib.eachSystem (builtins.attrNames inputs.zig-overlay.packages) (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (final: prev: {
            zigpkgs = inputs.zig-overlay.packages.${prev.system};
          })
        ];
      };
      zigVersion = "0.14.0";
      zig = pkgs.zigpkgs.${zigVersion};
      zls = inputs.zls-overlay.packages.${system}.zls.overrideAttrs (old: {
        nativeBuildInputs = [zig];
      });
      alejandra = inputs.alejandra.packages.${system}.default;
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          zig
          zls
          alejandra
        ];

        shellHook = ''
          alejandra --version
          echo 'zls' "$(zls --version)"
          echo 'zig' "$(zig version)"
        '';
      };

      formatter = alejandra;
    });
}
