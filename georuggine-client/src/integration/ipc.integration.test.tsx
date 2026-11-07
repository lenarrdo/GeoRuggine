import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { App } from "../App";
import { createFakeBackend, FakeBackend } from "../test/fakeBackend";

// The two modules below are the only boundary between React and Rust:
// `invoke` for commands and `listen` for backend events.
vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn() }));

const invokeMock = vi.mocked(invoke);
const listenMock = vi.mocked(listen);

let backend: FakeBackend;

beforeEach(() => {
  backend = createFakeBackend();

  invokeMock.mockImplementation(((
    command: string,
    args?: Record<string, unknown>,
  ) => backend.invoke(command, args ?? {})) as never);

  listenMock.mockImplementation(((
    event: string,
    handler: (event: { payload: unknown }) => void,
  ) => backend.listen(event, handler)) as never);
});

async function registerAndLogin(user: ReturnType<typeof userEvent.setup>) {
  await user.click(screen.getByRole("button", { name: "Sign up" }));
  await user.type(screen.getByLabelText("Username"), "alice");
  await user.type(screen.getByLabelText("Password"), "secret");
  await user.type(screen.getByLabelText("Confirm password"), "secret");
  await user.click(screen.getByRole("button", { name: "Sign up" }));

  await waitFor(() =>
    expect(
      screen.getByText("Registration successful. You can now sign in."),
    ).toBeInTheDocument(),
  );

  await user.type(screen.getByLabelText("Username"), "alice");
  await user.type(screen.getByLabelText("Password"), "secret");
  await user.click(screen.getByRole("button", { name: "Sign in" }));

  await waitFor(() =>
    expect(screen.getByText("No messages in this conversation")).toBeInTheDocument(),
  );
}

async function loginExisting(user: ReturnType<typeof userEvent.setup>) {
  await user.type(screen.getByLabelText("Username"), "alice");
  await user.type(screen.getByLabelText("Password"), "secret");
  await user.click(screen.getByRole("button", { name: "Sign in" }));
}

function selectCoordinateFile(container: HTMLElement, content: string) {
  const input = container.querySelector<HTMLInputElement>('input[type="file"]');
  if (!input) {
    throw new Error("file input not found");
  }
  fireEvent.change(input, {
    target: { files: [new File([content], "coordinates.json", { type: "application/json" })] },
  });
}

describe("frontend <-> backend IPC integration", () => {
  it("registers and logs in using the Rust command names and arguments", async () => {
    const user = userEvent.setup();
    render(<App />);

    await registerAndLogin(user);

    expect(backend.session).toBe("alice");
    expect(backend.connected).toBe(true);
    expect(backend.invocations.map((i) => i.command)).toEqual(["register", "login"]);
    expect(backend.invocations[0].args).toEqual({
      username: "alice",
      password: "secret",
    });
    expect(backend.invocations[1].args).toEqual({
      username: "alice",
      password: "secret",
    });
  });

  it("shows the backend error when the credentials are wrong", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("button", { name: "Sign up" }));
    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.type(screen.getByLabelText("Confirm password"), "secret");
    await user.click(screen.getByRole("button", { name: "Sign up" }));
    await waitFor(() =>
      expect(
        screen.getByText("Registration successful. You can now sign in."),
      ).toBeInTheDocument(),
    );

    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "wrong-password");
    await user.click(screen.getByRole("button", { name: "Sign in" }));

    await waitFor(() =>
      expect(screen.getByText("Invalid username or password.")).toBeInTheDocument(),
    );
    expect(backend.session).toBeNull();
  });

  it("keeps the user on the login screen when the backend cannot open the WebSocket", async () => {
    const user = userEvent.setup();
    render(<App />);

    await user.click(screen.getByRole("button", { name: "Sign up" }));
    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.type(screen.getByLabelText("Confirm password"), "secret");
    await user.click(screen.getByRole("button", { name: "Sign up" }));
    await waitFor(() =>
      expect(
        screen.getByText("Registration successful. You can now sign in."),
      ).toBeInTheDocument(),
    );

    backend.failNextConnection = true;
    await loginExisting(user);

    await waitFor(() =>
      expect(
        screen.getByText("Unable to connect. Please try again later."),
      ).toBeInTheDocument(),
    );
    expect(backend.connected).toBe(false);
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
  });

  it("forwards chat messages to the backend and renders them locally", async () => {
    const user = userEvent.setup();
    render(<App />);
    await registerAndLogin(user);

    await user.type(screen.getByPlaceholderText("Type a message..."), "hello backend");
    await user.click(screen.getByTitle("Send message"));

    await waitFor(() => expect(backend.sentMessages).toHaveLength(1));
    expect(backend.sentMessages[0].contents).toBe("hello backend");
    expect(Number.isNaN(Date.parse(backend.sentMessages[0].timestamp))).toBe(false);

    expect(screen.getByText("hello backend")).toBeInTheDocument();
    const sendInvocation = backend.invocations.find(
      (i) => i.command === "send_chat_message",
    );
    expect(sendInvocation?.args).toEqual({ contents: "hello backend" });
  });

  it("marks the message as failed when the backend is not connected", async () => {
    const user = userEvent.setup();
    render(<App />);
    await registerAndLogin(user);

    backend.connected = false;
    await user.type(screen.getByPlaceholderText("Type a message..."), "will fail");
    await user.click(screen.getByTitle("Send message"));

    await waitFor(() => expect(screen.getByText("Not sent")).toBeInTheDocument());
    expect(screen.getByPlaceholderText("Type a message...")).toHaveValue("will fail");
    expect(backend.sentMessages).toHaveLength(0);
  });

  it("delivers chat messages emitted by the backend to the message list", async () => {
    const user = userEvent.setup();
    render(<App />);
    await registerAndLogin(user);

    act(() => {
      backend.emitChatMessage({
        timestamp: "2026-01-01T10:00:00Z",
        contents: "message from the Rust side",
      });
    });

    expect(screen.getByText("message from the Rust side")).toBeInTheDocument();
  });

  it("uploads a coordinate file through the upload_coordinates command", async () => {
    const user = userEvent.setup();
    const { container } = render(<App />);
    await registerAndLogin(user);

    selectCoordinateFile(
      container,
      '[{"lat":45.0703,"lon":7.6869},{"x":45.071,"y":7.688}]',
    );
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(screen.getByText("File uploaded successfully!")).toBeInTheDocument(),
    );
    expect(backend.uploadedPositions).toHaveLength(2);
    expect(backend.uploadedPositions[0].lat).toBeCloseTo(45.0703);
    expect(backend.uploadedPositions[0].lon).toBeCloseTo(7.6869);
    expect(backend.uploadedPositions[1].lat).toBeCloseTo(45.071);
    expect(backend.uploadedPositions[1].lon).toBeCloseTo(7.688);

    const uploadInvocation = backend.invocations.find(
      (i) => i.command === "upload_coordinates",
    );
    expect(uploadInvocation?.args).toHaveProperty("content");
    expect(String(uploadInvocation?.args.content)).toContain("45.0703");
  });

  it("shows the backend validation error for an invalid coordinate file", async () => {
    const user = userEvent.setup();
    const { container } = render(<App />);
    await registerAndLogin(user);

    selectCoordinateFile(container, "not-json");
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(
        screen.getByText(/does not contain a valid JSON coordinate format/),
      ).toBeInTheDocument(),
    );
    expect(backend.uploadedPositions).toHaveLength(0);
  });

  it("rejects a coordinate upload when the WebSocket is closed", async () => {
    const user = userEvent.setup();
    const { container } = render(<App />);
    await registerAndLogin(user);

    backend.connected = false;
    selectCoordinateFile(container, '[{"lat":1,"lon":2}]');
    await user.click(screen.getByRole("button", { name: "Upload coordinates" }));

    await waitFor(() =>
      expect(screen.getByText(/WebSocket connection to the server is not active/)).toBeInTheDocument(),
    );
    expect(backend.uploadedPositions).toHaveLength(0);
  });

  it("shows the server problem modal on ws-disconnected and logs out through the backend", async () => {
    const user = userEvent.setup();
    render(<App />);
    await registerAndLogin(user);

    act(() => {
      backend.disconnect();
    });

    expect(
      screen.getByText(/There was a problem with the server/),
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Back to login" }));

    await waitFor(() =>
      expect(backend.invocations.some((i) => i.command === "logout")).toBe(true),
    );
    expect(backend.session).toBeNull();
    expect(backend.connected).toBe(false);
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
  });

  it("logs out through the header menu and clears the backend session", async () => {
    const user = userEvent.setup();
    render(<App />);
    await registerAndLogin(user);

    await user.click(screen.getByTitle("Options"));
    await user.click(screen.getByText("Logout"));

    await waitFor(() => expect(backend.session).toBeNull());
    expect(backend.connected).toBe(false);
    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
  });
});
