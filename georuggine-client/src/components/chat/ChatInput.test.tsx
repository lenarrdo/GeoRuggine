import { describe, expect, it, vi } from "vitest";
import { act, fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ChatInput } from "./ChatInput";

describe("ChatInput", () => {
  it("keeps the send button disabled while the input is empty", async () => {
    const onSendMessage = vi.fn().mockResolvedValue(undefined);
    render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();

    expect(screen.getByTitle("Send message")).toBeDisabled();

    await user.type(screen.getByPlaceholderText("Type a message..."), "   ");
    expect(screen.getByTitle("Send message")).toBeDisabled();
  });

  it("sends a trimmed message and clears the input", async () => {
    const onSendMessage = vi.fn().mockResolvedValue(undefined);
    render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();
    const input = screen.getByPlaceholderText("Type a message...");

    await user.type(input, "  hello world  ");
    await user.click(screen.getByTitle("Send message"));

    expect(onSendMessage).toHaveBeenCalledWith("hello world");
    expect(input).toHaveValue("");
  });

  it("sends when Enter is pressed", async () => {
    const onSendMessage = vi.fn().mockResolvedValue(undefined);
    render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();

    await user.type(
      screen.getByPlaceholderText("Type a message..."),
      "hello{Enter}",
    );

    expect(onSendMessage).toHaveBeenCalledWith("hello");
  });

  it("does not send when Shift+Enter is pressed", async () => {
    const onSendMessage = vi.fn().mockResolvedValue(undefined);
    render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();

    await user.type(
      screen.getByPlaceholderText("Type a message..."),
      "hello{Shift>}{Enter}{/Shift}",
    );

    expect(onSendMessage).not.toHaveBeenCalled();
  });

  it("submits through the form submit event", async () => {
    const onSendMessage = vi.fn().mockResolvedValue(undefined);
    const { container } = render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();

    await user.type(screen.getByPlaceholderText("Type a message..."), "hello");
    const form = container.querySelector("form");
    expect(form).not.toBeNull();
    await act(async () => {
      fireEvent.submit(form!);
    });

    expect(onSendMessage).toHaveBeenCalledWith("hello");
  });

  it("restores the text when sending fails", async () => {
    const onSendMessage = vi.fn().mockRejectedValue(new Error("offline"));
    render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();
    const input = screen.getByPlaceholderText("Type a message...");

    await user.type(input, "hello");
    await user.click(screen.getByTitle("Send message"));

    expect(onSendMessage).toHaveBeenCalledWith("hello");
    expect(input).toHaveValue("hello");
  });

  it("disables the input and button through the disabled prop", () => {
    render(<ChatInput onSendMessage={vi.fn()} disabled />);

    expect(screen.getByPlaceholderText("Type a message...")).toBeDisabled();
    expect(screen.getByTitle("Send message")).toBeDisabled();
  });

  it("ignores a second send while the first one is still in flight", async () => {
    let resolveSend: (() => void) | undefined;
    const onSendMessage = vi.fn(
      () =>
        new Promise<void>((resolve) => {
          resolveSend = resolve;
        }),
    );
    render(<ChatInput onSendMessage={onSendMessage} />);
    const user = userEvent.setup();

    await user.type(screen.getByPlaceholderText("Type a message..."), "hello");
    await user.click(screen.getByTitle("Send message"));

    expect(onSendMessage).toHaveBeenCalledTimes(1);
    expect(screen.getByPlaceholderText("Type a message...")).toBeDisabled();

    await act(async () => {
      resolveSend?.();
    });
  });
});
