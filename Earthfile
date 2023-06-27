VERSION 0.7
PROJECT applied-knowledge-systems/terraphim-logseq-md-parser-simple
FROM rust:1.70
WORKDIR /code

main-pipeline:
  PIPELINE --push 
  TRIGGER push main 
  TRIGGER pr main 
  ARG tag=latest
  BUILD +docker --tag=$tag

deps:
  COPY Cargo.lock Cargo.toml .
  RUN mkdir src
  RUN touch src/main.rs # adding main.rs stub so cargo fetch works to prepare the cache
  RUN cargo fetch 

test:
  FROM +deps
  RUN cargo test

build:
  FROM +deps
  COPY --dir src .
  RUN cargo build --release --bin logseq-md-parser-simple
  SAVE ARTIFACT target/release/logseq-md-parser-simple logseq-md-parser-simple

docker:
  ARG --required tag
  COPY +build/logseq-md-parser-simple logseq-md-parser-simple
  ENTRYPOINT ["./logseq-md-parser-simple","--path","/data"]
  SAVE IMAGE --push ghcr.io/applied-knowledge-systems/logseq-md-parser-simple:$tag
