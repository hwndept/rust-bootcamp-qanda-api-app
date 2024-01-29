#!/bin/sh

docker run --name rust-bootcamp-qanda-api-app-db \
    -e POSTGRES_USER=user \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_DB=default \
    -p 5432:5432 \
    --rm \
    postgres
