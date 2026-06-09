#!/usr/bin/env bash
set -euo pipefail; readonly IFS=$'\n\t'
# set -x  # debug

# Starts a new container based on the DEV image or attaches to an already
# running DEV container.
#
# Programs run as non-root-user inside the container for security reasons.
#
# The container mounts the project directory via bind-mount. Only a single
# container (and the host) can access the project files (`:Z` uppercase).
#
# The container uses named volumes for persisting cache files to speed up the
# development process. Multiple containers can access the cache volumes
# simultaneously (`:z` lowercase).

readonly script_dir=$( cd "$( dirname "${BASH_SOURCE[0]:-${(%):-%x}}" )" && pwd )
readonly parent_dir=$(dirname ${script_dir})

# Container user: uid=1000(appuser) gid=1000(appuser) groups=1000(appuser)
readonly uid=1000
readonly gid=1000


# Use --publish <host_port>:<container_port> to enable networking.
# Use --rm to remove the container after exit.
# Use --env-file "${parent_dir}/.env" to read environment variables from a file.
podman container start --attach --interactive "$(basename ${parent_dir})" 2>/dev/null || podman container run \
    --interactive \
    --name="$(basename ${parent_dir})" \
    --publish 8001:8001 \
    --pull=newer \
    --rm \
    --tty \
    --user ${uid}:${gid} \
    --userns keep-id:uid=${uid},gid=${gid} \
    --volume "${HOME}/.cache/gem:/home/appuser/.cache/gem:z,rw" \
    --volume "${HOME}/.cache/huggingface:/home/appuser/.cache/huggingface:z,rw" \
    --volume "${HOME}/.cache/pip:/home/appuser/.cache/pip:z,rw" \
    --volume "${HOME}/.cache/pre-commit:/home/appuser/.cache/pre-commit:z,rw" \
    --volume "${HOME}/.cache/rattler:/home/appuser/.cache/rattler:z,rw" \
    --volume "${HOME}/.cache/uv:/home/appuser/.cache/uv:z,rw" \
    --volume "${HOME}/.cargo:/home/appuser/.cargo:z,rw" \
    --volume "${HOME}/.pixi:/home/appuser/.pixi:z,rw" \
    --volume "${HOME}/.rustup:/home/appuser/.rustup:z,rw" \
    --volume "${parent_dir}:/home/appuser/$(basename ${parent_dir}):Z,rw" \
    --workdir "/home/appuser/$(basename ${parent_dir})" \
        localhost/"$(basename ${parent_dir})":latest
