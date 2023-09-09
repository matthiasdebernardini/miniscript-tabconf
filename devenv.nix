{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.git ];


  enterShell = ''
    git --version
  '';

  languages.python.enable = true;
  languages.python.venv.enable = true;

}
