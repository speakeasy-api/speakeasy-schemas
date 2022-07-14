#!/bin/bash
set -e

BASEDIR=$(pwd)
GOOUTDIR="${BASEDIR}/grpc/go"
JAVAOUTDIR="${BASEDIR}/grpc/java"

echo "${BASEDIR}"
FILES=$(find grpc/protos -type f -name "*.proto")
echo $FILES

rm -rf "${GOOUTDIR}"
mkdir -p "${GOOUTDIR}"

rm -rf "${JAVAOUTDIR}"
mkdir -p "${JAVAOUTDIR}"

for proto in $FILES; do
    protoc \
        --go_out="${GOOUTDIR}" --go_opt=module="github.com/speakeasy-api/speakeasy-schemas/grpc/go" --go-grpc_out="${GOOUTDIR}" --go-grpc_opt=module="github.com/speakeasy-api/speakeasy-schemas/grpc/go" \
        "${proto}"
        # TODO: add this to generate java --plugin=protoc-gen-grpc-java=/home/trist/protoc-gen-grpc-java.exe --grpc-java_out="${JAVAOUTDIR}" \
done