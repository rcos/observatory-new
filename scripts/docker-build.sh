#!/bin/sh

if !command -v docker 2>/dev/null; then
    echo "Docker is not installed"
    exit
fi

SUDO="sudo"

if id -nG $(whoami) | grep -qw "docker"; then
    SUDO=""
fi

eval "${SUDO} docker build -t rcos/observatory .."
