FROM ubuntu:focal

RUN apt-get update -y
RUN apt-get install -y curl 

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN cat $HOME/.cargo/env
RUN echo "export PATH=$HOME/.cargo/bin:$PATH" >> ~/.bashrc
RUN cat ~/.bashrc
