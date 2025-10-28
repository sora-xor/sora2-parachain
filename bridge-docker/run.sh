#!/bin/bash
set -e

docker(){
    # Security hardening: do NOT auto-install Docker or Compose.
    # Implicitly downloading and executing remote installers as root is a supply-chain risk.
    # Instead, fail fast with clear guidance.
    if ! command -v docker &> /dev/null; then
        echo "Error: Docker is not installed or not on PATH." >&2
        echo "Please install Docker from https://docs.docker.com/get-docker/ and retry." >&2
        exit 1
    fi
    if ! command -v docker-compose &> /dev/null; then
        # Support Compose v2 via 'docker compose'
        if docker compose version &>/dev/null; then
            # Define a simple shim for docker-compose -> docker compose
            docker-compose() { docker compose "$@"; }
        else
            echo "Error: Docker Compose is not installed." >&2
            echo "Install Compose v2 (preferred) or v1. See: https://docs.docker.com/compose/" >&2
            exit 1
        fi
    fi
}

build() {
    docker-compose up -d --build
}

up() {
    docker-compose up -d
}

stop() {
    docker-compose down
}

wipe() {
    docker-compose down -v
    sudo rm -rf nodes-data build-spec-data
}

log() {
    docker-compose logs --tail 20 -f
}


if [ "$(type -t $1)" = "function" ]; then
    "$1"
else
    echo "Func '$1' is not exists"
fi
