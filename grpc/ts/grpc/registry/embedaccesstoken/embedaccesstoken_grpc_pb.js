// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var registry_embedaccesstoken_embedaccesstoken_pb = require('../../registry/embedaccesstoken/embedaccesstoken_pb.js');

function serialize_embedaccesstoken_EmbedAccessTokenRequest(arg) {
  if (!(arg instanceof registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest)) {
    throw new Error('Expected argument of type embedaccesstoken.EmbedAccessTokenRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_embedaccesstoken_EmbedAccessTokenRequest(buffer_arg) {
  return registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_embedaccesstoken_EmbedAccessTokenResponse(arg) {
  if (!(arg instanceof registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse)) {
    throw new Error('Expected argument of type embedaccesstoken.EmbedAccessTokenResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_embedaccesstoken_EmbedAccessTokenResponse(buffer_arg) {
  return registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse.deserializeBinary(new Uint8Array(buffer_arg));
}


// EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
var EmbedAccessTokenServiceService = exports.EmbedAccessTokenServiceService = {
  //  Get is the rpc handling access token retrieval from the SDK
get: {
    path: '/embedaccesstoken.EmbedAccessTokenService/Get',
    requestStream: false,
    responseStream: false,
    requestType: registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenRequest,
    responseType: registry_embedaccesstoken_embedaccesstoken_pb.EmbedAccessTokenResponse,
    requestSerialize: serialize_embedaccesstoken_EmbedAccessTokenRequest,
    requestDeserialize: deserialize_embedaccesstoken_EmbedAccessTokenRequest,
    responseSerialize: serialize_embedaccesstoken_EmbedAccessTokenResponse,
    responseDeserialize: deserialize_embedaccesstoken_EmbedAccessTokenResponse,
  },
};

exports.EmbedAccessTokenServiceClient = grpc.makeGenericClientConstructor(EmbedAccessTokenServiceService);
