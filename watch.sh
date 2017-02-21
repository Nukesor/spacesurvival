#!/bin/bash

mkdir -p static
source ./local_env.sh
elm-package install
watchexec --exts rs,toml,sql --restart "./reset.sh && RUST_BACKTRACE=1 cargo run" &
watchexec --exts elm --restart "elm-make client/Main.elm --output static/main.js" &
watchexec --watch assets "cp -r assets/* static/"
