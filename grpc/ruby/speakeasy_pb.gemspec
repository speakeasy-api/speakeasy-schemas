$LOAD_PATH.push File.expand_path('lib', __dir__)
require 'speakeasy_pb'

Gem::Specification.new do |s|
  s.name        = 'speakeasy_pb'
  s.version     = SpeakeasyPb::VERSION
  s.platform    = Gem::Platform::RUBY
  s.licenses    = ['Apache-2.0']
  s.summary     = 'Speakeasy google rpc binding'
  s.homepage    = 'https://github.com/speakeasy-api/speakeasy-schemas'
  s.authors     = ['Ian Bentley']
  s.files         = Dir['{lib}/**/*', 'LICENSE', 'README.md']
  s.require_paths = ['lib']
  s.required_ruby_version = '>= 2.2.0'

  # s.add_runtime_dependency('grpc', '~> 1.51.0')
  # s.add_runtime_dependency('har', '~> 0.1.5')
  # s.add_runtime_dependency('http-cookie', '~> 1.0')
end
