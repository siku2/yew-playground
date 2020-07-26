# COMBINED

# Run the server and watch for changes in both the frontend and the server.
watch:
    #!/usr/bin/env sh
    just watch-frontend &
    just watch-server &

    wait

# Build the frontend and then run the server.
@run:
    just build-frontend
    just run-server

# Build both the frontend and the server.
@build:
    just build-frontend
    just _print_line
    just build-server


# FRONTEND

# Build the frontend.
build-frontend out_dir="www":
    @just _info "building frontend"
    wasm-pack build --dev --no-typescript \
        --out-dir "$(pwd)/{{out_dir}}" \
        --out-name app \
        --target web \
        frontend
    @cd "{{out_dir}}" && rm .gitignore package.json
    @just _info "adding static files"
    cp -r frontend/static/* "{{out_dir}}"
    @just _info "DONE"

# Build the frontend and watch for changes.
watch-frontend:
    @just _watchexec frontend "just build-frontend"


# SERVER

# Compile the server.
build-server:
    @just _info "building server"
    cargo build --package server

# Start the server.
# This assumes that the frontend and the docker images have already been built.
run-server:
    @just _info "starting the server"
    @just docker/check
    cargo run --package server

# Start the server and update on changes.
watch-server:
    @just _watchexec server "just run-server"


# HELPERS

_assert_crate_installed crate:
    #!/usr/bin/env sh
    if ! [ -x "$(command -v {{crate}})" ]; then
        just _error "'{{crate}}' isn't installed. Please run $(just _fmt_cmd "cargo install {{crate}}") to install it"
    fi

_watchexec path command:
    @just _assert_crate_installed watchexec
    watchexec --clear --restart \
        --exts css,ftl,html,rs,toml \
        --watch "{{path}}" \
        "{{command}}"


# FORMATTING

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
