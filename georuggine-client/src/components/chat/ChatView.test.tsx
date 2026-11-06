import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ChatView } from "./ChatView";
import { UiMessage } from "../../types/chat";

const messages: UiMessage[] = [
  {
    id: "msg-1",
    isSelf: false,
    senderName: "Server",
    text: "welcome",
    time: "10:00",
  },
  {
    id: "msg-2",
    isSelf: true,
    senderName: "alice",
    text: "hello",
    time: "10:01",
  },
];

function setup(overrides: Partial<React.ComponentProps<typeof ChatView>> = {}) {
  const onSendMessage = vi.fn().mockResolvedValue(undefined);
  const onLogout = vi.fn();
  const onClearChat = vi.fn();
  const props: React.ComponentProps<typeof ChatView> = {
    messages: [],
    currentUser: { username: "alice" },
    onSendMessage,
    onLogout,
    onClearChat,
    ...overrides,
  };

  const { container } = render(<ChatView {...props} />);

  return { onSendMessage, onLogout, onClearChat, user: userEvent.setup(), container };
}

describe("ChatView", () => {
  it("renders the empty state when there are no messages", () => {
    setup();

    expect(
      screen.getByText("No messages in this conversation"),
    ).toBeInTheDocument();
    expect(screen.getByText("Send a message to start chatting.")).toBeInTheDocument();
  });

  it("renders every message in order", () => {
    setup({ messages });

    const renderedMessages = screen.getAllByText(/welcome|hello/);
    expect(renderedMessages).toHaveLength(2);
    expect(screen.getByText("welcome")).toBeInTheDocument();
    expect(screen.getByText("hello")).toBeInTheDocument();
  });

  it("delegates message sending to the parent", async () => {
    const { onSendMessage, user } = setup();

    await user.type(screen.getByPlaceholderText("Type a message..."), "hello");
    await user.click(screen.getByTitle("Send message"));

    expect(onSendMessage).toHaveBeenCalledWith("hello");
  });

  it("exposes logout through the header menu", async () => {
    const { onLogout, user } = setup();

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Sign out"));

    expect(onLogout).toHaveBeenCalledTimes(1);
  });

  it("exposes clear chat through the header menu", async () => {
    const { onClearChat, user } = setup({ messages });

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Clear chat"));

    expect(onClearChat).toHaveBeenCalledTimes(1);
  });

  it("always renders the scroll anchor at the end of the list", () => {
    const { container } = setup({ messages });

    expect(container.querySelector("#chat-messages-end")).not.toBeNull();
  });
});
