{
  description = "rain-float devshell with wasm-pack";

  inputs.rainix.url = "github:rainlanguage/rainix";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, rainix, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = rainix.pkgs.${system};
        baseShell = rainix.devShells.${system}.default;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = baseShell.buildInputs ++ [ pkgs.wasm-pack ];
          shellHook = baseShell.shellHook or "";
        };
      }
    );
}