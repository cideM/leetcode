let
  sources = import ./nix/sources.nix;

  pkgs = import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };

in
pkgs.mkShell {
  buildInputs = [
    pkgs.rust-bin.stable.latest.default
    pkgs.rust-analyzer
  ];
}
