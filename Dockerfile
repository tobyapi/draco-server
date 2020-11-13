FROM rustlang/rust:nightly

RUN apt-get update && apt-get install -y \
    git \
    cmake \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/google/draco.git \
 && cd draco \
 && mkdir build_dir \ 
 && cd build_dir \
 && cmake ../ \
 && make

RUN mkdir /output

ENV ROCKET_ENV development
ENV ENCODER_PATH /draco/build_dir/draco_encoder
ENV OUTPUT_DIR /output

WORKDIR /draco-server
COPY . .

RUN cargo install --path .

ENTRYPOINT ["cargo", "run"]
