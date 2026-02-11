import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
  static values = { type: { type: String, default: "single" } };

  get items() {
    return Array.from(this.element.querySelectorAll("[data-state]"));
  }

  connect() {
    this.element.addEventListener("click", this.handleClick.bind(this));
  }

  disconnect() {
    this.element.removeEventListener("click", this.handleClick.bind(this));
  }

  handleClick(event) {
    const item = event.target.closest("[data-state]");
    if (!item || !this.element.contains(item)) return;

    if (this.typeValue === "single") {
      this.items.forEach((el) => {
        if (el === item) {
          el.dataset.state = el.dataset.state === "on" ? "off" : "on";
        } else {
          el.dataset.state = "off";
        }
      });
    } else {
      item.dataset.state = item.dataset.state === "on" ? "off" : "on";
    }
  }
}
