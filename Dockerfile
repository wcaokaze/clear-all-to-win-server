
FROM rustlang/rust:nightly

RUN apt update
RUN apt -y install libpq-dev postgresql

WORKDIR /usr/src/clear-all-to-win-server
COPY . .

RUN rustup run nightly cargo install --path .

CMD ["clear-all-to-win-server"]

