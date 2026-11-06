import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { LoginForm } from "./LoginForm";

function setup(overrides: Partial<React.ComponentProps<typeof LoginForm>> = {}) {
  const onSubmit = vi.fn().mockResolvedValue(undefined);
  const onSwitchToRegister = vi.fn();
  const props: React.ComponentProps<typeof LoginForm> = {
    onSubmit,
    onSwitchToRegister,
    isLoading: false,
    errorMessage: null,
    successMessage: null,
    ...overrides,
  };

  render(<LoginForm {...props} />);

  return { onSubmit, onSwitchToRegister, user: userEvent.setup() };
}

describe("LoginForm", () => {
  it("renders the login title and both fields", () => {
    setup();

    expect(screen.getByText("Sign in to your account")).toBeInTheDocument();
    expect(screen.getByLabelText("Username")).toBeInTheDocument();
    expect(screen.getByLabelText("Password")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Sign in" })).toBeInTheDocument();
  });

  it("keeps the submit button disabled until both fields are filled", async () => {
    const { user } = setup();
    const submit = screen.getByRole("button", { name: "Sign in" });

    expect(submit).toBeDisabled();

    await user.type(screen.getByLabelText("Username"), "alice");
    expect(submit).toBeDisabled();

    await user.type(screen.getByLabelText("Password"), "secret");
    expect(submit).toBeEnabled();
  });

  it("submits trimmed credentials", async () => {
    const { onSubmit, user } = setup();

    await user.type(screen.getByLabelText("Username"), "  alice  ");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.click(screen.getByRole("button", { name: "Sign in" }));

    expect(onSubmit).toHaveBeenCalledTimes(1);
    expect(onSubmit).toHaveBeenCalledWith({
      username: "alice",
      password: "secret",
    });
  });

  it("does not submit when the username contains only whitespace", async () => {
    const { onSubmit, user } = setup();

    await user.type(screen.getByLabelText("Username"), "   ");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.click(screen.getByRole("button", { name: "Sign in" }));

    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("toggles password visibility", async () => {
    const { user } = setup();
    const password = screen.getByLabelText("Password");

    expect(password).toHaveAttribute("type", "password");

    await user.click(screen.getByTitle("Show password"));
    expect(password).toHaveAttribute("type", "text");

    await user.click(screen.getByTitle("Hide password"));
    expect(password).toHaveAttribute("type", "password");
  });

  it("shows the error message returned by the backend", () => {
    setup({ errorMessage: "Invalid credentials." });

    expect(screen.getByText("Invalid credentials.")).toBeInTheDocument();
  });

  it("shows the success message after a registration", () => {
    setup({ successMessage: "Registration successful. You can now sign in." });

    expect(
      screen.getByText("Registration successful. You can now sign in."),
    ).toBeInTheDocument();
  });

  it("switches to the registration view", async () => {
    const { onSwitchToRegister, user } = setup();

    await user.click(screen.getByRole("button", { name: "Sign up" }));

    expect(onSwitchToRegister).toHaveBeenCalledTimes(1);
  });

  it("disables every control while a login is in progress", () => {
    setup({ isLoading: true });

    expect(screen.getByLabelText("Username")).toBeDisabled();
    expect(screen.getByLabelText("Password")).toBeDisabled();
    expect(screen.getByText("Signing in...")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Signing in/ })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Sign up" })).toBeDisabled();
  });
});
