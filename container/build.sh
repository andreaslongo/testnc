#!/usr/bin/env bash
set -euo pipefail; readonly IFS=$'\n\t'
# set -x  # debug

# Builds a new DEV container image
#
# TODO: Update BASE_IMAGE when a new Debian stable gets released.
#
# * https://hub.docker.com/_/debian
# * https://github.com/debuerreotype/docker-debian-artifacts
# * https://github.com/docker-library/official-images

readonly base_image=docker.io/library/debian:trixie-slim
readonly script_dir=$( cd "$( dirname "${BASH_SOURCE[0]:-${(%):-%x}}" )" && pwd )
readonly parent_dir=$(dirname ${script_dir})

# Use --no-cache if you want to build a fresh container.
# Use --target=stageName to stop the build at a specific stage.
# Use --build-arg=arg=value to pass variable values to the Containerfile
# NOTE: `latest` tags do always trigger auto-pull.
# Use --pull=always to do this also for images without `latest` tags.
# NOTE: Named volumes not supported during build, only bind-mounts are supported.
podman image build \
    --build-arg=BASE_IMAGE="${base_image}" \
    --file="${script_dir}/Containerfile" \
    --ignorefile="${script_dir}/Containerignore" \
    --no-cache \
    --pull=always \
    --tag="$(basename ${parent_dir})":"$(date --iso-8601)" \
    --tag="$(basename ${parent_dir})":latest \
    --target=dev \
        "${parent_dir}"
