import React from "react";
import { Server, User } from "lucide-react";
import { UiMessage } from "../../types/chat";

interface ChatMessageItemProps {
  message: UiMessage;
}

export const ChatMessageItem: React.FC<ChatMessageItemProps> = ({ message }) => {
  if (message.isSelf) {
    return (
      <div className="d-flex w-100 justify-content-end mb-3 chat-msg-row user-row">
        <div className="chat-bubble user-bubble px-3 py-2">
          <div className="d-flex align-items-center justify-content-between gap-3 mb-1 bubble-header user-header">
            <span className="bubble-sender fw-bold">{message.senderName || "Tu"}</span>
            <User size={13} className="bubble-sender-icon text-white-50" />
          </div>
          <p className="mb-1 text-break bubble-text">{message.text}</p>
          <div className="d-flex align-items-center justify-content-end bubble-meta">
            <span className="bubble-time">{message.time}</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="d-flex w-100 justify-content-start align-items-start gap-2 mb-3 chat-msg-row server-row">
      <div className="msg-server-avatar">
        <Server size={16} className="msg-server-icon" />
      </div>
      <div className="chat-bubble server-bubble px-3 py-2">
        <div className="d-flex align-items-center justify-content-between gap-3 mb-1 bubble-header server-header">
          <span className="bubble-sender fw-bold">{message.senderName || "Server"}</span>
          <span className="badge bg-info-subtle text-info border border-info-subtle px-1 py-0" style={{ fontSize: "11px" }}>CENTRALE</span>
        </div>
        <p className="mb-1 text-break bubble-text">{message.text}</p>
        <div className="d-flex justify-content-start bubble-meta">
          <span className="bubble-time">{message.time}</span>
        </div>
      </div>
    </div>
  );
};
