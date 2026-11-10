export interface Message {
  id: string;
  sender: "user" | "agent";
  text: string;
}

export interface Attachment {
  id: string;
  name: string;
  path: string;
  size: number;
  mimeType: string | null;
}
