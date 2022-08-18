// GENERATED CODE -- DO NOT EDIT!

// package: embedaccesstoken
// file: registry/embedaccesstoken/embedaccesstoken.proto

import * as registry_embedaccesstoken_embedaccesstoken_pb from "../../registry/embedaccesstoken/embedaccesstoken_pb";
import * as grpc from "@grpc/grpc-js";

interface IEmbedAccessTokenServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  get: grpc.MethodDefinition<registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest, registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse>;
}

export const EmbedAccessTokenServiceService: IEmbedAccessTokenServiceService;

export interface IEmbedAccessTokenServiceServer extends grpc.UntypedServiceImplementation {
  get: grpc.handleUnaryCall<registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest, registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse>;
}

export class EmbedAccessTokenServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  get(argument: registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest, callback: grpc.requestCallback<registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse>): grpc.ClientUnaryCall;
  get(argument: registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse>): grpc.ClientUnaryCall;
  get(argument: registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse>): grpc.ClientUnaryCall;
}
