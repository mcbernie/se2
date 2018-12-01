## Cross compile for Raspberry Pi

## Cross compiling your project with the image from the dockerhub

You need to pull the image first from the dockerhub (assuming you have docker installed):

```
docker pull ragnaroek/rust-raspberry:<version>
```
where `<version>` is the Rust compiler version. The docker images are provided starting from
version 1.12.0.

If you successfully pulled the `Docker image` containing the cross compiler, you can cross compile your project:
```
$ docker run \
    --volume <path to your rust project directory>:/home/cross/project \
    --volume <path to directory containing the platform dependencies>:/home/cross/deb-deps \ # optional, see section "Platform dependencies"
    --volume <path to local cargo registry, e.g. ~/.cargo/registry>:/home/cross/.cargo/registry \ # optional, using cached registry avoids updating on every build
    ragnaroek/rust-raspberry:<version>
    <cargo command> # e.g. "build --release"
```

The compiled project can then be found in your `target` directory.


## Building your own Docker image
```
$ git clone https://github.com/Ragnaroek/rust-on-raspberry-docker
$ cd rust-on-raspberry-pi/docker
$ docker build \
    --build-arg PI_TOOLS_GIT_REF=<branch/tag/commit> \ # defaults to "master"
    --build-arg RUST_VERSION=<rustup version stable/beta/nightly> \ # defaults to "stable"
    --tag <tag for your docker image> \ # e.g. "rust-nightly-pi-cross"
    .
```

## Example for build
```
docker run --volume ~/ownCloud/Projekte/StarEntertainer/rust/se_2:/home/cross/project --volume ~/.cargo/registry:/home/cross/.cargo/registry --volume ~/ownCloud/Projekte/StarEntertainer/rust/se_2/deb_cross:/home/cross/deb-deps ragnaroek/rust-raspberry:1.30.1 build --release
```

## Some notes
special feature ffmpeg_rpi_zero_special do all required stuff to avoid errors on rpi-ffmpeg lib


## How to get all deb files for an package...
mkdir temp
cd temp
for i in $(apt-cache depends libsdl2-2.0-0 | grep -E 'Hängt ab von|Recommends|Suggests' | cut -d ':' -f 2,3 | sed -e s/'<'/''/ -e s/'>'/''/); do sudo apt-get download $i 2>downloads.txt; done

### dev files 
for i in $(apt-cache depends libsdl2-2.0-0 | grep -E 'Depends|Recommends|Suggests' | cut -d ':' -f 2,3 | sed -e s/'<'/''/ -e s/'>'/''/); do sudo apt-get download $i 2>>errors.txt; done


### dev debs for sdl2
libasound2-dev libc6-dev libpulse-dev libsndio-dev libwayland-dev libx11-dev libxcursor-dev libxxf86vm-dev libxss-dev libxrandr-dev libxkbcommon-dev libxinerama-dev libxi-dev libxext-dev

libxxf86vm1 libasound2 libc6 libpulse0 libsndio6.1 libwayland-client0 libwayland-cursor0 libwayland-egl1-mesa libwayland-egl1 libx11-6 libxcursor1 libxext6 libxi6 libxinerama1 libxkbcommon0 libxrandr2 libxss1

libdbus-1-3 libdbus-1-dev libbsd0 libbsd-dev libxcb1 libxcb1-dev libxrender-dev libxrender1 libffi-dev libffi6 libx11-xcb-dev libx11-xcb1 libice-dev libice6 libsm-dev libsm6 libxtst-dev libxtst6 libwrap0-dev libwrap0 libsndfile1-dev libsndfile1 libasyncns-dev libasyncns0

libcap2 libcap-dev libxfixes-dev libxfixes3 libsystemd-dev libsystemd0 libxau-dev libxau6 libxdmcp-dev libxdmcp6 libuuid1 uuid-dev libflac-dev libflac8 libogg-dev libogg0 libvorbis-dev libvorbis0a libvorbisenc2

libselinux1 libselinux1-dev liblzma-dev liblzma5 liblz4-dev liblz4-1 libgcrypt20 libgcrypt20-dev

libpcre3 libpcre3-dev libgpg-error-dev libgpg-error0


scp -r pi@192.168.0.42:~/temp_depends/ ./
scp -r pi@192.168.0.42:~/temp_depends/ ./deb_cross/
scp -r pi@192.168.0.42:~/temp_depends ./deb_cross

```
for i in $(apt-cache depends libsdl2-2.0-0 | grep -E 'Hängt ab von|Recommends|Suggests' | cut -d ':' -f 2,3 | sed -e s/'<'/''/ -e s/'>'/''/); do echo $i >>downloads.txt; done
```