#!/bin/bash
docker run \
    --volume ~/ownCloud/Projekte/StarEntertainer/rust/se_2:/home/cross/project \
    --volume ~/.cargo/registry:/home/cross/.cargo/registry \
    --volume ~/ownCloud/Projekte/StarEntertainer/rust/se_2/deb_cross:/home/cross/deb-deps \
    rust-pi-cross build \
        --features=ffmpeg_rpi_zero_special
    
