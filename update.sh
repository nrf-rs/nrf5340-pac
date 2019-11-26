#!/usr/bin/env bash
set -x
set -e

function generate {
    (
        cd "$1"
        rm -rf src
        mkdir src
        svd2rust -i "../$2"
        form -i lib.rs -o src
        rm lib.rs

        rustfmt build.rs
    )
}

generate nrf5340-app-pac nrf5340_application.svd
generate nrf5340-net-pac nrf5340_network.svd

cargo fmt --all
