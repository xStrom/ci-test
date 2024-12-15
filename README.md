# CI Test

Testing various CI strategies here ..

## How to specify additional dependencies

```yml
dependencies: |
  ${{ matrix.target == 'x86_64-unknown-linux-gnu' && '
	sudo apt install ripgrep
	rg --version
  '}}

dependencies: |
  ${{ (matrix.target == 'x86_64-unknown-linux-gnu' && '
	sudo apt install ripgrep
	rg --version
  ') || (matrix.target == 'x86_64-pc-windows-msvc' && '
	choco install ripgrep
	rg --version
  ')}}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
