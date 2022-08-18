# speakeasy-schemas

## Local Setup

1. Follow the quickstart guide to install `protoc` and the golang plugins: https://grpc.io/docs/languages/go/quickstart/
   1. Just be aware newer versions broke JS/TS support so you need to for now make sure you are on version `3.20.1`
   2. Install via `brew install protobuf@3 && brew link protobuf@3` (or another method if you prefer)
2. Make sure you are using JDK 8
3. Install gradle so its usable from the command line: https://gradle.org/install/
4. Run `npm install` in `speakeasy-schemas/grpc/ts` to install the typescript compiler (if you're using an M1, add the `--target_arch=x64` flag to the install command)
5. Setup `protolint` in your IDE. (vscode: https://marketplace.visualstudio.com/items?itemName=Plex.vscode-protolint)

## Linting

1. Run `make lint` from the root of the project.

## Generate schemas

### Golang

1. Run `make generate` from the root of the project.
   
This will generate to golang files in `/grpc/go`. Then this repo just needs to be pushed to github and the golang files can be included in other projects.

### Java

1. Run `make generate` from the root of the project.

This will generate the java library in `/grpc/java/lib/build/libs/lib.jar` which can be copied to the `lib/` directory of another project.