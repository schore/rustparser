FROM ubuntu:focal

RUN apt-get update -y
RUN apt-get install curl -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN source $HOME/.cargo/env
