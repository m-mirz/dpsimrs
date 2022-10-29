FROM rust:bullseye

RUN apt-get -y update
RUN apt-get -y install sudo python3-dev python-dev python3-pip vim nano
RUN pip install maturin
RUN rustup component add rustfmt

ARG USERNAME=dpsimrs
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
	&& useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
	&& echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
	&& chmod 0440 /etc/sudoers.d/$USERNAME

WORKDIR /usr/src/dpsimrs

ENV PYTHONPATH=/workspaces/dpsimrs/target/wheels

COPY . .
