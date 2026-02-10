class ApplicationController < ActionController::Base
  def index
  end

  def examples
    render "examples/authentication/index"
  end

  def block
    block_name = params[:block].tr("-", "_")
    @block_partial = "blocks/#{block_name}/#{block_name}"
    render "blocks/show", layout: "block"
  end
end
