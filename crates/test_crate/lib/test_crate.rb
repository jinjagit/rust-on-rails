require "helix_runtime"

begin
  require "test_crate/native"
rescue LoadError
  warn "Unable to load test_crate/native. Please run `rake build`"
end
