class TestsController < ApplicationController
  require "test_crate"
  #require "example_rust_lib"

  def index
    @text = ExampleRustLib.hello
  end
end