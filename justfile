VERSION := `toml get cli/Cargo.toml package.version | jq -r`
export TAG:=`toml get cli/Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

help:
	just --list

test:
	cargo nextest run --no-fail-fast

# Test & watch
test_dev:
	cargo watch -x "test -- --no-capture"

# Test including ignored tests
test_all:
	cargo test -- --include-ignored

# Generate usage samples
usage:
	cargo run -q -- --help | sed -e 's/\x1b\[[0-9;]*m//g' > doc/help.adoc
	cargo run -q -- registry --help | sed -e 's/\x1b\[[0-9;]*m//g' > doc/usage_registry.adoc
	cargo run -q -- system --help | sed -e 's/\x1b\[[0-9;]*m//g' > doc/usage_system.adoc
	cargo run -q -- endpoints --help | sed -e 's/\x1b\[[0-9;]*m//g' > doc/usage_endpoints.adoc
	cargo run -q -- config --help | sed -e 's/\x1b\[[0-9;]*m//g' > doc/usage_config.adoc

# Generate documentation
doc:
	cargo doc -p subrpc -p subrpc-core --all-features --no-deps

# Run rustfmt
fmt:
	cargo +nightly fmt --all

# Run clippy
clippy:
	cargo +nightly clippy --features="v14" --all-targets --tests

deny:
	cargo deny check

# Run checks such as clippy, rustfmt, etc...
check: clippy fmt

# Minor bump, can be used once the release is ready
bump:
	cargo workspaces version --no-git-commit

# Generate the readme as .md
md:
	#!/usr/bin/env bash
	asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md
	cp README.md cli/

release: check test_all bump doc md

tag:
    #!/bin/sh
    echo Tagging version v$TAG
    git tag "v$TAG" -f
    git tag | sort -Vr | head

# Start a local web server to serve the registries located under the registry folder
serve_reg:
	#!/bin/sh
	LOCAL_REG=registry
	ls -al --color $LOCAL_REG
	python -m http.server 9000 --directory $LOCAL_REG --bind 127.0.0.1

reg_reset:
	cargo run -- init
	cargo run -- reg add http://localhost:9000/sample1.json
	cargo run -- reg add http://localhost:9000/sample2.json
	cargo run -- reg up
