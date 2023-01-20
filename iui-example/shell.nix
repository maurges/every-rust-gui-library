let pkgs = import <nixpkgs> {};
in pkgs.mkShell {
  name = "iui-example";

  buildInputs = [ pkgs.cmake pkgs.libiconv ];
}
