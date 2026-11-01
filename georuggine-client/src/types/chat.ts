export interface ChatMessage {
  timestamp: string;
  contents: string;
}

export interface UiMessage {
  id: string;
  isSelf: boolean;
  senderName: string;
  text: string;
  time: string;
  failed?: boolean;
}
