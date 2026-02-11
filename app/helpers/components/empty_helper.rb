module Components::EmptyHelper
  def render_empty(title:, description: nil, icon: nil, **options, &block)
    render "components/ui/empty", title: title, description: description,
      icon: icon, content: (block ? capture(&block) : nil), options: options
  end
end
