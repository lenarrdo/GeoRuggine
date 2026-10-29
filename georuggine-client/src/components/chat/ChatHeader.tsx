import React, { useState } from "react";
import { Server, MoreVertical, LogOut, Trash2, User as UserIcon } from "lucide-react";
import { UserSession } from "../../types/auth";

interface ChatHeaderProps {
  currentUser: UserSession | null;
  onLogout: () => void;
  onClearChat?: () => void;
}

export const ChatHeader: React.FC<ChatHeaderProps> = ({
  currentUser,
  onLogout,
  onClearChat,
}) => {
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <header className="navbar navbar-dark bg-dark border-bottom border-secondary px-4 py-2 chat-header">
      <div className="d-flex align-items-center gap-3 chat-header-info">
        <div className="bg-secondary-subtle rounded-circle p-2 d-flex align-items-center justify-content-center server-avatar-circle">
          <Server size={18} className="text-info server-avatar-icon" />
        </div>
        <div className="d-flex flex-column server-details">
          <h3 className="h6 mb-0 fw-bold font-monospace text-light server-name">
            Server
          </h3>
        </div>
      </div>

      <div className="position-relative chat-header-actions">
        <button
          type="button"
          className="btn btn-outline-secondary btn-sm p-1 d-flex align-items-center justify-content-center border-0 header-action-btn"
          title="Options"
          onClick={() => setMenuOpen(!menuOpen)}
        >
          <MoreVertical size={20} />
        </button>

        {menuOpen && (
          <>
            <div
              className="dropdown-backdrop"
              onClick={() => setMenuOpen(false)}
            />
            <ul className="dropdown-menu dropdown-menu-dark dropdown-menu-end show position-absolute end-0 mt-2 shadow-lg border-secondary header-dropdown-menu">
              {currentUser && (
                <li className="dropdown-item-text text-secondary small font-monospace d-flex align-items-center gap-2 border-bottom border-secondary pb-2 mb-1">
                  <UserIcon size={14} className="text-info" />
                  <span className="text-truncate">{currentUser.username}</span>
                </li>
              )}
              {onClearChat && (
                <li>
                  <button
                    type="button"
                    className="dropdown-item font-monospace small d-flex align-items-center gap-2"
                    onClick={() => {
                      setMenuOpen(false);
                      onClearChat();
                    }}
                  >
                    <Trash2 size={14} />
                    <span>Clear chat</span>
                  </button>
                </li>
              )}
              <li>
                <button
                  type="button"
                  className="dropdown-item text-danger font-monospace small d-flex align-items-center gap-2"
                  onClick={() => {
                    setMenuOpen(false);
                    onLogout();
                  }}
                >
                  <LogOut size={14} />
                  <span>Sign out</span>
                </button>
              </li>
            </ul>
          </>
        )}
      </div>
    </header>
  );
};
