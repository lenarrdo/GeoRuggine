import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { ChatMessageItem } from "./ChatMessageItem";
import { UiMessage } from "../../types/chat";

function message(overrides: Partial<UiMessage> = {}): UiMessage {
  return {
    id: "msg-1",
    isSelf: false,
    senderName: "Server",
    text: "hello",
    time: "10:30",
    ...overrides,
  };
}

describe("ChatMessageItem", () => {
  it("renders a message coming from the server", () => {
    render(<ChatMessageItem message={message()} />);

    expect(screen.getByText("hello")).toBeInTheDocument();
    expect(screen.getByText("Server")).toBeInTheDocument();
    expect(screen.getByText("10:30")).toBeInTheDocument();
    expect(screen.getByText("CENTRAL")).toBeInTheDocument();
  });

  it("falls back to 'Server' when the sender name is missing", () => {
    render(<ChatMessageItem message={message({ senderName: "" })} />);

    expect(screen.getByText("Server")).toBeInTheDocument();
  });

  it("renders a message sent by the current user", () => {
    render(
      <ChatMessageItem
        message={message({ isSelf: true, senderName: "alice", text: "my message" })}
      />,
    );

    expect(screen.getByText("my message")).toBeInTheDocument();
    expect(screen.getByText("alice")).toBeInTheDocument();
    expect(screen.queryByText("CENTRAL")).not.toBeInTheDocument();
  });

  it("falls back to 'You' when a self message has no sender name", () => {
    render(
      <ChatMessageItem
        message={message({ isSelf: true, senderName: "", text: "my message" })}
      />,
    );

    expect(screen.getByText("You")).toBeInTheDocument();
  });

  it("shows the failure marker for a message that could not be sent", () => {
    render(<ChatMessageItem message={message({ isSelf: true, failed: true })} />);

    expect(screen.getByText("Not sent")).toBeInTheDocument();
  });

  it("does not show the failure marker for successfully sent messages", () => {
    render(<ChatMessageItem message={message({ isSelf: true })} />);

    expect(screen.queryByText("Not sent")).not.toBeInTheDocument();
  });
});
