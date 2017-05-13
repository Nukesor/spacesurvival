#!/bin/bash

mkdir -p static
source ./local_env.sh
elm-package install
npm install
watchexec --watch server --watch migrations --restart "./reset.sh && RUST_BACKTRACE=1 cargo run" &
watchexec --watch server --restart "source ./local_env.sh && cargo doc --lib --no-deps" &
watchexec --watch client --restart "elm-make client/Main.elm --output static/main.js" &
watchexec --watch assets "cp -r assets/* static/" &
watchexec --watch client/Styles "node_modules/.bin/elm-css --output static/css client/Stylesheets.elm"
