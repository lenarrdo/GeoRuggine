import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  apiGetCurrentUser,
  apiLogin,
  apiLogout,
  apiRegister,
  apiSendChatMessage,
  apiSendPosition,
  onChatMessageReceived,
  onWsDisconnected,
} from "./tauriBridge";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(),
}));

const invokeMock = vi.mocked(invoke);
const listenMock = vi.mocked(listen);

beforeEach(() => {
  invokeMock.mockReset();
  listenMock.mockReset();
});

describe("authentication bridge", () => {
  it("apiRegister forwards username and password to the Rust command", async () => {
    invokeMock.mockResolvedValueOnce({ success: true, message: "ok" });

    const result = await apiRegister("alice", "secret");

    expect(invokeMock).toHaveBeenCalledWith("register", {
      username: "alice",
      password: "secret",
    });
    expect(result).toEqual({ success: true, message: "ok" });
  });

  it("apiRegister converts a string rejection into a failed AuthResult", async () => {
    invokeMock.mockRejectedValueOnce("Username already taken.");

    await expect(apiRegister("alice", "secret")).resolves.toEqual({
      success: false,
      message: "Username already taken.",
    });
  });

  it("apiRegister converts an Error rejection into a failed AuthResult", async () => {
    invokeMock.mockRejectedValueOnce(new Error("IPC unavailable"));

    await expect(apiRegister("alice", "secret")).resolves.toEqual({
      success: false,
      message: "IPC unavailable",
    });
  });

  it("apiRegister falls back to a generic message for unknown rejections", async () => {
    invokeMock.mockRejectedValueOnce({ unexpected: true });

    await expect(apiRegister("alice", "secret")).resolves.toEqual({
      success: false,
      message: "Registration failed.",
    });
  });

  it("apiLogin forwards credentials and returns the backend result", async () => {
    invokeMock.mockResolvedValueOnce({ success: true, message: "Welcome" });

    const result = await apiLogin("bob", "hunter2");

    expect(invokeMock).toHaveBeenCalledWith("login", {
      username: "bob",
      password: "hunter2",
    });
    expect(result).toEqual({ success: true, message: "Welcome" });
  });

  it("apiLogin converts a string rejection into a failed AuthResult", async () => {
    invokeMock.mockRejectedValueOnce("Invalid credentials.");

    await expect(apiLogin("bob", "wrong")).resolves.toEqual({
      success: false,
      message: "Invalid credentials.",
    });
  });

  it("apiLogin falls back to a generic message for empty rejections", async () => {
    invokeMock.mockRejectedValueOnce("   ");

    await expect(apiLogin("bob", "wrong")).resolves.toEqual({
      success: false,
      message: "Login failed.",
    });
  });

  it("apiLogout invokes the logout command", async () => {
    invokeMock.mockResolvedValueOnce(undefined);

    await expect(apiLogout()).resolves.toBeUndefined();
    expect(invokeMock).toHaveBeenCalledWith("logout");
  });

  it("apiLogout rejects with a normalized Error", async () => {
    invokeMock.mockRejectedValueOnce("session error");

    await expect(apiLogout()).rejects.toThrow("session error");
  });

  it("apiGetCurrentUser maps the username returned by the backend", async () => {
    invokeMock.mockResolvedValueOnce("alice");

    await expect(apiGetCurrentUser()).resolves.toEqual({ username: "alice" });
    expect(invokeMock).toHaveBeenCalledWith("get_current_user");
  });

  it("apiGetCurrentUser returns null when no session exists", async () => {
    invokeMock.mockResolvedValueOnce(null);

    await expect(apiGetCurrentUser()).resolves.toBeNull();
  });

  it("apiGetCurrentUser returns null when the backend call fails", async () => {
    invokeMock.mockRejectedValueOnce(new Error("boom"));

    await expect(apiGetCurrentUser()).resolves.toBeNull();
  });
});

describe("chat bridge", () => {
  it("apiSendChatMessage sends the contents as a Tauri argument", async () => {
    invokeMock.mockResolvedValueOnce(undefined);

    await apiSendChatMessage("hello");

    expect(invokeMock).toHaveBeenCalledWith("send_chat_message", {
      contents: "hello",
    });
  });

  it("apiSendChatMessage wraps backend failures in an Error", async () => {
    invokeMock.mockRejectedValueOnce("Message cannot be empty.");

    await expect(apiSendChatMessage("hello")).rejects.toThrow(
      "Message cannot be empty.",
    );
  });

  it("apiSendChatMessage uses the fallback message for unknown errors", async () => {
    invokeMock.mockRejectedValueOnce(undefined);

    await expect(apiSendChatMessage("hello")).rejects.toThrow(
      "Unable to send the message.",
    );
  });

  it("onChatMessageReceived subscribes to the correct event and forwards payloads", async () => {
    const unlisten = vi.fn();
    let capturedHandler:
      | ((event: { payload: { timestamp: string; contents: string } }) => void)
      | undefined;
    listenMock.mockImplementationOnce(async (_event, handler) => {
      capturedHandler = handler as typeof capturedHandler;
      return unlisten;
    });

    const callback = vi.fn();
    const result = await onChatMessageReceived(callback);

    expect(listenMock).toHaveBeenCalledWith(
      "chat-message-received",
      expect.any(Function),
    );
    expect(result).toBe(unlisten);

    capturedHandler?.({
      payload: { timestamp: "2026-01-01T10:00:00Z", contents: "hi" },
    });
    expect(callback).toHaveBeenCalledWith({
      timestamp: "2026-01-01T10:00:00Z",
      contents: "hi",
    });
  });

  it("onChatMessageReceived surfaces listener registration failures", async () => {
    listenMock.mockRejectedValueOnce("no ipc");

    await expect(onChatMessageReceived(vi.fn())).rejects.toThrow("no ipc");
  });

  it("onWsDisconnected subscribes to the disconnect event", async () => {
    const unlisten = vi.fn();
    let capturedHandler: (() => void) | undefined;
    listenMock.mockImplementationOnce(async (_event, handler) => {
      capturedHandler = handler as () => void;
      return unlisten;
    });

    const callback = vi.fn();
    const result = await onWsDisconnected(callback);

    expect(listenMock).toHaveBeenCalledWith("ws-disconnected", expect.any(Function));
    expect(result).toBe(unlisten);

    capturedHandler?.();
    expect(callback).toHaveBeenCalledTimes(1);
  });

  it("onWsDisconnected surfaces listener registration failures", async () => {
    listenMock.mockRejectedValueOnce(new Error("no event bus"));

    await expect(onWsDisconnected(vi.fn())).rejects.toThrow("no event bus");
  });
});

describe("position bridge", () => {
  it("apiSendPosition invokes upload_coordinates with the file content", async () => {
    invokeMock.mockResolvedValueOnce(undefined);
    const content = '[{"lat":1,"lon":2}]';

    await apiSendPosition(content);

    expect(invokeMock).toHaveBeenCalledWith("upload_coordinates", { content });
  });

  it("apiSendPosition wraps backend failures in an Error", async () => {
    invokeMock.mockRejectedValueOnce("The file is empty.");

    await expect(apiSendPosition("[]")).rejects.toThrow("The file is empty.");
  });

  it("apiSendPosition uses the fallback message for unknown errors", async () => {
    invokeMock.mockRejectedValueOnce(null);

    await expect(apiSendPosition("[]")).rejects.toThrow(
      "Unable to send the position.",
    );
  });
});
