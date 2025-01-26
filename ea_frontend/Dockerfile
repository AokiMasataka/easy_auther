FROM denoland/deno:latest as build

WORKDIR /app
COPY ./ .
run deno task build

FROM nginx:stable-perl

COPY --from=build /app/dist /usr/share/nginx/html