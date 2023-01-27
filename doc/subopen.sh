#!/usr/bin/env bash

function subopen() {
    chains=$(subrpc reg chains)
    chain=$(echo $chains | sort -r | fzf -1 --prompt="Select the chain to open in your browser > ")
    subrpc endpoints open $chain
}
