#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf

cargo build --target=${TARGET_ARCH}
scp ./target/${TARGET_ARCH}/debug/telnet_chatroom pi@10.0.0.144:~
ssh pi@10.0.0.144 ./telnet_chatroom