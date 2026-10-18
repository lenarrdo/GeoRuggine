import React, { useEffect } from "react";
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
          {messages.map((msg) => (
            <ChatMessageItem key={msg.id} message={msg} />
          ))}
          <div id="chat-messages-end" />
        </div>
      </div>

      {/* Barra di digitazione e invio */}
      <ChatInput onSendMessage={onSendMessage} />
    </div>
  );
};
