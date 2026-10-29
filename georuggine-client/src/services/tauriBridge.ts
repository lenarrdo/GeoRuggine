import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AuthResult, UserSession } from "../types/auth";
import { ChatMessage } from "../types/chat";

/** Normalizes the error returned by Tauri (string or Error) into a readable message */
function toErrorMessage(err: unknown, fallback: string): string {
  if (typeof err === "string" && err.trim().length > 0) {
    return err;
  }
  if (err instanceof Error && err.message.trim().length > 0) {
    return err.message;
  }
  return fallback;
}

/**
 * Authentication: new account registration
 */
export async function apiRegister(username: string, password: string): Promise<AuthResult> {
  try {
    return await invoke<AuthResult>("register", { username, password });
  } catch (err) {
    return {
      success: false,
      message: toErrorMessage(err, "Registration failed."),
    };
  }
}

/**
 * Authentication: login
 */
export async function apiLogin(username: string, password: string): Promise<AuthResult> {
  try {
    return await invoke<AuthResult>("login", { username, password });
  } catch (err) {
    return {
      success: false,
      message: toErrorMessage(err, "Login failed."),
    };
  }
}

/**
 * Authentication: logout
 */
export async function apiLogout(): Promise<void> {
  try {
    await invoke("logout");
  } catch (err) {
    throw new Error(toErrorMessage(err, "Logout failed."));
  }
}

/**
 * Authentication: retrieve the current user stored in the session
 */
export async function apiGetCurrentUser(): Promise<UserSession | null> {
  try {
    const username = await invoke<string | null>("get_current_user");
    return username ? { username } : null;
  } catch {
    return null;
  }
}

/**
 * Chat: send a text message to the server or to a recipient
 */
export async function apiSendChatMessage(contents: string): Promise<void> {
  try {
    await invoke("send_chat_message", { contents });
  } catch (err) {
    throw new Error(toErrorMessage(err, "Unable to send the message."));
  }
}

/**
 * WebSocket events: listen for chat messages received from the server
 */
export async function onChatMessageReceived(
  callback: (msg: ChatMessage) => void,
): Promise<UnlistenFn> {
  try {
    return await listen<ChatMessage>("chat-message-received", (event) => {
      callback(event.payload);
    });
  } catch (err) {
    throw new Error(
      toErrorMessage(err, "Unable to listen for messages from the server."),
    );
  }
}

/**
 * WebSocket events: listen for unexpected disconnections from the server
 */
export async function onWsDisconnected(callback: () => void): Promise<UnlistenFn> {
  try {
    return await listen("ws-disconnected", () => {
      callback();
    });
  } catch (err) {
    throw new Error(
      toErrorMessage(err, "Unable to listen for the server status."),
    );
  }
}

/**
 * Position: send coordinates to the server
 */
export async function apiSendPosition(content: string): Promise<void> {
  try {
    await invoke("upload_coordinates", { content });
  } catch (err) {
    throw new Error(toErrorMessage(err, "Unable to send the position."));
  }
}
