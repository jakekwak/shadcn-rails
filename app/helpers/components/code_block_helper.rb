module Components::CodeBlockHelper
  def render_code_block(**options, &block)
    @_code_items = []
    capture(&block)
    render "components/ui/code_block", code_items: @_code_items, options: options
  end

  def code_block_item(language:, filename:, code:, active: false, **options)
    formatter = Rouge::Formatters::HTML.new
    lexer = Rouge::Lexer.find(language) || Rouge::Lexers::PlainText.new
    highlighted = formatter.format(lexer.lex(code))

    @_code_items << {
      language: language,
      filename: filename,
      code: code,
      highlighted: highlighted,
      active: active,
      options: options
    }
    nil
  end
end
