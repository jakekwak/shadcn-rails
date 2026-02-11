module Components::KbdHelper
  def render_kbd(key, **options)
    kbd_classes = tw(
      "pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium opacity-100",
      options[:class]
    )

    content_tag :kbd, key, class: kbd_classes, **options.except(:class)
  end

  def render_kbd_group(**options, &block)
    group_classes = tw("inline-flex items-center gap-1", options[:class])
    content_tag :span, class: group_classes, **options.except(:class), &block
  end
end
