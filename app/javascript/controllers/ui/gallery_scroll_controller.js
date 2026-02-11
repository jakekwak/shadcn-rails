import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
  static targets = ["track"];

  prev() {
    const track = this.hasTrackTarget ? this.trackTarget : this.element.closest("section")?.querySelector("[data-ui--gallery-scroll-target='track']");
    if (track) track.scrollBy({ left: -460, behavior: "smooth" });
  }

  next() {
    const track = this.hasTrackTarget ? this.trackTarget : this.element.closest("section")?.querySelector("[data-ui--gallery-scroll-target='track']");
    if (track) track.scrollBy({ left: 460, behavior: "smooth" });
  }
}
