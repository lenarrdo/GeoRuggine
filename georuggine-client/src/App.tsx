import { useState, useEffect, useRef } from "react";
import { Container } from "react-bootstrap";
import { AuthLayout } from "./components/auth/AuthLayout";
import { LoginForm } from "./components/auth/LoginForm";
import { RegisterForm } from "./components/auth/RegisterForm";
import { ChatSidebar } from "./components/chat/ChatSidebar";
import { ChatView } from "./components/chat/ChatView";
import {
  apiLogin,
  apiRegister,
  apiLogout,
  apiGetCurrentUser,
  apiSendChatMessage,
  onChatMessageReceived,
  isInTauri,
} from "./services/tauriBridge";
import { UserSession, LoginFormData, RegisterFormData } from "./types/auth";
import { UiMessage, ChatMessage } from "./types/chat";
import "./App.css";

function formatCurrentTime(): string {
  const now = new Date();
  const hours = String(now.getHours()).padStart(2, "0");
  const minutes = String(now.getMinutes()).padStart(2, "0");
  return `${hours}:${minutes}`;
}

export function App() {
  const [currentUser, setCurrentUser] = useState<UserSession | null>(null);
  const [authView, setAuthView] = useState<"login" | "register">("login");
  const [activeTab, setActiveTab] = useState<string>("chat");

  const [isLoading, setIsLoading] = useState(false);
  const [authError, setAuthError] = useState<string | null>(null);
  const [authSuccess, setAuthSuccess] = useState<string | null>(null);

  // Inizializza la cronologia messaggi vuota
  const [messages, setMessages] = useState<UiMessage[]>([]);
  const mockTimeoutRef = useRef<number | null>(null);

  // Controlla se c'è già una sessione attiva all'avvio
  useEffect(() => {
    async function checkExistingSession() {
      const session = await apiGetCurrentUser();
      if (session) {
        setCurrentUser(session);
      }
    }
    checkExistingSession();
  }, []);

  // Cleanup eventuale timeout simulazione su unmount
  useEffect(() => {
    return () => {
      if (mockTimeoutRef.current) {
        clearTimeout(mockTimeoutRef.current);
      }
    };
  }, []);

  // Registrazione dei listener di eventi Tauri: attivo solo per l'utente loggato
  // Alla disconnessione o cambio utente, stacca il listener e resetta i messaggi
  useEffect(() => {
    if (!currentUser) {
      setMessages([]);
      return;
    }

    let isCancelled = false;
    let unlisten: (() => void) | undefined;

    async function setupListeners() {
      // Ascolto nuovi messaggi di chat in arrivo via WebSocket dal server
      const fn = await onChatMessageReceived((msg: ChatMessage) => {
        const timeStr = msg.timestamp
          ? new Date(msg.timestamp).toLocaleTimeString([], {
              hour: "2-digit",
              minute: "2-digit",
            })
          : formatCurrentTime();

        const messageId = `msg-${Date.now()}-${Math.random()}`;
        const senderName = "Server";

        setMessages((prev) => {
          // Prevenzione rigorosa dei duplicati (stesso testo e stesso orario/timestamp)
          const alreadyExists = prev.some(
            (m) => !m.isSelf && m.text === msg.contents && m.time === timeStr
          );
          if (alreadyExists) {
            return prev;
          }

          return [
            ...prev,
            {
              id: messageId,
              isSelf: false,
              senderName,
              text: msg.contents,
              time: timeStr,
            },
          ];
        });
      });

      if (isCancelled) {
        fn();
      } else {
        unlisten = fn;
      }
    }

    setupListeners();

    return () => {
      isCancelled = true;
      if (unlisten) {
        unlisten();
      }
    };
  }, [currentUser]);

  // Gestione Login
  const handleLogin = async (data: LoginFormData) => {
    setIsLoading(true);
    setAuthError(null);
    setAuthSuccess(null);
    try {
      const res = await apiLogin(data.email, data.password);
      if (res.success) {
        setMessages([]); // Reset preventivo di eventuali messaggi residui
        setCurrentUser({ username: data.email });
      } else {
        setAuthError(res.message);
      }
    } catch (err: unknown) {
      const errorMsg =
        typeof err === "string"
          ? err
          : err instanceof Error
          ? err.message
          : "Errore durante il login. Controlla le credenziali.";
      setAuthError(errorMsg);
    } finally {
      setIsLoading(false);
    }
  };

  // Gestione Registrazione
  const handleRegister = async (data: RegisterFormData) => {
    setIsLoading(true);
    setAuthError(null);
    setAuthSuccess(null);
    try {
      const regRes = await apiRegister(data.email, data.password);
      if (regRes.success) {
        // Successo: riporta l'utente al login con messaggio di successo
        setAuthSuccess("Registrazione avvenuta con successo, ora puoi effettuare il login.");
        setAuthError(null);
        setAuthView("login");
      } else {
        // Fallimento: rimane sulla schermata di registrazione e mostra il problema
        setAuthError(regRes.message);
      }
    } catch (err: unknown) {
      const errorMsg =
        typeof err === "string"
          ? err
          : err instanceof Error
          ? err.message
          : "Errore durante la registrazione.";
      setAuthError(errorMsg);
    } finally {
      setIsLoading(false);
    }
  };

  // Gestione Logout: reset completo dello stato utente, cronologia chat ed errori
  const handleLogout = async () => {
    try {
      await apiLogout();
    } catch (err) {
      console.warn("Errore logout:", err);
    } finally {
      if (mockTimeoutRef.current) {
        clearTimeout(mockTimeoutRef.current);
        mockTimeoutRef.current = null;
      }
      setMessages([]);
      setCurrentUser(null);
      setAuthView("login");
      setAuthError(null);
      setAuthSuccess(null);
      setActiveTab("chat");
    }
  };

  // Invio messaggio di chat
  const handleSendMessage = async (text: string) => {
    const newMsg: UiMessage = {
      id: `msg-${Date.now()}`,
      isSelf: true,
      senderName: currentUser?.username || "Tu",
      text,
      time: formatCurrentTime(),
      delivered: true,
    };

    setMessages((prev) => [...prev, newMsg]);

    try {
      await apiSendChatMessage(text);
    } catch (err) {
      console.error("Errore invio messaggio chat:", err);
    }

    // Se non siamo in Tauri (simulazione browser per anteprima UI), generiamo una risposta mock
    if (!isInTauri()) {
      mockTimeoutRef.current = window.setTimeout(() => {
        setMessages((prev) => [
          ...prev,
          {
            id: `msg-echo-${Date.now()}`,
            isSelf: false,
            senderName: "Server",
            text: `Messaggio ricevuto dal server: "${text}"`,
            time: formatCurrentTime(),
          },
        ]);
        mockTimeoutRef.current = null;
      }, 1200);
    }
  };

  const handleClearChat = () => {
    setMessages([]);
  };

  return (
    <Container fluid className="app-window-frame vh-100 vw-100 p-0 d-flex overflow-hidden bg-dark">
      <main className="app-main-content flex-grow-1 d-flex overflow-hidden bg-dark">
        {!currentUser ? (
          /* Schermate di Autenticazione (Login / Registrazione con Split Layout) */
          <AuthLayout>
            {authView === "login" ? (
              <LoginForm
                onSubmit={handleLogin}
                onSwitchToRegister={() => {
                  setAuthError(null);
                  setAuthSuccess(null);
                  setAuthView("register");
                }}
                isLoading={isLoading}
                errorMessage={authError}
                successMessage={authSuccess}
              />
            ) : (
              <RegisterForm
                onSubmit={handleRegister}
                onSwitchToLogin={() => {
                  setAuthError(null);
                  setAuthSuccess(null);
                  setAuthView("login");
                }}
                isLoading={isLoading}
                errorMessage={authError}
              />
            )}
          </AuthLayout>
        ) : (
          /* Schermata Principale Post-Autenticazione: Sidebar & Chat Area */
          <div className="d-flex w-100 h-100 overflow-hidden chat-dashboard-layout">
            <ChatSidebar
              activeTab={activeTab}
              onTabChange={setActiveTab}
            />
            <ChatView
              messages={messages}
              currentUser={currentUser}
              onSendMessage={handleSendMessage}
              onLogout={handleLogout}
              onClearChat={handleClearChat}
            />
          </div>
        )}
      </main>
    </Container>
  );
}

export default App;
