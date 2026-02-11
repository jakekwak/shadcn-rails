import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
  static targets = ["tab", "panel", "copyButton", "copyIcon", "checkIcon"];

  connect() {
    if (!this.tabTargets.length) return;
    const activeIndex = this.tabTargets.findIndex(
      (tab) => tab.dataset.state === "active"
    );
    this._index = activeIndex >= 0 ? activeIndex : 0;
  }

  change(event) {
    const index = parseInt(event.currentTarget.dataset.index);
    if (!isNaN(index)) {
      this._index = index;
      this.showTab();
    }
  }

  copy() {
    const activePanel = this.panelTargets[this._index];
    if (!activePanel) return;

    const code = activePanel.dataset.code;
    navigator.clipboard.writeText(code).then(() => {
      this.copyIconTarget.classList.add("hidden");
      this.checkIconTarget.classList.remove("hidden");

      clearTimeout(this._copyTimeout);
      this._copyTimeout = setTimeout(() => {
        this.copyIconTarget.classList.remove("hidden");
        this.checkIconTarget.classList.add("hidden");
      }, 2000);
    });
  }

  showTab() {
    this.tabTargets.forEach((tab, index) => {
      const panel = this.panelTargets[index];
      if (index === this._index) {
        tab.dataset.state = "active";
        panel.dataset.state = "active";
      } else {
        tab.dataset.state = "inactive";
        panel.dataset.state = "inactive";
      }
    });
  }
}
