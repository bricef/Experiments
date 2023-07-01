FROM gerbil/ubuntu:latest AS build
WORKDIR /build
RUN gxpkg install github.com/drewc/ftw
ADD src/ src/
ADD gerbil.pkg gerbil.pkg
RUN gxc -v -exe -static -o app src/entrypoint.ss

FROM scratch
# Set up libraries
ENV LD_LIBRARY_PATH=/lib64/     
COPY --from=build /lib/x86_64-linux-gnu/libssl.so.3 /lib64/
COPY --from=build /lib/x86_64-linux-gnu/libcrypto.so.3 /lib64/
COPY --from=build /lib/x86_64-linux-gnu/libc.so.6 /lib64/
COPY --from=build /lib/x86_64-linux-gnu/libm.so.6 /lib64/
COPY --from=build /lib64/ld-linux-x86-64.so.2 /lib64/
# Move application binary
COPY --from=build /build/app /
ENTRYPOINT ["/app"]