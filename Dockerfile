FROM rust:1.67

RUN mkdir /code

WORKDIR /code

RUN apt-get update
RUN apt-get install -qy beignet-opencl-icd ocl-icd-opencl-dev

CMD cargo build