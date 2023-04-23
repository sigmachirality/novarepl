FROM rustlang/rust:nightly as build-stage
RUN apt-get update && \
		apt-get install -y clang binaryen
ENV PATH="$PATH:/root/.cache/.wasm-pack/.wasm-bindgen-cargo-install-0.2.84/bin"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup toolchain install nightly-2022-08-09-aarch64-unknown-linux-gnu
RUN rustup component add rust-src --toolchain nightly-2022-08-09-aarch64-unknown-linux-gnu
COPY . /
RUN rustup run nightly-2022-08-09-aarch64-unknown-linux-gnu wasm-pack --verbose build --target web
FROM scratch
COPY --from=build-stage /pkg /build