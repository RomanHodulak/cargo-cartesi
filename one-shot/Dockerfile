# stage to build the initial cartesi machine
FROM cartesi/server-manager:0.4.0 as machine-server

ARG BINARY
ARG DAPP_FS
WORKDIR /opt/cartesi/dapp

# copy dapp ext2 from dapp stage
COPY ${DAPP_FS} .

# copy rootfs, linux, rom
COPY linux-5.5.19-ctsi-6.bin .
COPY rootfs.ext2 .
COPY rom.bin .

# build machine
RUN cartesi-machine \
  --ram-length=128Mi \
  --rollup \
  --flash-drive=label:dapp,filename:dapp.ext2 \
  --flash-drive=label:root,filename:rootfs.ext2 \
  --ram-image=linux-5.5.19-ctsi-6.bin \
  --rom-image=rom.bin \
  --store=/opt/cartesi/share/dapp-bin \
  -- "/mnt/dapp/${BINARY}"

# switch back to server-manager workdir
WORKDIR /opt/cartesi/bin

# stage to copy the stored machine
FROM scratch
COPY --from=machine-server /opt/cartesi/share/dapp-bin .
