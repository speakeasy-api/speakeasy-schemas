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

  hasMaskingMetadata(): boolean;
  clearMaskingMetadata(): void;
  getMaskingMetadata(): IngestRequest.MaskingMetadata | undefined;
  setMaskingMetadata(value?: IngestRequest.MaskingMetadata): void;

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
    maskingMetadata?: IngestRequest.MaskingMetadata.AsObject,
  }

  export class MaskingMetadata extends jspb.Message {
    getRequestHeaderMasksMap(): jspb.Map<string, string>;
    clearRequestHeaderMasksMap(): void;
    getRequestCookieMasksMap(): jspb.Map<string, string>;
    clearRequestCookieMasksMap(): void;
    getRequestFieldMasksStringMap(): jspb.Map<string, string>;
    clearRequestFieldMasksStringMap(): void;
    getRequestFieldMasksNumberMap(): jspb.Map<string, string>;
    clearRequestFieldMasksNumberMap(): void;
    getResponseHeaderMasksMap(): jspb.Map<string, string>;
    clearResponseHeaderMasksMap(): void;
    getResponseCookieMasksMap(): jspb.Map<string, string>;
    clearResponseCookieMasksMap(): void;
    getResponseFieldMasksStringMap(): jspb.Map<string, string>;
    clearResponseFieldMasksStringMap(): void;
    getResponseFieldMasksNumberMap(): jspb.Map<string, string>;
    clearResponseFieldMasksNumberMap(): void;
    getQueryStringMasksMap(): jspb.Map<string, string>;
    clearQueryStringMasksMap(): void;
    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): MaskingMetadata.AsObject;
    static toObject(includeInstance: boolean, msg: MaskingMetadata): MaskingMetadata.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: MaskingMetadata, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): MaskingMetadata;
    static deserializeBinaryFromReader(message: MaskingMetadata, reader: jspb.BinaryReader): MaskingMetadata;
  }

  export namespace MaskingMetadata {
    export type AsObject = {
      requestHeaderMasksMap: Array<[string, string]>,
      requestCookieMasksMap: Array<[string, string]>,
      requestFieldMasksStringMap: Array<[string, string]>,
      requestFieldMasksNumberMap: Array<[string, string]>,
      responseHeaderMasksMap: Array<[string, string]>,
      responseCookieMasksMap: Array<[string, string]>,
      responseFieldMasksStringMap: Array<[string, string]>,
      responseFieldMasksNumberMap: Array<[string, string]>,
      queryStringMasksMap: Array<[string, string]>,
    }
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

