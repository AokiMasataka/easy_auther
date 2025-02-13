FROM denoland/deno:latest as build

WORKDIR /app
COPY ./ .
run deno task build

FROM nginx:stable-perl

# RUN rm -f /etc/nginx/conf.d/*
# COPY ./default.conf /etc/nginx/conf.d/
COPY ./nginx.conf /etc/nginx/

COPY --from=build /app/dist /usr/share/nginx/html
