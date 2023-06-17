#!/bin/bash

docker run --rm -v ${PWD}/crates:/local/ openapitools/openapi-generator-cli generate \
    -i https://git.front.kjuulh.io/swagger.v1.json \
    -g rust \
    -o /local/gitea_client


