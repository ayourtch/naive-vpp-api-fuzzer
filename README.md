# approximate instructions

## install and compile VPP

```
cd ${HOME}
git clone https://git.fd.io/vpp
cd vpp
UNATTENDED=yes make install-deps install-ext-deps build
```

## install and build the naive API fuzzer

```
export LD_LIBRARY_PATH=/home/ayourtch/vpp/build-root/install-vpp-native/vpp/lib/
```

## start VPP

```
cd ${HOME}/vpp
LD_LIBRARY_PATH=${HOME}/vpp/build-root/install-vpp-native/vpp/lib/ ./build-root/install-vpp-native/vpp/bin/vpp 'unix { interactive cli-listen /tmp/vpp-api-cli.sock } plugins { plugin dpdk_plugin.so { disable } }'
```

## start the naive fuzzer

```
cargo run
```

