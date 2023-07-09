FROM rust as build

WORKDIR /usr/src/tracking-app
COPY . .

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    openssl \
    libssl-dev
RUN cargo install --path .

EXPOSE 8000
CMD /usr/src/tracking-app/target/release/tracking