VERSION 0.7
FROM  rust:1.70
WORKDIR /code


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
  FROM ubuntu:18.04
  COPY +build/logseq-md-parser-simple logseq-md-parser-simple
  ENTRYPOINT ["./logseq_md_parser"]
  SAVE IMAGE --push aks/logseq_md_parser:$tag