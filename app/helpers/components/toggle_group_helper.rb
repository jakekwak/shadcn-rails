module Components::ToggleGroupHelper
  def render_toggle_group(type: :single, variant: :default, **options, &block)
    render "components/ui/toggle_group", type: type, variant: variant,
      content: capture(&block), options: options
  end
end
