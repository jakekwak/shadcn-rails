module Components::RadioGroupHelper
  def render_radio_group(name:, value: nil, **options, &block)
    render "components/ui/radio_group", name: name, value: value,
      content: capture(&block), options: options
  end

  def render_radio_group_item(value:, label: nil, id: nil, **options)
    item_id = id || "radio-#{value}"
    checked = options.delete(:checked) || false

    content_tag :div, class: "flex items-center space-x-2" do
      radio_input(name: nil, value: value, id: item_id, checked: checked, **options) +
        (label ? content_tag(:label, label, for: item_id, class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70") : "".html_safe)
    end
  end

  private

  def radio_input(name:, value:, id:, checked: false, **options)
    input_classes = tw("peer sr-only", options[:input_class])

    content_tag(:input, nil, type: "radio", id: id, value: value, class: input_classes,
      checked: checked || nil,
      data: {action: "change->ui--radio-group#select", ui__radio_group_target: "radio"}) +
      content_tag(:label, for: id,
        class: "aspect-square h-4 w-4 rounded-full border border-primary text-primary ring-offset-background focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 flex items-center justify-center cursor-pointer peer-checked:[&>span]:visible") do
        content_tag(:span, nil, class: "invisible h-2.5 w-2.5 rounded-full bg-current")
      end
  end
end
