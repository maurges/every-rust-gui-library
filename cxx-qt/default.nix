{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation rec {
  pname = "cxx-qt-example";
  version = "1.0.0";

  src = ./.;
  nativeBuildInputs = [ pkgs.cmake pkgs.corrosion pkgs.qt5.wrapQtAppsHook pkgs.cargo ];
  buildInputs = [
    pkgs.qt5.qtbase
    pkgs.qt5.qtdeclarative
    pkgs.qt5.qtquickcontrols2

    pkgs.rustc
    pkgs.cargo
  ];
}
