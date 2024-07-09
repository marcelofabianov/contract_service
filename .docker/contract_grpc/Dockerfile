FROM rust:1.78

WORKDIR /app

RUN apt-get update -yqq && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends \
  zsh \
  git \
  wget \
  curl \
  unzip \
  nano \
  tree \
  build-essential \
  librdkafka-dev \
  libprotobuf-dev \
  protobuf-compiler && \
  rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli && cargo install cargo-watch

COPY . .

# gRPC
EXPOSE 50051
# tracer
EXPOSE 4317

CMD ["tail", "-f", "/dev/null"]
