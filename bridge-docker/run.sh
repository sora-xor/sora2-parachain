#!/bin/bash
set -e

docker(){
    if ! command -v docker &> /dev/null; then
        echo "Docker isn't installed. Installing..."
        curl -fsSL https://get.docker.com -o get-docker.sh
        sh get-docker.sh
    fi
    if ! command -v docker-compose &> /dev/null; then
        echo "Docker Compose isn't installed. Installing..."
        curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
        chmod +x /usr/local/bin/docker-compose
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
    sudo rm -rf nodes-data
}

log() {
    docker-compose logs --tail 20 -f
}


if [ "$(type -t $1)" = "function" ]; then
    "$1"
else
    echo "Func '$1' is not exists"
fi