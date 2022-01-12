{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	buildInputs = with pkgs; [
        cargo
        binutils
        openssl
        git
        neovim
    ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    shellHook = ''
        echo enterting dev env
        alias la="ls -la"
        export PS1="\e[0;32m[\u@\h \W]\$ \e[0m (nix-develop)"
    '';
}
