{
  description = "rosetta — universal data decoder for the command line";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        rosetta = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "rosetta";
          version = "0.1.0";
          cargoExtraArgs = "--locked";
          meta = with pkgs.lib; {
            description = "A universal data decoder for the command line";
            license = licenses.mit;
            mainProgram = "rosetta";
          };
          postInstall = ''
            mkdir -p $out/share/man/man1
            find target -path '*/out/man1/*.1' -exec install -Dm444 {} $out/share/man/man1/ \; 2>/dev/null || true
          '';
        });
      in {
        packages.default = rosetta;
        packages.rosetta = rosetta;
        devShells.default = craneLib.devShell {
          inputsFrom = [ rosetta ];
          packages = with pkgs; [ cargo-watch cargo-audit ];
        };
        checks.default = rosetta;
      })
    // {
      overlays.default = final: prev: {
        rosetta = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };
    };
}
