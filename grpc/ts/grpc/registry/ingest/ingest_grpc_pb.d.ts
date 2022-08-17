// GENERATED CODE -- DO NOT EDIT!

// package: ingest
// file: registry/ingest/ingest.proto

import * as registry_ingest_ingest_pb from "../../registry/ingest/ingest_pb";
import * as grpc from "@grpc/grpc-js";

interface IIngestServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  ingest: grpc.MethodDefinition<registry_ingest_ingest_pb.IngestRequest, registry_ingest_ingest_pb.IngestResponse>;
}

export const IngestServiceService: IIngestServiceService;

export interface IIngestServiceServer extends grpc.UntypedServiceImplementation {
  ingest: grpc.handleUnaryCall<registry_ingest_ingest_pb.IngestRequest, registry_ingest_ingest_pb.IngestResponse>;
}

export class IngestServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  ingest(argument: registry_ingest_ingest_pb.IngestRequest, callback: grpc.requestCallback<registry_ingest_ingest_pb.IngestResponse>): grpc.ClientUnaryCall;
  ingest(argument: registry_ingest_ingest_pb.IngestRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<registry_ingest_ingest_pb.IngestResponse>): grpc.ClientUnaryCall;
  ingest(argument: registry_ingest_ingest_pb.IngestRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<registry_ingest_ingest_pb.IngestResponse>): grpc.ClientUnaryCall;
}
