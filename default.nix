{ stdenv, pkgs, fetchFromGitHub, rustPlatform, openssl, pkg-config, ... }:

rustPlatform.buildRustPackage rec {
  pname = "cloudflareupdated";
  version = "93302fe83e06595583fb7ba6df2e56006df634cd";

  src = "./"
  
  nativeBuildInputs = [ openssl pkg-config ];

  cargoSha256 = "0drf5xnqin26zdyvx9n2zzgahcnrna0y56cphk2pb97qhpakvhbj";
  verifyCargoDeps = true;
  
  preConfigure = ''
    export HOME=$(mktemp -d)
  '';
}