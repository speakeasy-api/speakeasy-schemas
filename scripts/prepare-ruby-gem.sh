#!/bin/bash
set -e

BASEDIR=$(pwd)
RUBYDIR="${BASEDIR}/grpc/ruby"

bundle exec gem bump speakeasy_pb
pushd $RUBYDIR
gem build
popd