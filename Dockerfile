FROM rust

WORKDIR /usr/src/app

COPY ./Cargo.toml /usr/src/app/Cargo.toml
COPY ./src/* /usr/src/app/

EXPOSE 1337

CMD ["/usr/local/cargo/bin/cargo", "run"]
