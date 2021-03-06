#########################################################################
## Builder Stage
#########################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=m_calc
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /m_calc

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

###########################################################################
## Final Stage
###########################################################################
FROM alpine

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /m_calc

# Copy our build
COPY --from=builder /m_calc/target/x86_64-unknown-linux-musl/release/m_calc ./

# Use an unprivileged user.
USER root

ENTRYPOINT ["/m_calc/m_calc"]
