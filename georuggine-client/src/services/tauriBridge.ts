import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AuthResult, UserSession } from "../types/auth";
import { ChatMessage } from "../types/chat";

/** Normalizza l'errore restituito da Tauri (stringa o Error) in un messaggio leggibile */
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
 * Autenticazione: Registrazione nuovo account
 */
export async function apiRegister(username: string, password: string): Promise<AuthResult> {
  try {
    return await invoke<AuthResult>("register", { username, password });
  } catch (err) {
    return {
      success: false,
      message: toErrorMessage(err, "Errore durante la registrazione."),
    };
  }
}

/**
 * Autenticazione: Login
 */
export async function apiLogin(username: string, password: string): Promise<AuthResult> {
  try {
    return await invoke<AuthResult>("login", { username, password });
  } catch (err) {
    return {
      success: false,
      message: toErrorMessage(err, "Errore durante il login."),
    };
  }
}

/**
 * Autenticazione: Logout
 */
export async function apiLogout(): Promise<void> {
  try {
    await invoke("logout");
  } catch (err) {
    throw new Error(toErrorMessage(err, "Errore durante il logout."));
  }
}

/**
 * Autenticazione: Recupero utente corrente salvato in sessione
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
 * Chat: Invio messaggio di testo al server o destinatario
 */
export async function apiSendChatMessage(contents: string): Promise<void> {
  try {
    await invoke("send_chat_message", { contents });
  } catch (err) {
    throw new Error(toErrorMessage(err, "Impossibile inviare il messaggio."));
  }
}

/**
 * Eventi WebSocket: ascolto ricezione messaggi di chat dal server
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
      toErrorMessage(err, "Impossibile ascoltare i messaggi dal server."),
    );
  }
}

/**
 * Eventi WebSocket: ascolto della chiusura imprevista della connessione al server
 */
export async function onWsDisconnected(callback: () => void): Promise<UnlistenFn> {
  try {
    return await listen("ws-disconnected", () => {
      callback();
    });
  } catch (err) {
    throw new Error(
      toErrorMessage(err, "Impossibile ascoltare lo stato del server."),
    );
  }
}

/**
 * Posizione: invio delle coordinate al server
 */
export async function apiSendPosition(content: string): Promise<void> {
  try {
    await invoke("upload_coordinates", { content });
  } catch (err) {
    throw new Error(toErrorMessage(err, "Impossibile inviare la posizione."));
  }
}
