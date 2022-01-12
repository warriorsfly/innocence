FROM rust:latest as build

WORKDIR /usr/src/innocence
COPY . .

RUN cargo install --path .

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/innocence /usr/local/bin/innocence

CMD ["innocence"]

# ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# # Our first FROM statement declares the build environment.
# FROM ${BASE_IMAGE} AS builder

# # Add our source code.
# ADD . ./

# # Fix permissions on source code.
# RUN sudo chown -R rust:rust /home/rust

# # For cache, if build failed
# RUN cargo update
# # Build our application.
# RUN cargo build --release

# FROM alpine:latest
# RUN apk --no-cache add ca-certificates
# COPY --from=builder \
#     /home/rust/src/target/x86_64-unknown-linux-musl/release/innocence \
#     /usr/local/bin/
# CMD /usr/local/bin/innocence