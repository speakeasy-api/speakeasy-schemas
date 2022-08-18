package dev.speakeasyapi.accesstokens;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
 * </pre>
 */
@javax.annotation.Generated(
    value = "by gRPC proto compiler (version 1.48.1)",
    comments = "Source: registry/embedaccesstoken/embedaccesstoken.proto")
@io.grpc.stub.annotations.GrpcGenerated
public final class EmbedAccessTokenServiceGrpc {

  private EmbedAccessTokenServiceGrpc() {}

  public static final String SERVICE_NAME = "embedaccesstoken.EmbedAccessTokenService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest,
      dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse> getGetMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "Get",
      requestType = dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest.class,
      responseType = dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest,
      dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse> getGetMethod() {
    io.grpc.MethodDescriptor<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest, dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse> getGetMethod;
    if ((getGetMethod = EmbedAccessTokenServiceGrpc.getGetMethod) == null) {
      synchronized (EmbedAccessTokenServiceGrpc.class) {
        if ((getGetMethod = EmbedAccessTokenServiceGrpc.getGetMethod) == null) {
          EmbedAccessTokenServiceGrpc.getGetMethod = getGetMethod =
              io.grpc.MethodDescriptor.<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest, dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "Get"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse.getDefaultInstance()))
              .setSchemaDescriptor(new EmbedAccessTokenServiceMethodDescriptorSupplier("Get"))
              .build();
        }
      }
    }
    return getGetMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static EmbedAccessTokenServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmbedAccessTokenServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmbedAccessTokenServiceStub>() {
        @java.lang.Override
        public EmbedAccessTokenServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmbedAccessTokenServiceStub(channel, callOptions);
        }
      };
    return EmbedAccessTokenServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static EmbedAccessTokenServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmbedAccessTokenServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmbedAccessTokenServiceBlockingStub>() {
        @java.lang.Override
        public EmbedAccessTokenServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmbedAccessTokenServiceBlockingStub(channel, callOptions);
        }
      };
    return EmbedAccessTokenServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static EmbedAccessTokenServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<EmbedAccessTokenServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<EmbedAccessTokenServiceFutureStub>() {
        @java.lang.Override
        public EmbedAccessTokenServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new EmbedAccessTokenServiceFutureStub(channel, callOptions);
        }
      };
    return EmbedAccessTokenServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
   * </pre>
   */
  public static abstract class EmbedAccessTokenServiceImplBase implements io.grpc.BindableService {

    /**
     * <pre>
     *  Get is the rpc handling access token retrieval from the SDK
     * </pre>
     */
    public void get(dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest request,
        io.grpc.stub.StreamObserver<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getGetMethod(), responseObserver);
    }

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
          .addMethod(
            getGetMethod(),
            io.grpc.stub.ServerCalls.asyncUnaryCall(
              new MethodHandlers<
                dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest,
                dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse>(
                  this, METHODID_GET)))
          .build();
    }
  }

  /**
   * <pre>
   * EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
   * </pre>
   */
  public static final class EmbedAccessTokenServiceStub extends io.grpc.stub.AbstractAsyncStub<EmbedAccessTokenServiceStub> {
    private EmbedAccessTokenServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmbedAccessTokenServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmbedAccessTokenServiceStub(channel, callOptions);
    }

    /**
     * <pre>
     *  Get is the rpc handling access token retrieval from the SDK
     * </pre>
     */
    public void get(dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest request,
        io.grpc.stub.StreamObserver<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getGetMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * <pre>
   * EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
   * </pre>
   */
  public static final class EmbedAccessTokenServiceBlockingStub extends io.grpc.stub.AbstractBlockingStub<EmbedAccessTokenServiceBlockingStub> {
    private EmbedAccessTokenServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmbedAccessTokenServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmbedAccessTokenServiceBlockingStub(channel, callOptions);
    }

    /**
     * <pre>
     *  Get is the rpc handling access token retrieval from the SDK
     * </pre>
     */
    public dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse get(dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getGetMethod(), getCallOptions(), request);
    }
  }

  /**
   * <pre>
   * EmbedAccessTokenService is the service definition for the registry embed-access-token endpoint.
   * </pre>
   */
  public static final class EmbedAccessTokenServiceFutureStub extends io.grpc.stub.AbstractFutureStub<EmbedAccessTokenServiceFutureStub> {
    private EmbedAccessTokenServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected EmbedAccessTokenServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new EmbedAccessTokenServiceFutureStub(channel, callOptions);
    }

    /**
     * <pre>
     *  Get is the rpc handling access token retrieval from the SDK
     * </pre>
     */
    public com.google.common.util.concurrent.ListenableFuture<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse> get(
        dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getGetMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_GET = 0;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final EmbedAccessTokenServiceImplBase serviceImpl;
    private final int methodId;

    MethodHandlers(EmbedAccessTokenServiceImplBase serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_GET:
          serviceImpl.get((dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenRequest) request,
              (io.grpc.stub.StreamObserver<dev.speakeasyapi.accesstokens.Embedaccesstoken.EmbedAccessTokenResponse>) responseObserver);
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

  private static abstract class EmbedAccessTokenServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    EmbedAccessTokenServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return dev.speakeasyapi.accesstokens.Embedaccesstoken.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("EmbedAccessTokenService");
    }
  }

  private static final class EmbedAccessTokenServiceFileDescriptorSupplier
      extends EmbedAccessTokenServiceBaseDescriptorSupplier {
    EmbedAccessTokenServiceFileDescriptorSupplier() {}
  }

  private static final class EmbedAccessTokenServiceMethodDescriptorSupplier
      extends EmbedAccessTokenServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final String methodName;

    EmbedAccessTokenServiceMethodDescriptorSupplier(String methodName) {
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
      synchronized (EmbedAccessTokenServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new EmbedAccessTokenServiceFileDescriptorSupplier())
              .addMethod(getGetMethod())
              .build();
        }
      }
    }
    return result;
  }
}
