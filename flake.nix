{
  description = "cloudflareupdated packaging and development stuff";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat-ci.url = "github:hercules-ci/flake-compat-ci";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-compat-ci, flake-compat, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        rust = rust-overlay.latest.rustChannels.stable.rust; 
        naersk-lib = naersk.lib."${system}";
        buildEnvVars = {
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in rec {
        # `nix build`
        packages.cloudflareupdated = naersk-lib.buildPackage {
          pname = "cloudflareupdated";
          root = ./.;
          buildInputs = with pkgs; [ openssl pkgconfig ];
        } // buildEnvVars;
        defaultPackage = packages.cloudflareupdated;

        # `nix run`
        apps.cloudflareupdated =
          utils.lib.mkApp { drv = packages.cloudflareupdated; };
        defaultApp = apps.cloudflareupdated;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo zsh ];
          shellHook = ''
            test -f ~/.zshrc && exec zsh
          '';
        };

        ciNix = flake-compat-ci.lib.recurseIntoFlakeWith { flake = self; };

      });
}

