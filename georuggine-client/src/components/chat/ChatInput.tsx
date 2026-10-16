import React, { useState } from "react";

interface ChatInputProps {
  onSendMessage: (text: string) => Promise<void>;
  disabled?: boolean;
}

export const ChatInput: React.FC<ChatInputProps> = ({
  onSendMessage,
  disabled = false,
}) => {
  const [text, setText] = useState("");
  const [isSending, setIsSending] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = text.trim();
    if (!trimmed || isSending || disabled) return;

    try {
      setIsSending(true);
      setText("");
      await onSendMessage(trimmed);
    } finally {
      setIsSending(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  return (
    <div className="p-3 bg-dark border-top border-secondary chat-input-wrapper">
      <form
        className="input-group shadow-sm bg-dark border border-secondary rounded-3 p-1 chat-input-container"
        onSubmit={handleSubmit}
        style={{ maxWidth: "900px", margin: "0 auto" }}
      >
        <input
          id="chat-message-input"
          type="text"
          className="form-control bg-transparent text-light border-0 shadow-none px-3 chat-text-input"
          placeholder="Scrivi un messaggio..."
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={disabled || isSending}
          autoComplete="off"
        />

        <button
          type="submit"
          className="btn btn-primary d-flex align-items-center justify-content-center px-3 rounded-2 shadow-sm chat-send-btn"
          title="Invia messaggio"
          disabled={disabled || isSending || !text.trim()}
        >
          <i className="bi bi-arrow-right fs-5"></i>
        </button>
      </form>
    </div>
  );
};
