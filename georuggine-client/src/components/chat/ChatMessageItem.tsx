import React from "react";
import { Server, CheckCheck } from "lucide-react";
import { UiMessage } from "../../types/chat";

interface ChatMessageItemProps {
  message: UiMessage;
}

export const ChatMessageItem: React.FC<ChatMessageItemProps> = ({ message }) => {
  if (message.isSelf) {
    return (
      <div className="d-flex w-100 justify-content-end mb-3 chat-msg-row user-row">
        <div
          className="card bg-primary text-white border-0 shadow-sm px-3 py-2 font-monospace chat-bubble user-bubble"
          style={{ maxWidth: "70%", borderRadius: "14px 14px 2px 14px" }}
        >
          <div className="small text-info-emphasis fw-bold mb-1 bubble-prompt user-prompt">
            {message.senderName || "client"}@georuggine:~$
          </div>
          <p className="mb-1 text-break bubble-text">{message.text}</p>
          <div className="d-flex align-items-center justify-content-end gap-1 small text-white-50 bubble-meta">
            <span className="bubble-time">[{message.time}]</span>
            <CheckCheck size={14} className="text-info bubble-check-icon" />
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="d-flex w-100 justify-content-start align-items-start gap-2 mb-3 chat-msg-row server-row">
      <div className="bg-secondary-subtle rounded-circle p-2 d-flex align-items-center justify-content-center flex-shrink-0 msg-server-avatar">
        <Server size={16} className="text-info msg-server-icon" />
      </div>
      <div
        className="card bg-dark text-light border-secondary shadow-sm px-3 py-2 font-monospace chat-bubble server-bubble"
        style={{ maxWidth: "70%", borderRadius: "14px 14px 14px 2px" }}
      >
        <div className="small text-info fw-bold mb-1 bubble-prompt server-prompt">
          server@georuggine:~$
        </div>
        <p className="mb-1 text-break bubble-text">{message.text}</p>
        <div className="d-flex justify-content-start small text-secondary bubble-meta">
          <span className="bubble-time">[{message.time}]</span>
        </div>
      </div>
    </div>
  );
};
