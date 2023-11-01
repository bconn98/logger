FROM rust

RUN apt-get update && apt-get install -y cmake g++

COPY . /test-app
WORKDIR /test-app
RUN cmake -S . -B artifacts/ && cmake --build artifacts --parallel

ENTRYPOINT [ "./artifacts/log-example" ]
