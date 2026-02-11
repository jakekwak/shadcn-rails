module Components::BreadcrumbHelper
  def render_breadcrumb(items: [], **options)
    render "components/ui/breadcrumb", items: items, options: options
  end
end
