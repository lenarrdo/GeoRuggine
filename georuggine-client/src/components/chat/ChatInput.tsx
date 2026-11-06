import React, { useState } from "react";
import { ArrowRight } from "lucide-react";

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

  const handleSend = async () => {
    const trimmed = text.trim();
    if (!trimmed || isSending || disabled) return;

    try {
      setIsSending(true);
      setText("");
      await onSendMessage(trimmed);
    } catch {
      // Send failed: restore the text in the input bar for another attempt
      setText(trimmed);
    } finally {
      setIsSending(false);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    e.stopPropagation();
    handleSend();
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      // Always prevent the implicit form submit so that Shift+Enter cannot
      // accidentally send the message; only plain Enter triggers a send.
      e.preventDefault();
      e.stopPropagation();
      if (!e.shiftKey) {
        handleSend();
      }
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
          placeholder="Type a message..."
          value={text}
          onChange={(e) => setText(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={disabled || isSending}
          autoComplete="off"
        />

        <button
          type="submit"
          className="btn btn-primary d-flex align-items-center justify-content-center rounded-2 shadow-sm chat-send-btn"
          title="Send message"
          disabled={disabled || isSending || !text.trim()}
        >
          <ArrowRight size={18} strokeWidth={2.25} aria-hidden="true" />
        </button>
      </form>
    </div>
  );
};
