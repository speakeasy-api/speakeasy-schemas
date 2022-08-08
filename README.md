# speakeasy-schemas

## Local Setup

1. Follow the quickstart guide to install `protoc` and the golang plugins: https://grpc.io/docs/languages/go/quickstart/
2. Make sure you are using JDK 8
3. Install gradle so its usable from the command line: https://gradle.org/install/
4. Setup `protolint` in your IDE. (vscode: https://marketplace.visualstudio.com/items?itemName=Plex.vscode-protolint)

## Linting

1. Run `make lint` from the root of the project.

## Generate schemas

### Golang

1. Run `make generate` from the root of the project.
   
This will generate to golang files in `/grpc/go`. Then this repo just needs to be pushed to github and the golang files can be included in other projects.

### Java

1. Run `make generate` from the root of the project.

This will generate the java library in `/grpc/java/lib/build/libs/lib.jar` which can be copied to the `lib/` directory of another project.