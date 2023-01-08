# syntax=docker/dockerfile:1
FROM ubuntu:latest
ENV HOME /root
WORKDIR /project
COPY . /project/
RUN /project/scripts/installProdDeps.sh
RUN /project/scripts/installTestDeps.sh
ENV PATH="$HOME/.cargo/bin:$PATH"
CMD python3 /project/scripts/test.py
