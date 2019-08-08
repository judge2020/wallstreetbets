FROM rustlang/rust:nightly

LABEL "com.github.actions.name"="wsb-wincompile"
LABEL "com.github.actions.description"="Compile rust program with target windows"
LABEL "com.github.actions.icon"="package"
LABEL "com.github.actions.color"="gray-dark"

LABEL version="0.0.1"
LABEL repository="https://github.com/judge2020/wallstreetbets"
LABEL maintainer="Hunter Ray <me@judge.sh>"

RUN rustup target add x86_64-pc-windows-gnu
RUN apt update && apt install -y mingw-w64

COPY "entrypoint.sh" "/entrypoint.sh"
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
