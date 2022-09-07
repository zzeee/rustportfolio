FROM rust:1.51-buster
#RUN rustup install nightly-2020-10-05
RUN rustup target add wasm32-unknown-unknown
##--toolchain nightly-2020-10-05
RUN mkdir /var/fast_gateway
WORKDIR /var/fast_gateway
COPY . /var/fast_gateway/
RUN pwd && ls
RUN rm -rf ./target
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs
#RUN cd swagger
RUN npm install
#RUN cd ..
#RUN apk add --update nodejs npm
RUN rustup default stable
RUN cargo build
RUN cargo build --release
#RUN cargo  +nightly-2020-10-05 build --release
ENTRYPOINT ["/bin/sh","--","/var/fast_gateway/entrypoint.sh"]

