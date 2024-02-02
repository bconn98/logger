FROM mcr.microsoft.com/devcontainers/cpp as builder

COPY . /test-app
WORKDIR /test-app
RUN \
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal -y && \
   . $HOME/.cargo/env && \
   cmake -S . -B artifacts/ && \
   cmake --build artifacts --parallel

FROM debian as final

WORKDIR /test-app
COPY --from=builder /test-app /test-app

ENTRYPOINT [ "./artifacts/log-example" ]
