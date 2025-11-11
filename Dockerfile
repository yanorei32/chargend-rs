FROM rust:1.91.1 as build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src
RUN cargo new chargend-rs
COPY LICENSE Cargo.toml Cargo.lock /usr/src/chargend-rs/
WORKDIR /usr/src/chargend-rs
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN	cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

RUN cargo build --release
COPY src/ /usr/src/chargend-rs/src/

RUN touch src/* && cargo build --release

FROM debian:bookworm-slim

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/chargend-rs/CREDITS \
	/usr/src/chargend-rs/LICENSE \
	/usr/share/licenses/chargend-rs/

COPY --chown=root:root --from=build-env \
	/usr/src/chargend-rs/target/release/chargend-rs \
	/usr/bin/chargend-rs

CMD ["/usr/bin/chargend-rs"]
