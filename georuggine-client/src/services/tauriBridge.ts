import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AuthResult, UserSession } from "../types/auth";
import { ChatMessage } from "../types/chat";

// Verifica se stiamo eseguendo dentro il runtime Tauri
export const isInTauri = (): boolean => {
  try {
    return isTauri();
  } catch {
    return false;
  }
};

/**
 * Autenticazione: Registrazione nuovo account
 */
export async function apiRegister(username: string, password: string): Promise<AuthResult> {
  if (isInTauri()) {
    return await invoke<AuthResult>("register", { username, password });
  } else {
    // Modalità mock per sviluppo frontend in browser
    console.log("[Mock Tauri] Invocata registrazione per:", username);
    await new Promise((resolve) => setTimeout(resolve, 600));
    return {
      success: true,
      message: "Registrazione completata con successo!",
    };
  }
}

/**
 * Autenticazione: Login
 */
export async function apiLogin(username: string, password: string): Promise<AuthResult> {
  if (isInTauri()) {
    return await invoke<AuthResult>("login", { username, password });
  } else {
    // Modalità mock per sviluppo frontend in browser
    console.log("[Mock Tauri] Invocato login per:", username);
    await new Promise((resolve) => setTimeout(resolve, 600));
    return {
      success: true,
      message: "Login effettuato con successo",
    };
  }
}

/**
 * Autenticazione: Logout
 */
export async function apiLogout(): Promise<void> {
  if (isInTauri()) {
    await invoke("logout");
  } else {
    console.log("[Mock Tauri] Logout effettuato");
  }
}

/**
 * Autenticazione: Recupero utente corrente salvato in sessione
 */
export async function apiGetCurrentUser(): Promise<UserSession | null> {
  if (isInTauri()) {
    try {
      const username = await invoke<string | null>("get_current_user");
      return username ? { username } : null;
    } catch (err) {
      console.warn("Errore get_current_user:", err);
      return null;
    }
  } else {
    return null;
  }
}

/**
 * Chat: Invio messaggio di testo al server o destinatario
 */
export async function apiSendChatMessage(
  contents: string,
): Promise<void> {
  if (isInTauri()) {
    await invoke("send_chat_message", {
      contents,
    });
  } else {
    console.log("[Mock Tauri] Invio messaggio:", contents);
  }
}

/**
 * Eventi WebSocket: ascolto ricezione messaggi di chat dal server
 */
export async function onChatMessageReceived(
  callback: (msg: ChatMessage) => void,
): Promise<UnlistenFn> {
  if (isInTauri()) {
    return await listen<ChatMessage>("chat-message-received", (event) => {
      callback(event.payload);
    });
  } else {
    // In ambiente browser, nessun listener nativo
    return () => { };
  }
}
export async function apiSendPosition(content: string): Promise<void> {
  if (isInTauri()) {
    await invoke("upload_coordinates", {
      content,
    });
  } else {
    console.log("[Mock Tauri] Invio posizione:", content);
  }
}
