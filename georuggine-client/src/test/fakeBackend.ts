export interface TextMessagePayload {
  contents: string;
  timestamp: string;
}

export interface PositionPayload {
  lat: number;
  lon: number;
  timestamp?: string;
}

export interface Invocation {
  command: string;
  args: Record<string, unknown>;
}

type EventHandler = (event: { payload: unknown }) => void;

/**
 * In-memory simulation of the Rust backend exposed through Tauri.
 *
 * The goal is to verify the *contract* between the React frontend and the
 * Rust side (command names, argument names, event names and payload shapes)
 * without starting the actual Tauri window.
 *
 * Every command mirrors the behaviour of the corresponding Rust command:
 *   - register / login / logout / get_current_user
 *   - send_chat_message
 *   - upload_coordinates
 */
export class FakeBackend {
  private readonly users = new Map<string, string>();
  private readonly listeners = new Map<string, Set<EventHandler>>();

  readonly invocations: Invocation[] = [];
  readonly sentMessages: TextMessagePayload[] = [];
  readonly uploadedPositions: PositionPayload[] = [];

  connected = false;
  session: string | null = null;
  failNextConnection = false;

  invoke = async (
    command: string,
    args: Record<string, unknown> = {},
  ): Promise<unknown> => {
    this.invocations.push({ command, args });

    switch (command) {
      case "register":
        return this.register(args);
      case "login":
        return this.login(args);
      case "logout":
        return this.logout();
      case "get_current_user":
        return this.session;
      case "send_chat_message":
        return this.sendChatMessage(args);
      case "upload_coordinates":
        return this.uploadCoordinates(args);
      default:
        throw new Error(`Unknown Tauri command: ${command}`);
    }
  };

  listen = async (
    event: string,
    handler: EventHandler,
  ): Promise<(() => void)> => {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(handler);

    return () => {
      this.listeners.get(event)?.delete(handler);
    };
  };

  emit(event: string, payload?: unknown): void {
    for (const handler of this.listeners.get(event) ?? []) {
      handler({ payload });
    }
  }

  emitChatMessage(message: TextMessagePayload): void {
    this.emit("chat-message-received", message);
  }

  /** Simulates an unexpected WebSocket close on the Rust side. */
  disconnect(): void {
    this.connected = false;
    this.emit("ws-disconnected");
  }

  private register(args: Record<string, unknown>) {
    const username = String(args.username ?? "").trim();
    const password = String(args.password ?? "");

    if (!username) {
      return { success: false, message: "Username cannot be empty." };
    }
    if (password.length < 4) {
      return {
        success: false,
        message: "Password must be at least 4 characters long.",
      };
    }
    if (this.users.has(username)) {
      return { success: false, message: "Username already taken." };
    }

    this.users.set(username, password);
    return { success: true, message: "Registration successful." };
  }

  private login(args: Record<string, unknown>) {
    const username = String(args.username ?? "").trim();
    const password = String(args.password ?? "");

    if (!username || !password) {
      return {
        success: false,
        message: "Username and password are required.",
      };
    }
    if (this.users.get(username) !== password) {
      return { success: false, message: "Invalid username or password." };
    }
    if (this.failNextConnection) {
      this.failNextConnection = false;
      return {
        success: false,
        message: "Unable to connect. Please try again later.",
      };
    }

    this.connected = true;
    this.session = username;
    return { success: true, message: "Login successful." };
  }

  private logout() {
    this.connected = false;
    this.session = null;
    return null;
  }

  private sendChatMessage(args: Record<string, unknown>) {
    const contents = String(args.contents ?? "").trim();
    if (!contents) {
      throw "Message cannot be empty.";
    }
    if (!this.connected) {
      throw "WebSocket is not connected to the server";
    }

    this.sentMessages.push({
      contents,
      timestamp: new Date().toISOString(),
    });
    return null;
  }

  private uploadCoordinates(args: Record<string, unknown>) {
    const content = String(args.content ?? "").trim();
    if (!content) {
      throw "The file is empty.";
    }
    if (!this.connected) {
      throw "Unable to start tracking: the WebSocket connection to the server is not active. Sign in or check your network status.";
    }

    let parsed: unknown;
    try {
      parsed = JSON.parse(content);
    } catch (error) {
      throw `The file does not contain a valid JSON coordinate format: ${String(error)}`;
    }

    if (!Array.isArray(parsed) || parsed.length === 0) {
      throw "The JSON file does not contain any coordinate.";
    }

    const points = parsed.map((point: Record<string, unknown>) => {
      const lat = Number(point.lat ?? point.x);
      const lon = Number(point.lon ?? point.y);
      if (Number.isNaN(lat) || Number.isNaN(lon)) {
        throw "The file does not contain a valid JSON coordinate format";
      }
      return { lat, lon, timestamp: new Date().toISOString() };
    });

    this.uploadedPositions.push(...points);
    return null;
  }
}

export function createFakeBackend(): FakeBackend {
  return new FakeBackend();
}
