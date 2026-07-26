# syntax=docker/dockerfile:1.7

FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG YOMIKA_REPOSITORY=proxlavee/yomika

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    fonts-noto-cjk \
    libayatana-appindicator3-1 \
    libgomp1 \
    librsvg2-2 \
    libssl3 \
    libwebkit2gtk-4.1-0 \
    libxdo3 \
    && curl -fL "https://github.com/${YOMIKA_REPOSITORY}/releases/latest/download/yomika_linux_x64" -o /usr/local/bin/yomika \
    && chmod 0755 /usr/local/bin/yomika \
    && apt-get purge -y --auto-remove curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --shell /bin/bash yomika \
    && install -d -o yomika -g yomika -m 755 /home/yomika/.local/share/Yomika

USER yomika
WORKDIR /home/yomika

VOLUME ["/home/yomika/.local/share/Yomika"]
EXPOSE 4000

CMD ["/usr/local/bin/yomika", "--headless", "--host", "0.0.0.0", "--port", "4000"]
