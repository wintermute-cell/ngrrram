{ pkgs ? import (fetchTarball {
    # pinned to unstable, 2024-05-14
    url = "https://github.com/NixOS/nixpkgs/archive/3281bec7174f679eabf584591e75979a258d8c40.tar.gz";
  }) {}
}:

pkgs.mkShell {
  buildInputs = with pkgs; [ 
    cargo 
    rustc
  ];
}

