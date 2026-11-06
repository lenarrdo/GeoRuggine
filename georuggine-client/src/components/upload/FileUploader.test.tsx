import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { FileUploader } from "./FileUploader";

function jsonFile(content = '[{"lat":1,"lon":2}]', name = "coordinates.json") {
  return new File([content], name, { type: "application/json" });
}

function fileInput(container: HTMLElement): HTMLInputElement {
  const input = container.querySelector<HTMLInputElement>('input[type="file"]');
  if (!input) {
    throw new Error("file input not found");
  }
  return input;
}

describe("FileUploader", () => {
  it("opens the native file picker when the choose button is clicked", async () => {
    const clickSpy = vi
      .spyOn(HTMLInputElement.prototype, "click")
      .mockImplementation(() => {});
    const user = userEvent.setup();
    const { container } = render(<FileUploader />);

    await user.click(screen.getByRole("button", { name: /Choose .json/ }));

    expect(clickSpy).toHaveBeenCalledTimes(1);
    expect(fileInput(container)).toHaveAttribute("accept", ".json");
  });

  it("rejects files that are not JSON", async () => {
    const { container } = render(<FileUploader />);

    fireEvent.change(fileInput(container), {
      target: { files: [new File(["hello"], "notes.txt", { type: "text/plain" })] },
    });

    expect(screen.getByText("Only .json files are allowed")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Choose .json/ })).toBeInTheDocument();
  });

  it("shows the selected JSON file and its size", async () => {
    const { container } = render(<FileUploader />);
    const file = jsonFile('{"lat":1,"lon":2}');

    fireEvent.change(fileInput(container), { target: { files: [file] } });

    expect(screen.getByText("coordinates.json")).toBeInTheDocument();
    expect(screen.getByText(`${(file.size / 1024).toFixed(1)} KB`)).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Upload coordinates" }),
    ).toBeInTheDocument();
  });

  it("reads the file and forwards its content to the parent", async () => {
    const onFileUploaded = vi.fn().mockResolvedValue(undefined);
    const user = userEvent.setup();
    const { container } = render(<FileUploader onFileUploaded={onFileUploaded} />);
    const content = '[{"lat":45.07,"lon":7.68}]';

    fireEvent.change(fileInput(container), {
      target: { files: [jsonFile(content)] },
    });
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(onFileUploaded).toHaveBeenCalledWith(content),
    );
    expect(screen.getByText("File uploaded successfully!")).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Upload coordinates" }),
    ).toBeEnabled();
  });

  it("shows the error thrown by the upload callback", async () => {
    const onFileUploaded = vi
      .fn()
      .mockRejectedValue("WebSocket is not connected to the server");
    const user = userEvent.setup();
    const { container } = render(<FileUploader onFileUploaded={onFileUploaded} />);

    fireEvent.change(fileInput(container), {
      target: { files: [jsonFile()] },
    });
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(
        screen.getByText("WebSocket is not connected to the server"),
      ).toBeInTheDocument(),
    );
    expect(screen.queryByText("File uploaded successfully!")).not.toBeInTheDocument();
  });

  it("normalizes a thrown Error into its message", async () => {
    const onFileUploaded = vi.fn().mockRejectedValue(new Error("network down"));
    const user = userEvent.setup();
    const { container } = render(<FileUploader onFileUploaded={onFileUploaded} />);

    fireEvent.change(fileInput(container), {
      target: { files: [jsonFile()] },
    });
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(screen.getByText("network down")).toBeInTheDocument(),
    );
  });

  it("falls back to a generic message for unknown upload errors", async () => {
    const onFileUploaded = vi.fn().mockRejectedValue({});
    const user = userEvent.setup();
    const { container } = render(<FileUploader onFileUploaded={onFileUploaded} />);

    fireEvent.change(fileInput(container), {
      target: { files: [jsonFile()] },
    });
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(screen.getByText("Upload failed")).toBeInTheDocument(),
    );
  });

  it("clears the selected file", async () => {
    const user = userEvent.setup();
    const { container } = render(<FileUploader />);

    fireEvent.change(fileInput(container), {
      target: { files: [jsonFile()] },
    });
    expect(screen.getByText("coordinates.json")).toBeInTheDocument();

    await user.click(screen.getByTitle("Remove file"));

    expect(screen.queryByText("coordinates.json")).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Choose .json/ })).toBeInTheDocument();
    expect(fileInput(container).value).toBe("");
  });

  it("shows the uploading state and disables the button while sending", async () => {
    let resolveUpload: (() => void) | undefined;
    const onFileUploaded = vi.fn(
      () =>
        new Promise<void>((resolve) => {
          resolveUpload = resolve;
        }),
    );
    const user = userEvent.setup();
    const { container } = render(<FileUploader onFileUploaded={onFileUploaded} />);

    fireEvent.change(fileInput(container), {
      target: { files: [jsonFile()] },
    });
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    expect(screen.getByText("Sending...")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Sending/ })).toBeDisabled();

    resolveUpload?.();
    await waitFor(() =>
      expect(screen.getByText("File uploaded successfully!")).toBeInTheDocument(),
    );
  });

  it("does nothing when upload is triggered without a selected file", async () => {
    const onFileUploaded = vi.fn();
    const { container } = render(<FileUploader onFileUploaded={onFileUploaded} />);

    // The upload button is not rendered until a file is selected.
    expect(
      screen.queryByRole("button", { name: "Upload coordinates" }),
    ).not.toBeInTheDocument();
    expect(onFileUploaded).not.toHaveBeenCalled();
    expect(fileInput(container).files).toHaveLength(0);
  });
});
