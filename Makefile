generate:
	./scripts/generate-protos.sh
lint:
	go install github.com/yoheimuta/protolint/cmd/protolint@latest
	protolint lint ./grpc/protos