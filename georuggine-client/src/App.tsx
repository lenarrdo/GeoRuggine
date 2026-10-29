import { useState, useEffect } from "react";
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
  apiSendChatMessage,
  onChatMessageReceived,
  onWsDisconnected,
} from "./services/tauriBridge";
import { Modal, Button } from "react-bootstrap";
import { UserSession, LoginFormData, RegisterFormData } from "./types/auth";
import { UiMessage, ChatMessage } from "./types/chat";
import "./App.css";

function formatTime(date: Date): string {
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  return `${hours}:${minutes}`;
}

function formatCurrentTime(): string {
  return formatTime(new Date());
}

export function App() {
  const [currentUser, setCurrentUser] = useState<UserSession | null>(null);
  const [authView, setAuthView] = useState<"login" | "register">("login");
  const [activeTab, setActiveTab] = useState<string>("chat");

  const [isLoading, setIsLoading] = useState(false);
  const [authError, setAuthError] = useState<string | null>(null);
  const [authSuccess, setAuthSuccess] = useState<string | null>(null);
  const [serverDown, setServerDown] = useState(false);

  // Inizializza la cronologia messaggi vuota
  const [messages, setMessages] = useState<UiMessage[]>([]);

  // Registrazione dei listener di eventi Tauri: attivo solo per l'utente loggato
  // Alla disconnessione o cambio utente, stacca il listener e resetta i messaggi
  useEffect(() => {
    if (!currentUser) {
      setMessages([]);
      return;
    }

    let isCancelled = false;
    let unlisten: (() => void) | undefined;
    let unlistenDisconnect: (() => void) | undefined;

    async function setupListeners() {
      try {
        // Connessione al server caduta mentre l'utente è loggato
        const fnDisconnect = await onWsDisconnected(() => setServerDown(true));
        if (isCancelled) {
          fnDisconnect();
        } else {
          unlistenDisconnect = fnDisconnect;
        }

        // Ascolto nuovi messaggi di chat in arrivo via WebSocket dal server
        const fn = await onChatMessageReceived((msg: ChatMessage) => {
          const msgDate = msg.timestamp ? new Date(msg.timestamp) : null;
          const timeStr =
            msgDate && !isNaN(msgDate.getTime())
              ? formatTime(msgDate)
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
      } catch {
        // Listener non registrato: l'app resta utilizzabile ma senza eventi dal server
        setServerDown(true);
      }
    }

    setupListeners();

    return () => {
      isCancelled = true;
      if (unlisten) {
        unlisten();
      }
      if (unlistenDisconnect) {
        unlistenDisconnect();
      }
    };
  }, [currentUser]);

  // Gestione Login
  const handleLogin = async (data: LoginFormData) => {
    setIsLoading(true);
    setAuthError(null);
    setAuthSuccess(null);
    try {
      const res = await apiLogin(data.username, data.password);
      if (res.success) {
        setMessages([]); // Reset preventivo di eventuali messaggi residui
        setCurrentUser({ username: data.username });
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
      const regRes = await apiRegister(data.username, data.password);
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
    } catch {
      // Errore lato backend ignorato: la sessione locale viene comunque azzerata nel finally
    } finally {
      setMessages([]);
      setCurrentUser(null);
      setAuthView("login");
      setAuthError(null);
      setAuthSuccess(null);
      setActiveTab("chat");
      setServerDown(false);
    }
  };

  // Invio messaggio di chat
  const handleSendMessage = async (text: string) => {
    const messageId = `msg-${Date.now()}-${Math.random()}`;
    const newMsg: UiMessage = {
      id: messageId,
      isSelf: true,
      senderName: currentUser?.username || "Tu",
      text,
      time: formatCurrentTime(),
    };

    setMessages((prev) => [...prev, newMsg]);

    try {
      await apiSendChatMessage(text);
    } catch (err) {
      setMessages((prev) =>
        prev.map((m) => (m.id === messageId ? { ...m, failed: true } : m))
      );
      // Rilancia l'errore: ChatInput ripristina il testo nella barra
      throw err;
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

      <Modal show={serverDown && !!currentUser} backdrop="static" keyboard={false} centered>
          <Modal.Body className="font-monospace">
            C'è stato un problema con il server.
            <br />
            Si perdoni il disagio.
          </Modal.Body>
        <Modal.Footer>
          <Button variant="primary" onClick={handleLogout}>
            Torna al login
          </Button>
        </Modal.Footer>
      </Modal>
    </Container>
  );
}

export default App;
