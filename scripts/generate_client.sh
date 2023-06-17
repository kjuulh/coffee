#!/bin/bash

docker run --rm -v ${PWD}/crates:/local/ swaggerapi/swagger-codegen-cli generate \
    -i https://git.front.kjuulh.io/swagger.v1.json \
    -l rust \
    -o /local/gitea_client


