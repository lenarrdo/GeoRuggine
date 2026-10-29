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

  // Initialize the message history as empty
  const [messages, setMessages] = useState<UiMessage[]>([]);

  // Tauri event listener registration: only active while a user is logged in.
  // On disconnect or user change, the listeners are detached and messages reset
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
        // Connection to the server lost while the user is logged in
        const fnDisconnect = await onWsDisconnected(() => setServerDown(true));
        if (isCancelled) {
          fnDisconnect();
        } else {
          unlistenDisconnect = fnDisconnect;
        }

        // Listen for new chat messages coming from the server over the WebSocket
        const fn = await onChatMessageReceived((msg: ChatMessage) => {
          const msgDate = msg.timestamp ? new Date(msg.timestamp) : null;
          const timeStr =
            msgDate && !isNaN(msgDate.getTime())
              ? formatTime(msgDate)
              : formatCurrentTime();

          const messageId = `msg-${Date.now()}-${Math.random()}`;
          const senderName = "Server";

          setMessages((prev) => {
            // Strict duplicate prevention (same text and same time/timestamp)
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
        // Listener not registered: the app stays usable but without server events
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

  // Login handling
  const handleLogin = async (data: LoginFormData) => {
    setIsLoading(true);
    setAuthError(null);
    setAuthSuccess(null);
    try {
      const res = await apiLogin(data.username, data.password);
      if (res.success) {
        setMessages([]); // Preventive reset of any leftover messages
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
          : "Login failed. Check your credentials.";
      setAuthError(errorMsg);
    } finally {
      setIsLoading(false);
    }
  };

  // Registration handling
  const handleRegister = async (data: RegisterFormData) => {
    setIsLoading(true);
    setAuthError(null);
    setAuthSuccess(null);
    try {
      const regRes = await apiRegister(data.username, data.password);
      if (regRes.success) {
        // Success: send the user back to login with a success message
        setAuthSuccess("Registration successful. You can now sign in.");
        setAuthError(null);
        setAuthView("login");
      } else {
        // Failure: stay on the registration screen and show the problem
        setAuthError(regRes.message);
      }
    } catch (err: unknown) {
      const errorMsg =
        typeof err === "string"
          ? err
          : err instanceof Error
          ? err.message
          : "Registration failed.";
      setAuthError(errorMsg);
    } finally {
      setIsLoading(false);
    }
  };

  // Logout handling: full reset of user state, chat history and errors
  const handleLogout = async () => {
    try {
      await apiLogout();
    } catch {
      // Backend error ignored: the local session is cleared in the finally block anyway
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

  // Chat message sending
  const handleSendMessage = async (text: string) => {
    const messageId = `msg-${Date.now()}-${Math.random()}`;
    const newMsg: UiMessage = {
      id: messageId,
      isSelf: true,
      senderName: currentUser?.username || "You",
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
      // Rethrow the error: ChatInput restores the text in the input bar
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
          /* Authentication screens (Login / Registration with split layout) */
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
          /* Main post-authentication screen: sidebar and chat area */
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
            There was a problem with the server.
            <br />
            We are sorry for the inconvenience.
          </Modal.Body>
        <Modal.Footer>
          <Button variant="primary" onClick={handleLogout}>
            Back to login
          </Button>
        </Modal.Footer>
      </Modal>
    </Container>
  );
}

export default App;
