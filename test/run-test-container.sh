#!/bin/sh
podman run --name blocky --rm -v ./blocky-config.yml:/app/config.yml:ro -p 4000:4000 -p 1234:53/udp ghcr.io/0xerr0r/blocky:v0.23
