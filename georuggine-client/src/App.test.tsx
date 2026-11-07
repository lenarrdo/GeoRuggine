import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import * as bridge from "./services/tauriBridge";
import { ChatMessage } from "./types/chat";

vi.mock("./services/tauriBridge", () => ({
  apiLogin: vi.fn(),
  apiRegister: vi.fn(),
  apiLogout: vi.fn(),
  apiSendChatMessage: vi.fn(),
  onChatMessageReceived: vi.fn(),
  onWsDisconnected: vi.fn(),
}));

const apiLogin = vi.mocked(bridge.apiLogin);
const apiRegister = vi.mocked(bridge.apiRegister);
const apiLogout = vi.mocked(bridge.apiLogout);
const apiSendChatMessage = vi.mocked(bridge.apiSendChatMessage);
const onChatMessageReceived = vi.mocked(bridge.onChatMessageReceived);
const onWsDisconnected = vi.mocked(bridge.onWsDisconnected);

let chatHandler: ((msg: ChatMessage) => void) | undefined;
let disconnectHandler: (() => void) | undefined;
let unlistenChat: () => void;
let unlistenDisconnect: () => void;

beforeEach(() => {
  chatHandler = undefined;
  disconnectHandler = undefined;
  unlistenChat = vi.fn();
  unlistenDisconnect = vi.fn();

  apiLogin.mockResolvedValue({ success: true, message: "Welcome" });
  apiRegister.mockResolvedValue({ success: true, message: "Registered" });
  apiLogout.mockResolvedValue(undefined);
  apiSendChatMessage.mockResolvedValue(undefined);

  onChatMessageReceived.mockImplementation(async (callback) => {
    chatHandler = callback;
    return unlistenChat;
  });
  onWsDisconnected.mockImplementation(async (callback) => {
    disconnectHandler = callback;
    return unlistenDisconnect;
  });
});

async function login(user: ReturnType<typeof userEvent.setup>, username = "alice") {
  await user.type(screen.getByLabelText("Username"), username);
  await user.type(screen.getByLabelText("Password"), "secret");
  await user.click(screen.getByRole("button", { name: "Sign in" }));
  await waitFor(() => expect(screen.queryByText("Sign in to your account")).not.toBeInTheDocument());
}

async function fillRegistration(
  user: ReturnType<typeof userEvent.setup>,
  password = "secret",
  confirmPassword = "secret",
) {
  await user.type(screen.getByLabelText("Username"), "alice");
  await user.type(screen.getByLabelText("Password"), password);
  await user.type(screen.getByLabelText("Confirm password"), confirmPassword);
  await user.click(screen.getByRole("button", { name: "Sign up" }));
}

describe("App authentication flow", () => {
  it("starts on the login screen", () => {
    render(<App />);

    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
    expect(screen.queryByText("No messages in this conversation")).not.toBeInTheDocument();
  });

  it("navigates between login and registration", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("button", { name: "Sign up" }));
    expect(screen.getByText("Create your account")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Sign in" }));
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
  });

  it("returns to login with a success message after a successful registration", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("button", { name: "Sign up" }));
    await fillRegistration(user);

    await waitFor(() =>
      expect(
        screen.getByText("Registration successful. You can now sign in."),
      ).toBeInTheDocument(),
    );
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
    expect(apiRegister).toHaveBeenCalledWith("alice", "secret");
  });

  it("stays on the registration screen when the backend rejects the account", async () => {
    apiRegister.mockResolvedValueOnce({
      success: false,
      message: "Username already taken.",
    });
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("button", { name: "Sign up" }));
    await fillRegistration(user);

    await waitFor(() =>
      expect(screen.getByText("Username already taken.")).toBeInTheDocument(),
    );
    expect(screen.getByText("Create your account")).toBeInTheDocument();
  });

  it("shows the login error returned by the backend", async () => {
    apiLogin.mockResolvedValueOnce({
      success: false,
      message: "Invalid credentials.",
    });
    const user = userEvent.setup();
    render(<App />);

    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "wrong");
    await user.click(screen.getByRole("button", { name: "Sign in" }));

    await waitFor(() =>
      expect(screen.getByText("Invalid credentials.")).toBeInTheDocument(),
    );
  });

  it("shows a normalized error when the login bridge throws a string", async () => {
    apiLogin.mockRejectedValueOnce("IPC failure");
    const user = userEvent.setup();
    render(<App />);

    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "wrong");
    await user.click(screen.getByRole("button", { name: "Sign in" }));

    await waitFor(() =>
      expect(screen.getByText("IPC failure")).toBeInTheDocument(),
    );
  });

  it("enters the chat after a successful login and registers listeners", async () => {
    const user = userEvent.setup();
    render(<App />);

    await login(user);

    expect(apiLogin).toHaveBeenCalledWith("alice", "secret");
    expect(screen.getByText("No messages in this conversation")).toBeInTheDocument();
    await waitFor(() => {
      expect(onChatMessageReceived).toHaveBeenCalledTimes(1);
      expect(onWsDisconnected).toHaveBeenCalledTimes(1);
    });
  });
});

describe("App chat flow", () => {
  it("appends messages received from the backend", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);
    await waitFor(() => expect(chatHandler).toBeDefined());

    act(() => {
      chatHandler?.({
        timestamp: "2026-01-01T10:00:00Z",
        contents: "hello from the server",
      });
    });

    expect(screen.getByText("hello from the server")).toBeInTheDocument();
    expect(screen.queryByText("No messages in this conversation")).not.toBeInTheDocument();
  });

  it("ignores duplicated messages with the same text and time", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);
    await waitFor(() => expect(chatHandler).toBeDefined());

    const payload = {
      timestamp: "2026-01-01T10:00:00Z",
      contents: "only once",
    };
    act(() => {
      chatHandler?.(payload);
      chatHandler?.(payload);
    });

    expect(screen.getAllByText("only once")).toHaveLength(1);
  });

  it("accepts two messages with the same text but different timestamps", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);
    await waitFor(() => expect(chatHandler).toBeDefined());

    act(() => {
      chatHandler?.({ timestamp: "2026-01-01T10:00:00Z", contents: "same" });
      chatHandler?.({ timestamp: "2026-01-01T10:05:00Z", contents: "same" });
    });

    expect(screen.getAllByText("same")).toHaveLength(2);
  });

  it("sends a message and renders it immediately", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);

    await user.type(screen.getByPlaceholderText("Type a message..."), "hi there");
    await user.click(screen.getByTitle("Send message"));

    await waitFor(() => expect(apiSendChatMessage).toHaveBeenCalledWith("hi there"));
    expect(screen.getByText("hi there")).toBeInTheDocument();
    expect(screen.getByText("alice")).toBeInTheDocument();
    expect(screen.getByPlaceholderText("Type a message...")).toHaveValue("");
  });

  it("marks a message as failed and restores the text when sending fails", async () => {
    apiSendChatMessage.mockRejectedValueOnce(new Error("offline"));
    const user = userEvent.setup();
    render(<App />);
    await login(user);

    await user.type(screen.getByPlaceholderText("Type a message..."), "hi there");
    await user.click(screen.getByTitle("Send message"));

    await waitFor(() => expect(screen.getByText("Not sent")).toBeInTheDocument());
    expect(screen.getByPlaceholderText("Type a message...")).toHaveValue("hi there");
  });

  it("clears the conversation from the header menu", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);
    await waitFor(() => expect(chatHandler).toBeDefined());

    act(() => {
      chatHandler?.({ timestamp: "2026-01-01T10:00:00Z", contents: "to clear" });
    });
    expect(screen.getByText("to clear")).toBeInTheDocument();

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Clear chat"));

    expect(screen.queryByText("to clear")).not.toBeInTheDocument();
    expect(screen.getByText("No messages in this conversation")).toBeInTheDocument();
  });

  it("logs out from the header menu and detaches the listeners", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);
    await waitFor(() => expect(chatHandler).toBeDefined());

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Logout"));

    await waitFor(() => expect(apiLogout).toHaveBeenCalledTimes(1));
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
    expect(unlistenChat).toHaveBeenCalledTimes(1);
    expect(unlistenDisconnect).toHaveBeenCalledTimes(1);
  });
});

describe("App connection failure flow", () => {
  it("shows the server problem modal when the WebSocket disconnects", async () => {
    const user = userEvent.setup();
    render(<App />);
    await login(user);
    await waitFor(() => expect(disconnectHandler).toBeDefined());

    act(() => {
      disconnectHandler?.();
    });

    expect(
      screen.getByText(/There was a problem with the server/),
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Back to login" }));

    await waitFor(() => expect(apiLogout).toHaveBeenCalledTimes(1));
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
  });

  it("shows the server problem modal when listener registration fails", async () => {
    onWsDisconnected.mockRejectedValueOnce(new Error("event bus unavailable"));
    const user = userEvent.setup();
    render(<App />);

    await login(user);

    await waitFor(() =>
      expect(
        screen.getByText(/There was a problem with the server/),
      ).toBeInTheDocument(),
    );
  });
});
