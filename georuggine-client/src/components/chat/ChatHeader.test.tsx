import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { ChatHeader } from "./ChatHeader";

describe("ChatHeader", () => {
  it("renders the server name and keeps the menu closed by default", () => {
    render(<ChatHeader currentUser={{ username: "alice" }} onLogout={vi.fn()} />);

    expect(screen.getByText("Server")).toBeInTheDocument();
    expect(screen.queryByText("alice")).not.toBeInTheDocument();
    expect(screen.queryByText("Sign out")).not.toBeInTheDocument();
  });

  it("opens the options menu with the username and actions", async () => {
    const user = userEvent.setup();
    render(
      <ChatHeader
        currentUser={{ username: "alice" }}
        onLogout={vi.fn()}
        onClearChat={vi.fn()}
      />,
    );

    await user.click(screen.getByTitle("Options"));

    expect(screen.getByText("alice")).toBeInTheDocument();
    expect(screen.getByText("Clear chat")).toBeInTheDocument();
    expect(screen.getByText("Sign out")).toBeInTheDocument();
  });

  it("calls onClearChat and closes the menu", async () => {
    const onClearChat = vi.fn();
    const user = userEvent.setup();
    render(
      <ChatHeader currentUser={{ username: "alice" }} onLogout={vi.fn()} onClearChat={onClearChat} />,
    );

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Clear chat"));

    expect(onClearChat).toHaveBeenCalledTimes(1);
    expect(screen.queryByText("Clear chat")).not.toBeInTheDocument();
  });

  it("calls onLogout and closes the menu", async () => {
    const onLogout = vi.fn();
    const user = userEvent.setup();
    render(<ChatHeader currentUser={{ username: "alice" }} onLogout={onLogout} />);

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Sign out"));

    expect(onLogout).toHaveBeenCalledTimes(1);
    expect(screen.queryByText("Sign out")).not.toBeInTheDocument();
  });

  it("does not render the clear action when no handler is provided", async () => {
    const user = userEvent.setup();
    render(<ChatHeader currentUser={{ username: "alice" }} onLogout={vi.fn()} />);

    await user.click(screen.getByTitle("Options"));

    expect(screen.queryByText("Clear chat")).not.toBeInTheDocument();
  });

  it("closes the menu when the backdrop is clicked", async () => {
    const user = userEvent.setup();
    const { container } = render(
      <ChatHeader currentUser={{ username: "alice" }} onLogout={vi.fn()} />,
    );

    await user.click(screen.getByTitle("Options"));
    expect(screen.getByText("Sign out")).toBeInTheDocument();

    const backdrop = container.querySelector(".dropdown-backdrop");
    expect(backdrop).not.toBeNull();
    await user.click(backdrop!);

    expect(screen.queryByText("Sign out")).not.toBeInTheDocument();
  });

  it("toggles the menu closed when options is clicked again", async () => {
    const user = userEvent.setup();
    render(<ChatHeader currentUser={{ username: "alice" }} onLogout={vi.fn()} />);

    await user.click(screen.getByTitle("Options"));
    expect(screen.getByText("Sign out")).toBeInTheDocument();

    await user.click(screen.getByTitle("Options"));
    expect(screen.queryByText("Sign out")).not.toBeInTheDocument();
  });

  it("hides the username entry when there is no current user", async () => {
    const user = userEvent.setup();
    render(<ChatHeader currentUser={null} onLogout={vi.fn()} />);

    await user.click(screen.getByTitle("Options"));

    expect(screen.queryByText("alice")).not.toBeInTheDocument();
    expect(screen.getByText("Sign out")).toBeInTheDocument();
  });
});
