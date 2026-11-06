import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach, vi } from "vitest";

// jsdom does not implement scrollIntoView, which ChatView calls after
// every new message. Provide a harmless stub for the test environment.
Object.defineProperty(Element.prototype, "scrollIntoView", {
  configurable: true,
  writable: true,
  value: vi.fn(),
});

// jsdom implements the File API but not File.text() in every version.
// The FileUploader reads the selected file with File.text(), so add a
// small polyfill backed by FileReader when the method is missing.
if (typeof File !== "undefined" && typeof File.prototype.text !== "function") {
  Object.defineProperty(File.prototype, "text", {
    configurable: true,
    writable: true,
    value: function text(this: File): Promise<string> {
      return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(String(reader.result ?? ""));
        reader.onerror = () => reject(reader.error);
        reader.readAsText(this);
      });
    },
  });
}

afterEach(() => {
  cleanup();
});
