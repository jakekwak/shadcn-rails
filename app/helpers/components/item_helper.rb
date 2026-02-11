module Components::ItemHelper
  def render_item(title:, description: nil, variant: :default, size: :default, **options, &block)
    render "components/ui/item", title: title, description: description, variant: variant,
      size: size, content: (block ? capture(&block) : nil), options: options
  end

  def item_media(**options, &block)
    media_classes = tw("flex shrink-0 items-center justify-center", options[:class])
    content_tag :div, class: media_classes, &block
  end

  def item_content(**options, &block)
    content_classes = tw("flex min-w-0 flex-1 flex-col", options[:class])
    content_tag :div, class: content_classes, &block
  end

  def item_actions(**options, &block)
    actions_classes = tw("flex shrink-0 items-center gap-2", options[:class])
    content_tag :div, class: actions_classes, &block
  end
end
