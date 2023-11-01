This folder contains symlinks to the `sol!` macro tests in `../sol-types/tests/doctests/*`.
We can't use a directory symlink because Git on Windows doesn't support them, so we use file
symlinks instead.

Run at the root of the repo:
```bash
ln -rs crates/sol-types/tests/doctests/* crates/sol-macro/doctests/
```
