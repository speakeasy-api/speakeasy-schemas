// package: ingest
// file: registry/ingest/ingest.proto

import * as jspb from "google-protobuf";

export class IngestRequest extends jspb.Message {
  getHar(): string;
  setHar(value: string): void;

  getPathHint(): string;
  setPathHint(value: string): void;

  getApiId(): string;
  setApiId(value: string): void;

  getVersionId(): string;
  setVersionId(value: string): void;

  getCustomerId(): string;
  setCustomerId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IngestRequest.AsObject;
  static toObject(includeInstance: boolean, msg: IngestRequest): IngestRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IngestRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IngestRequest;
  static deserializeBinaryFromReader(message: IngestRequest, reader: jspb.BinaryReader): IngestRequest;
}

export namespace IngestRequest {
  export type AsObject = {
    har: string,
    pathHint: string,
    apiId: string,
    versionId: string,
    customerId: string,
  }
}

export class IngestResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IngestResponse.AsObject;
  static toObject(includeInstance: boolean, msg: IngestResponse): IngestResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IngestResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IngestResponse;
  static deserializeBinaryFromReader(message: IngestResponse, reader: jspb.BinaryReader): IngestResponse;
}

export namespace IngestResponse {
  export type AsObject = {
  }
}

