{ pkgs }: {
	deps = [
		pkgs.run
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
    pkgs.rust-analyzer
	];
}