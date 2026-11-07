import React from "react";
import { MapPin, MessageSquare } from "lucide-react";
import { FileUploader } from "../upload/FileUploader";
import { apiSendPosition } from "../../services/tauriBridge";

interface ChatSidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

async function FileUpload(content: string) {
  await apiSendPosition(content);
}

export const ChatSidebar: React.FC<ChatSidebarProps> = ({
  activeTab,
  onTabChange,
}) => {
  return (
    <aside
      className="d-flex flex-column flex-shrink-0 p-3 bg-dark text-white border-end border-secondary chat-sidebar"
      style={{ width: "220px", height: "100vh" }}
    >
      {/* Brand logo in the sidebar */}
      <div className="d-flex align-items-center gap-2 pb-3 mb-3 border-bottom border-secondary sidebar-brand">
        <div className="bg-primary rounded-3 p-2 d-flex align-items-center justify-content-center sidebar-brand-icon-wrapper">
          <MapPin size={20} className="text-white sidebar-brand-icon" />
        </div>
        <span className="fs-5 fw-bold font-monospace text-white sidebar-brand-name">
          GeoRuggine
        </span>
      </div>

      {/* Navigation menu */}
      <ul className="nav nav-pills flex-column mb-auto sidebar-nav">
        <li className="nav-item">
          <button
            type="button"
            className={`nav-link w-100 text-start d-flex align-items-center gap-2 font-monospace ${activeTab === "chat" ? "active" : "text-secondary"
              } sidebar-nav-item`}
            onClick={() => onTabChange("chat")}
          >
            <MessageSquare size={18} className="sidebar-nav-icon" />
            <span className="sidebar-nav-label">Chat</span>
          </button>
        </li>
      </ul>

      <div className="mt-auto pt-3 border-top border-secondary">
        <FileUploader
          onFileUploaded={FileUpload}
        />
      </div>
    </aside>
  );
};
