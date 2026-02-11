module Components::SpinnerHelper
  def render_spinner(size: :default, **options)
    size_classes = case size.to_sym
    when :sm
      "size-3"
    when :default
      "size-4"
    when :lg
      "size-6"
    end

    spinner_classes = tw("animate-spin text-muted-foreground", size_classes, options[:class])

    content_tag :svg, role: "status", aria: {label: "Loading"},
      class: spinner_classes, xmlns: "http://www.w3.org/2000/svg",
      width: "24", height: "24", viewBox: "0 0 24 24", fill: "none",
      stroke: "currentColor", stroke_width: "2", stroke_linecap: "round",
      stroke_linejoin: "round", **options.except(:class) do
      content_tag(:path, nil, d: "M21 12a9 9 0 1 1-6.219-8.56")
    end
  end
end
