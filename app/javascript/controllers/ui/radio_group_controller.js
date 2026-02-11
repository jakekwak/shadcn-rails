import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
  static targets = ["radio"];
  static values = {
    name: String,
    value: { type: String, default: "" },
  };

  connect() {
    this.radioTargets.forEach((radio) => {
      radio.name = this.nameValue;
      if (this.valueValue && radio.value === this.valueValue) {
        radio.checked = true;
      }
    });

    this.element.addEventListener("keydown", this.handleKeydown.bind(this));
  }

  disconnect() {
    this.element.removeEventListener("keydown", this.handleKeydown.bind(this));
  }

  select(event) {
    this.valueValue = event.target.value;
    this.dispatch("change", { detail: { value: this.valueValue } });
  }

  handleKeydown(event) {
    const radios = this.radioTargets;
    const currentIndex = radios.findIndex((r) => r === document.activeElement || r.checked);

    let nextIndex;
    switch (event.key) {
      case "ArrowDown":
      case "ArrowRight":
        event.preventDefault();
        nextIndex = (currentIndex + 1) % radios.length;
        break;
      case "ArrowUp":
      case "ArrowLeft":
        event.preventDefault();
        nextIndex = (currentIndex - 1 + radios.length) % radios.length;
        break;
      default:
        return;
    }

    radios[nextIndex].checked = true;
    radios[nextIndex].focus();
    radios[nextIndex].dispatchEvent(new Event("change", { bubbles: true }));
  }
}
