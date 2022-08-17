// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.20.1
// source: registry/ingest/ingest.proto

package ingest

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// IngestServiceClient is the client API for IngestService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type IngestServiceClient interface {
	//  Ingest is the rpc handling ingest from the SDK
	Ingest(ctx context.Context, in *IngestRequest, opts ...grpc.CallOption) (*IngestResponse, error)
}

type ingestServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewIngestServiceClient(cc grpc.ClientConnInterface) IngestServiceClient {
	return &ingestServiceClient{cc}
}

func (c *ingestServiceClient) Ingest(ctx context.Context, in *IngestRequest, opts ...grpc.CallOption) (*IngestResponse, error) {
	out := new(IngestResponse)
	err := c.cc.Invoke(ctx, "/ingest.IngestService/Ingest", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// IngestServiceServer is the server API for IngestService service.
// All implementations must embed UnimplementedIngestServiceServer
// for forward compatibility
type IngestServiceServer interface {
	//  Ingest is the rpc handling ingest from the SDK
	Ingest(context.Context, *IngestRequest) (*IngestResponse, error)
	mustEmbedUnimplementedIngestServiceServer()
}

// UnimplementedIngestServiceServer must be embedded to have forward compatible implementations.
type UnimplementedIngestServiceServer struct {
}

func (UnimplementedIngestServiceServer) Ingest(context.Context, *IngestRequest) (*IngestResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Ingest not implemented")
}
func (UnimplementedIngestServiceServer) mustEmbedUnimplementedIngestServiceServer() {}

// UnsafeIngestServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to IngestServiceServer will
// result in compilation errors.
type UnsafeIngestServiceServer interface {
	mustEmbedUnimplementedIngestServiceServer()
}

func RegisterIngestServiceServer(s grpc.ServiceRegistrar, srv IngestServiceServer) {
	s.RegisterService(&IngestService_ServiceDesc, srv)
}

func _IngestService_Ingest_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(IngestRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(IngestServiceServer).Ingest(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/ingest.IngestService/Ingest",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(IngestServiceServer).Ingest(ctx, req.(*IngestRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// IngestService_ServiceDesc is the grpc.ServiceDesc for IngestService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var IngestService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "ingest.IngestService",
	HandlerType: (*IngestServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Ingest",
			Handler:    _IngestService_Ingest_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "registry/ingest/ingest.proto",
}
