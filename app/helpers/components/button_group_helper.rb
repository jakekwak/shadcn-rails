module Components::ButtonGroupHelper
  def render_button_group(orientation: :horizontal, **options, &block)
    render "components/ui/button_group", orientation: orientation, content: capture(&block), options: options
  end
end
