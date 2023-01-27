#!/usr/bin/env bash

# A function to interactively open a chain in your Browser
# Simply call `sub` or `sub <pattern>`
function sub() {
    chains=$(subrpc reg chains)

    if [ ! -z "$1" ]; then
        query="$1"
        echo "Searching for chains matching: $query"
        chain=$(echo "$chains" | sort -r | fzf -1 -q "$query" --prompt="Select the chain to open in your browser > ")
    else
        chain=$(echo "$chains" | sort -r | fzf -1 --prompt="Select the chain to open in your browser > ")
    fi
    subrpc endpoints open "$chain"
}
