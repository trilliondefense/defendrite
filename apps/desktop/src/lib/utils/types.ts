export interface Conversation {
  id: string;
  title: string;
  chatNumber: number;
  createdAt: number;
  updatedAt: number;
  messages: Message[];
}

export interface Attachment {
  id: string;
  messageId: number;
  name: string;
  path: string;
  size: number;
  mimeType: string | null;
  createdAt: number;
}

export type MessageRole = "user" | "model";

export interface Message {
  id: number | string;
  conversationId: string;
  role: MessageRole;
  content: string;
  createdAt: number;
  attachments: Attachment[];
}

export interface DraftAttachment {
  id: string;
  messageId: number;
  name: string;
  path: string;
  size: number;
  mimeType: string | null;
}
