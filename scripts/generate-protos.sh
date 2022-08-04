#!/bin/bash
set -e

BASEDIR=$(pwd)
GOOUTDIR="${BASEDIR}/grpc/go"
JAVADIR="${BASEDIR}/grpc/java"

echo "${BASEDIR}"
FILES=$(find grpc/protos -type f -name "*.proto")
echo $FILES

rm -rf "${GOOUTDIR}"
mkdir -p "${GOOUTDIR}"

for proto in $FILES; do
    protoc \
        --go_out="${GOOUTDIR}" --go_opt=module="github.com/speakeasy-api/speakeasy-schemas/grpc/go" --go-grpc_out="${GOOUTDIR}" --go-grpc_opt=module="github.com/speakeasy-api/speakeasy-schemas/grpc/go" \
        "${proto}"
done

pushd $JAVADIR
gradle build
popd