#!/bin/bash
set -e

BASEDIR=$(pwd)
PROTOPATH="${BASEDIR}/grpc/protos"
GOOUTDIR="${BASEDIR}/grpc/go"
TSBASEDIR="${BASEDIR}/grpc/ts"
TSOUTDIR="${TSBASEDIR}/grpc"
PROTOC_GEN_TS_PATH="${TSBASEDIR}/node_modules/.bin/protoc-gen-ts"
PROTOC_GEN_GRPC_PATH="${TSBASEDIR}/node_modules/.bin/grpc_tools_node_protoc_plugin"
JAVADIR="${BASEDIR}/grpc/java"

rm -rf "${GOOUTDIR}"
mkdir -p "${GOOUTDIR}"

rm -rf "${TSOUTDIR}"
mkdir -p "${TSOUTDIR}"

pushd "${PROTOPATH}"
FILES=$(find . -type f -name "*.proto")
echo $FILES

for proto in $FILES; do
    protoc \
        --go_out="${GOOUTDIR}" --go_opt=module="github.com/speakeasy-api/speakeasy-schemas/grpc/go" --go-grpc_out="${GOOUTDIR}" --go-grpc_opt=module="github.com/speakeasy-api/speakeasy-schemas/grpc/go" \
        --plugin="protoc-gen-ts=${PROTOC_GEN_TS_PATH}" --plugin=protoc-gen-grpc=${PROTOC_GEN_GRPC_PATH} --js_out="import_style=commonjs,binary:${TSOUTDIR}" --ts_out="service=grpc-node,mode=grpc-js:${TSOUTDIR}" --grpc_out="grpc_js:${TSOUTDIR}" \
        -I="." \
        "${proto}"
done
popd

pushd $JAVADIR
gradle build
popd