FROM rust:1.68.1-buster AS builder
WORKDIR /home
RUN apt update
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt install -y nodejs
#RUN apt install -y npm

COPY ./package.json .
COPY ./tsconfig.json .
COPY ./public ./public
RUN npm i

COPY ./wasm ./wasm
COPY ./src ./src

#ENTRYPOINT ["tail", "-f", "/dev/null"]

RUN npm run build:docker

FROM nginx:1.12-alpine
RUN echo "events {  \
  worker_connections 1024;  \
} \
http {  \
  include mime.types;  \
  sendfile on; \
  server {  \
    listen 80; \
    resolver 127.0.0.11;  \
    autoindex off;  \
    server_name sudoku.caua-rinaldi.dev;  \
    absolute_redirect off; \
    root /usr/share/nginx/html;  \
    server_tokens off;  \
    gzip_static on; \
    \
    location / { \
      try_files \$uri \$uri/ =404; \
    } \
  } \
}" > /etc/nginx/nginx.conf
COPY --from=builder /home/build /usr/share/nginx/html
EXPOSE 80
