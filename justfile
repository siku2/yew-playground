build-client:
    @just _info "building client"
    wasm-pack build --dev --target web --no-typescript --out-dir build --out-name app client
    @cd client/build && rm .gitignore package.json
    @just _info "adding static files"
    cp -r client/static/* client/build/
    @just _info "DONE"

build-server:
    @just _info "building server"
    cargo build --package server

# Build both the client and the server
@build:
    just build-client
    just _print_line
    just build-server

serve-client ip="127.0.0.1" port="8000":
    @just _info "serving client"
    @just _assert_crate_installed simple-http-server
    simple-http-server --index --nocache --ip "{{ip}}" --port "{{port}}" --try-file client/build/index.html -- client/build

run-client:
    @just build-client
    @just serve-client

watch-client:
    #!/usr/bin/env sh
    just _watchexec client "just build-client" &
    just serve-client &

    wait

run-server:
    @just _info "starting the server"
    cargo run --package server

watch-server:
    @just _watchexec server "just run-server"

# Print some basic instructions
@help:
    just _print_line
    echo "YEW SANDBOX"
    echo "-----------"
    echo "To get started, run $(just _fmt_cmd "just watch-server") in one console and $(just _fmt_cmd "just watch-client") in another."
    echo "Use the link printed by the watch-client console to open the local website."
    just _print_line

# Helper functions

_assert_crate_installed crate:
    #!/usr/bin/env sh
    if ! [ -x "$(command -v {{crate}})" ]; then
        just _error "'{{crate}}' isn't installed. Please run $(just _fmt_cmd "cargo install {{crate}}") to install it"
    fi

_watchexec path command:
    @just _assert_crate_installed watchexec
    watchexec --clear --restart --exts css,ftl,html,rs,toml --ignore build --watch "{{path}}" "{{command}}"

# Formatting helpers

_print_line:
    #!/usr/bin/env sh
    cols=$(tput cols)
    echo -e "\n"
    printf "%${cols:=42}s" | tr " " "="
    echo -e "\n"

@_info message:
    echo -e "{{_ansi_green}}{{_ansi_bold}}INFO:{{_ansi_reset_all}} {{message}}"
@_warning message:
    echo -e "{{_ansi_yellow}}{{_ansi_bold}}WARNING:{{_ansi_reset_all}} {{message}}"
@_error message code="1":
    echo ""
    echo -e "{{_ansi_red}}{{_ansi_bold}}ERROR:{{_ansi_reset_all}} {{message}}"
    just _print_line
    exit "{{code}}"

@_fmt_cmd command:
    echo -e "{{_ansi_blue}}{{_ansi_bold}}'{{command}}'{{_ansi_reset_all}}"

# ansi set
_ansi_bold := "\\e[1m"

# ansi reset
_ansi_reset_all := "\\e[0m"

# ansi colors
_ansi_red := "\\e[31m"
_ansi_green := "\\e[32m"
_ansi_yellow := "\\e[33m"
_ansi_blue := "\\e[34m"
