# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: registry/embedaccesstoken/embedaccesstoken.proto

require 'google/protobuf'

Google::Protobuf::DescriptorPool.generated_pool.build do
  add_file("registry/embedaccesstoken/embedaccesstoken.proto", :syntax => :proto3) do
    add_message "embedaccesstoken.EmbedAccessTokenRequest" do
      repeated :filters, :message, 1, "embedaccesstoken.EmbedAccessTokenRequest.Filter"
      proto3_optional :customer_id, :string, 2
      proto3_optional :display_name, :string, 3
      map :jwt_custom_claims, :string, :string, 4
      map :permissions, :string, :bool, 5
    end
    add_message "embedaccesstoken.EmbedAccessTokenRequest.Filter" do
      optional :key, :string, 1
      optional :operator, :string, 2
      optional :value, :string, 3
    end
    add_message "embedaccesstoken.EmbedAccessTokenResponse" do
      optional :access_token, :string, 1
    end
  end
end

module Embedaccesstoken
  EmbedAccessTokenRequest = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("embedaccesstoken.EmbedAccessTokenRequest").msgclass
  EmbedAccessTokenRequest::Filter = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("embedaccesstoken.EmbedAccessTokenRequest.Filter").msgclass
  EmbedAccessTokenResponse = ::Google::Protobuf::DescriptorPool.generated_pool.lookup("embedaccesstoken.EmbedAccessTokenResponse").msgclass
end