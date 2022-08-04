package dev.speakeasyapi.schemas;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * IngestService is the service definition for the registry ingest endpoint.
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.48.1)",
    comments = "Source: registry/ingest/ingest.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class IngestServiceGrpc {

  private IngestServiceGrpc() {}

  public static final String SERVICE_NAME = "ingest.IngestService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<dev.speakeasyapi.schemas.Ingest.IngestRequest,
      dev.speakeasyapi.schemas.Ingest.IngestResponse> getIngestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Ingest",
      requestType = dev.speakeasyapi.schemas.Ingest.IngestRequest.class,
      responseType = dev.speakeasyapi.schemas.Ingest.IngestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<dev.speakeasyapi.schemas.Ingest.IngestRequest,
      dev.speakeasyapi.schemas.Ingest.IngestResponse> getIngestMethod() {
    io.grpc.MethodDescriptor<dev.speakeasyapi.schemas.Ingest.IngestRequest, dev.speakeasyapi.schemas.Ingest.IngestResponse> getIngestMethod;
    if ((getIngestMethod = IngestServiceGrpc.getIngestMethod) == null) {
      synchronized (IngestServiceGrpc.class) {
        if ((getIngestMethod = IngestServiceGrpc.getIngestMethod) == null) {
          IngestServiceGrpc.getIngestMethod = getIngestMethod =
              io.grpc.MethodDescriptor.<dev.speakeasyapi.schemas.Ingest.IngestRequest, dev.speakeasyapi.schemas.Ingest.IngestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Ingest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  dev.speakeasyapi.schemas.Ingest.IngestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  dev.speakeasyapi.schemas.Ingest.IngestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new IngestServiceMethodDescriptorSupplier("Ingest"))
              .build();
        }
      }
    }
    return getIngestMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static IngestServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<IngestServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<IngestServiceStub>() {
        @java.lang.Override
        public IngestServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new IngestServiceStub(channel, callOptions);
        }
      };
    return IngestServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static IngestServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<IngestServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<IngestServiceBlockingStub>() {
        @java.lang.Override
        public IngestServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new IngestServiceBlockingStub(channel, callOptions);
        }
      };
    return IngestServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static IngestServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<IngestServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<IngestServiceFutureStub>() {
        @java.lang.Override
        public IngestServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new IngestServiceFutureStub(channel, callOptions);
        }
      };
    return IngestServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * IngestService is the service definition for the registry ingest endpoint.
   * </pre>
   */
  public static abstract class IngestServiceImplBase implements io.grpc.BindableService {

    /**
     * <pre>
     *  Ingest is the rpc handling ingest from the SDK
     * </pre>
     */
    public void ingest(dev.speakeasyapi.schemas.Ingest.IngestRequest request,
        io.grpc.stub.StreamObserver<dev.speakeasyapi.schemas.Ingest.IngestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getIngestMethod(), responseObserver);
    }

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
          .addMethod(
            getIngestMethod(),
            io.grpc.stub.ServerCalls.asyncUnaryCall(
              new MethodHandlers<
                dev.speakeasyapi.schemas.Ingest.IngestRequest,
                dev.speakeasyapi.schemas.Ingest.IngestResponse>(
                  this, METHODID_INGEST)))
          .build();
    }
  }

  /**
   * <pre>
   * IngestService is the service definition for the registry ingest endpoint.
   * </pre>
   */
  public static final class IngestServiceStub extends io.grpc.stub.AbstractAsyncStub<IngestServiceStub> {
    private IngestServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected IngestServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new IngestServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     *  Ingest is the rpc handling ingest from the SDK
     * </pre>
     */
    public void ingest(dev.speakeasyapi.schemas.Ingest.IngestRequest request,
        io.grpc.stub.StreamObserver<dev.speakeasyapi.schemas.Ingest.IngestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getIngestMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * <pre>
   * IngestService is the service definition for the registry ingest endpoint.
   * </pre>
   */
  public static final class IngestServiceBlockingStub extends io.grpc.stub.AbstractBlockingStub<IngestServiceBlockingStub> {
    private IngestServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected IngestServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new IngestServiceBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     *  Ingest is the rpc handling ingest from the SDK
     * </pre>
     */
    public dev.speakeasyapi.schemas.Ingest.IngestResponse ingest(dev.speakeasyapi.schemas.Ingest.IngestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getIngestMethod(), getCallOptions(), request);
    }
  }

  /**
   * <pre>
   * IngestService is the service definition for the registry ingest endpoint.
   * </pre>
   */
  public static final class IngestServiceFutureStub extends io.grpc.stub.AbstractFutureStub<IngestServiceFutureStub> {
    private IngestServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected IngestServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new IngestServiceFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     *  Ingest is the rpc handling ingest from the SDK
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<dev.speakeasyapi.schemas.Ingest.IngestResponse> ingest(
        dev.speakeasyapi.schemas.Ingest.IngestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getIngestMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_INGEST = 0;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final IngestServiceImplBase serviceImpl;
    private final int methodId;

    MethodHandlers(IngestServiceImplBase serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_INGEST:
          serviceImpl.ingest((dev.speakeasyapi.schemas.Ingest.IngestRequest) request,
              (io.grpc.stub.StreamObserver<dev.speakeasyapi.schemas.Ingest.IngestResponse>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  private static abstract class IngestServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    IngestServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return dev.speakeasyapi.schemas.Ingest.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("IngestService");
    }
  }

  private static final class IngestServiceFileDescriptorSupplier
      extends IngestServiceBaseDescriptorSupplier {
    IngestServiceFileDescriptorSupplier() {}
  }

  private static final class IngestServiceMethodDescriptorSupplier
      extends IngestServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final String methodName;

    IngestServiceMethodDescriptorSupplier(String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (IngestServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new IngestServiceFileDescriptorSupplier())
              .addMethod(getIngestMethod())
              .build();
        }
      }
    }
    return result;
  }
}
