#!/usr/bin/env zsh
cargo doc --verbose
mkdir -p docs/_static
mv target/doc docs/_static/rust_api
python3.13 -m venv .venv-3.13
. ./.venv-3.13/bin/activate
python -m pip install --upgrade pip
python -m pip install sphinx myst-parser groundwork-sphinx-theme

cp README.md docs/
sphinx-build -b html docs out/html