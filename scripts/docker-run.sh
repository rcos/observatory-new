#!/bin/sh

if !command -v docker 2>/dev/null; then
    echo "Docker is not installed"
    exit
fi

SUDO="sudo"

if id -nG $(whoami) | grep -qw "docker"; then
    SUDO=""
fi

eval "${SUDO} docker run -p 8000:80 --name observatory -v $(pwd)/data/:/var/lib/observatory rcos/observatory"
