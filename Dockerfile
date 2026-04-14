FROM --platform=linux/amd64 ubuntu:16.04
ENV DEBIAN_FRONTEND=noninteractive

# FUZZER_TYPE can be normal or angora


RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y git build-essential wget zlib1g-dev python-pip python-dev && \
    apt-get install -y vim && \
    apt-get clean

# Install newer cmake
RUN wget https://cmake.org/files/v3.20/cmake-3.20.0-linux-x86_64.tar.gz && \
    tar -C /usr/local -xzf cmake-3.20.0-linux-x86_64.tar.gz && \
    rm cmake-3.20.0-linux-x86_64.tar.gz && \
    ln -s /usr/local/cmake-3.20.0-linux-x86_64/bin/cmake /usr/local/bin/cmake  # ← symlink

# Install Go
RUN wget https://go.dev/dl/go1.18.10.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.18.10.linux-amd64.tar.gz && \
    rm go1.18.10.linux-amd64.tar.gz

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PIN_ROOT=/pin-3.7-97619-g0d0c92f4f-gcc-linux \
    GOPATH=/go \
    PATH=/clang+llvm/bin:/usr/local/cmake-3.20.0-linux-x86_64/bin:/usr/local/go/bin:/usr/local/cargo/bin:/angora/bin/:/go/bin:$PATH \
    LD_LIBRARY_PATH=/clang+llvm/lib:$LD_LIBRARY_PATH
# ↑ add cmake bin to PATH

RUN mkdir -p angora_closure
WORKDIR angora_closure
# Copy only the specific scripts needed — cache only busts if THAT script changes
COPY ./build/install_rust.sh  ./build/install_rust.sh
RUN ./build/install_rust.sh

COPY ./build/install_llvm.sh  ./build/install_llvm.sh
RUN PREFIX=/ ./build/install_llvm.sh

COPY ./build/install_tools.sh ./build/install_tools.sh
RUN ./build/install_tools.sh

# Now copy the rest of build/ and source
COPY ./build ./build

COPY . .

ARG FUZZER_TYPE=normal
ENV FUZZER_TYPE=${FUZZER_TYPE}

RUN ./build/build.sh

VOLUME ["/data"]
CMD ["/bin/bash"]