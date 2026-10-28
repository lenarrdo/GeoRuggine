import React, { useEffect } from "react";
import { MessageSquare } from "lucide-react";
import { ChatHeader } from "./ChatHeader";
import { ChatMessageItem } from "./ChatMessageItem";
import { ChatInput } from "./ChatInput";
import { UiMessage } from "../../types/chat";
import { UserSession } from "../../types/auth";

interface ChatViewProps {
  messages: UiMessage[];
  currentUser: UserSession | null;
  onSendMessage: (text: string) => Promise<void>;
  onLogout: () => void;
  onClearChat: () => void;
}

export const ChatView: React.FC<ChatViewProps> = ({
  messages,
  currentUser,
  onSendMessage,
  onLogout,
  onClearChat,
}) => {
  useEffect(() => {
    const endEl = document.getElementById("chat-messages-end");
    endEl?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  return (
    <div className="d-flex flex-column h-100 flex-grow-1 bg-dark text-light overflow-hidden chat-main-view">
      {/* Header chat con opzioni */}
      <ChatHeader
        currentUser={currentUser}
        onLogout={onLogout}
        onClearChat={onClearChat}
      />

      {/* Lista messaggi a scorrimento */}
      <div className="flex-grow-1 overflow-y-auto p-4 chat-messages-container">
        <div
          className="d-flex flex-column gap-3 mx-auto chat-messages-inner"
          style={{ maxWidth: "900px" }}
        >
          {messages.length === 0 ? (
            <div className="d-flex flex-column align-items-center justify-content-center text-center p-5 my-auto text-secondary font-monospace chat-empty-state">
              <div className="p-3 rounded-circle bg-secondary bg-opacity-10 border border-secondary border-opacity-25 mb-3">
                <MessageSquare size={32} className="text-info opacity-75" />
              </div>
              <p className="mb-1 text-light fw-semibold">Nessun messaggio nella conversazione</p>
              <p className="small text-secondary mb-0">Invia un messaggio per avviare la chat.</p>
            </div>
          ) : (
            messages.map((msg) => (
              <ChatMessageItem key={msg.id} message={msg} />
            ))
          )}
          <div id="chat-messages-end" />
        </div>
      </div>

      {/* Barra di digitazione e invio */}
      <ChatInput onSendMessage={onSendMessage} />
    </div>
  );
};
