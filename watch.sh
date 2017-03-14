#!/bin/bash

mkdir -p static
source ./local_env.sh
elm-package install
watchexec --watch server --watch migrations --restart "./reset.sh && RUST_BACKTRACE=1 cargo run" &
watchexec --watch client --restart "elm-make client/Main.elm --output static/main.js" &
watchexec --watch assets "cp -r assets/* static/"
