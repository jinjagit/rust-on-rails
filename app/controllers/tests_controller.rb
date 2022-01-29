class TestsController < ApplicationController
  require "test_crate"

  def index
    @text = TestCrate.hello
  end
end