import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
  static targets = ["section", "link"];

  connect() {
    this.observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            this.activate(entry.target.id);
          }
        });
      },
      { root: null, rootMargin: "0px", threshold: 1 }
    );

    this.sectionTargets.forEach((section) => {
      this.observer.observe(section);
    });
  }

  disconnect() {
    this.observer?.disconnect();
  }

  activate(sectionId) {
    this.linkTargets.forEach((link) => {
      if (link.getAttribute("href") === `#${sectionId}`) {
        link.classList.remove("text-muted-foreground");
        link.classList.add("text-primary", "font-medium");
      } else {
        link.classList.add("text-muted-foreground");
        link.classList.remove("text-primary", "font-medium");
      }
    });
  }
}
