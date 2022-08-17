// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var registry_ingest_ingest_pb = require('../../registry/ingest/ingest_pb.js');

function serialize_ingest_IngestRequest(arg) {
  if (!(arg instanceof registry_ingest_ingest_pb.IngestRequest)) {
    throw new Error('Expected argument of type ingest.IngestRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ingest_IngestRequest(buffer_arg) {
  return registry_ingest_ingest_pb.IngestRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_ingest_IngestResponse(arg) {
  if (!(arg instanceof registry_ingest_ingest_pb.IngestResponse)) {
    throw new Error('Expected argument of type ingest.IngestResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_ingest_IngestResponse(buffer_arg) {
  return registry_ingest_ingest_pb.IngestResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// IngestService is the service definition for the registry ingest endpoint.
var IngestServiceService = exports.IngestServiceService = {
  //  Ingest is the rpc handling ingest from the SDK.
ingest: {
    path: '/ingest.IngestService/Ingest',
    requestStream: false,
    responseStream: false,
    requestType: registry_ingest_ingest_pb.IngestRequest,
    responseType: registry_ingest_ingest_pb.IngestResponse,
    requestSerialize: serialize_ingest_IngestRequest,
    requestDeserialize: deserialize_ingest_IngestRequest,
    responseSerialize: serialize_ingest_IngestResponse,
    responseDeserialize: deserialize_ingest_IngestResponse,
  },
};

exports.IngestServiceClient = grpc.makeGenericClientConstructor(IngestServiceService);
