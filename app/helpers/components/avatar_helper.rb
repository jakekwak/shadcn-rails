module Components::AvatarHelper
  def render_avatar(src: nil, alt: "", fallback: nil, size: :default, **options)
    size_classes = case size.to_sym
    when :sm
      "h-8 w-8"
    when :default
      "h-10 w-10"
    when :lg
      "h-14 w-14"
    end

    container_classes = tw("relative flex shrink-0 overflow-hidden rounded-full", size_classes, options[:class])

    content_tag :span, class: container_classes, **options.except(:class) do
      if src.present?
        image_tag(src, alt: alt, class: "aspect-square h-full w-full object-cover") +
          (fallback ? avatar_fallback(fallback) : "".html_safe)
      elsif fallback.present?
        avatar_fallback(fallback)
      end
    end
  end

  def render_avatar_group(**options, &block)
    group_classes = tw("flex -space-x-4", options[:class])
    content_tag :div, class: group_classes, **options.except(:class), &block
  end

  private

  def avatar_fallback(text)
    content_tag :span, text, class: "flex h-full w-full items-center justify-center rounded-full bg-muted text-sm font-medium"
  end
end
