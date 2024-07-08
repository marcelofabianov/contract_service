FROM rust:1.78

WORKDIR /app

RUN apt-get update -yqq && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends \
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

EXPOSE 50051

CMD ["cargo", "watch", "-x", "run"]
