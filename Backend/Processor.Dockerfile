FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=processor
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /processor

COPY ./ .

# We no longer need to use the x86_64-unknown-linux-musl target
RUN cargo build -p data_processor --release

FROM debian:bookworm-slim as final

RUN apt-get update && apt install -y openssl && apt install -y ca-certificates

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /processor

# Copy our build
COPY --from=builder /processor/target/release/data_processor ./

# Use an unprivileged user.
USER processor:processor

CMD ["/processor/data_processor"]
