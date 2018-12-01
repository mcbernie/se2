#!/bin/bash
set -e;

printf "*** Extracting target dependencies ***\n";
if [ -d "$HOME/deb-deps" ]; then
	cd $HOME/pi-tools/arm-bcm2708/arm-linux-gnueabihf/arm-linux-gnueabihf/sysroot/;

	for i in `find $HOME/deb-deps -name "*.deb" -type f`; do
    	echo "Extracting: $i";

		output_ar=$(ar -t $i);
		if [[ $output_ar == *"data.tar.gz"* ]]; then
			ar -p $i data.tar.gz > "$i.tar.gz";
			tar xf "/$i.tar.gz";
			#rm "$i.tar.gz";
		else
			ar -p $i data.tar.xz > "$i.tar.xz" || true;
			tar xf "/$i.tar.xz" || true;
			#rm "$i.tar.xz";
		fi

	done

fi

#echo "$(ls /home/cross/pi-tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/arm-linux-gnueabihf/sysroot/usr/include/openssl)";

printf "\n*** Cross compiling project ***\n";
cd $HOME/project;

if [ $(uname -m) == 'x86_64' ]; then
	printf "\n Using x64 toolchain\n";
	TOOLCHAIN=$TOOLCHAIN_64;
else
	printf "\n Using 32bit toolchain\n";
	TOOLCHAIN=$TOOLCHAIN_32;
fi

#Include the cross compilation binaries

export PATH=$TOOLCHAIN:$PATH;

export SYSROOT="$HOME/pi-tools/arm-bcm2708/arm-linux-gnueabihf/arm-linux-gnueabihf/sysroot";
export ARM_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_INCLUDE_DIR="$SYSROOT/usr/include/arm-linux-gnueabihf";
export ARM_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR="$SYSROOT/usr/lib/arm-linux-gnueabihf";

#export CFLAGS="-march=armv6 -mfloat-abi=hard -mfpu=vfp";
#export CCFLAGS="-march=armv6 -mfloat-abi=hard -mfpu=vfp";

export RUSTFLAGS="-C link-args=-Wl,-rpath,$SYSROOT/lib/arm-linux-gnueabihf,-rpath,$SYSROOT/usr/lib/arm-linux-gnueabihf"

export CC="$TOOLCHAIN/gcc-sysroot";
export AR="$TOOLCHAIN/arm-linux-gnueabihf-ar";
export RUST_BACKTRACE=1;
#export FFMPEG_DIR="$SYSROOT/usr";
export PKG_CONFIG_PATH="$SYSROOT/usr/lib/arm-linux-gnueabihf/pkgconfig";
export PKG_CONFIG_ALLOW_CROSS=1;
#export LIBCLANG_PATH="$SYSROOT/usr/lib";
#export CARGO_FEATURE_STATIC=0;
#export CARGO_FEATURE_BUILD=1;

flags="--target=arm-unknown-linux-gnueabihf";
$HOME/.cargo/bin/cargo "$@" "$flags";
