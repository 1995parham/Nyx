default:
    @just --list

# build nyx binary
build:
    @echo '{{ BOLD + CYAN }}Building Nyx!{{ NORMAL }}'
    cargo build

# update rust dependencies
update:
    cargo update

# set up the dev environment with docker-compose
dev cmd *flags:
    #!/usr/bin/env bash
    echo '{{ BOLD + YELLOW }}Development environment based on docker-compose{{ NORMAL }}'
    set -eu
    set -o pipefail
    if [ {{ cmd }} = 'down' ]; then
      docker compose -f ./docker-compose.yml down --volumes --remove-orphans
    elif [ {{ cmd }} = 'up' ]; then
      docker compose -f ./docker-compose.yml up --wait -d {{ flags }}
    else
      docker compose -f ./docker-compose.yml {{ cmd }} {{ flags }}
    fi

# run tests in the dev environment
test: migrate
    cargo test

migrate: (dev "up")

# connect into the dev environment database
database: (dev "up") (dev "exec" "postgres psql -U nyx_user nyx_db")

lint:
    cargo fmt
    cargo clippy
