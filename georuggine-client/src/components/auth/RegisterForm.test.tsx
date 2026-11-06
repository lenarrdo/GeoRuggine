import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { RegisterForm } from "./RegisterForm";

function setup(overrides: Partial<React.ComponentProps<typeof RegisterForm>> = {}) {
  const onSubmit = vi.fn().mockResolvedValue(undefined);
  const onSwitchToLogin = vi.fn();
  const props: React.ComponentProps<typeof RegisterForm> = {
    onSubmit,
    onSwitchToLogin,
    isLoading: false,
    errorMessage: null,
    ...overrides,
  };

  render(<RegisterForm {...props} />);

  return { onSubmit, onSwitchToLogin, user: userEvent.setup() };
}

describe("RegisterForm", () => {
  it("renders the registration title and three fields", () => {
    setup();

    expect(screen.getByText("Create your account")).toBeInTheDocument();
    expect(screen.getByLabelText("Username")).toBeInTheDocument();
    expect(screen.getByLabelText("Password")).toBeInTheDocument();
    expect(screen.getByLabelText("Confirm password")).toBeInTheDocument();
  });

  it("keeps the submit button disabled until all fields are filled", async () => {
    const { user } = setup();
    const submit = screen.getByRole("button", { name: "Sign up" });

    expect(submit).toBeDisabled();
    await user.type(screen.getByLabelText("Username"), "alice");
    expect(submit).toBeDisabled();
    await user.type(screen.getByLabelText("Password"), "secret");
    expect(submit).toBeDisabled();
    await user.type(screen.getByLabelText("Confirm password"), "secret");
    expect(submit).toBeEnabled();
  });

  it("rejects passwords shorter than four characters", async () => {
    const { onSubmit, user } = setup();

    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "abc");
    await user.type(screen.getByLabelText("Confirm password"), "abc");
    await user.click(screen.getByRole("button", { name: "Sign up" }));

    expect(
      screen.getByText("Password must be at least 4 characters long"),
    ).toBeInTheDocument();
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("rejects mismatched passwords", async () => {
    const { onSubmit, user } = setup();

    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.type(screen.getByLabelText("Confirm password"), "different");
    await user.click(screen.getByRole("button", { name: "Sign up" }));

    expect(screen.getByText("Passwords do not match")).toBeInTheDocument();
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("rejects a whitespace-only username", async () => {
    const { onSubmit, user } = setup();

    await user.type(screen.getByLabelText("Username"), "    ");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.type(screen.getByLabelText("Confirm password"), "secret");

    // The submit button is disabled for a whitespace-only username, so the
    // form handler is exercised directly to cover the defensive validation.
    expect(screen.getByRole("button", { name: "Sign up" })).toBeDisabled();
    fireEvent.submit(document.querySelector("form")!);

    expect(screen.getByText("Username is required")).toBeInTheDocument();
    expect(onSubmit).not.toHaveBeenCalled();
  });

  it("submits valid data with a trimmed username", async () => {
    const { onSubmit, user } = setup();

    await user.type(screen.getByLabelText("Username"), "  alice  ");
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.type(screen.getByLabelText("Confirm password"), "secret");
    await user.click(screen.getByRole("button", { name: "Sign up" }));

    expect(onSubmit).toHaveBeenCalledTimes(1);
    expect(onSubmit).toHaveBeenCalledWith({
      username: "alice",
      password: "secret",
      confirmPassword: "secret",
    });
  });

  it("clears a previous validation error on a new submit", async () => {
    const { user } = setup();

    await user.type(screen.getByLabelText("Username"), "alice");
    await user.type(screen.getByLabelText("Password"), "abc");
    await user.type(screen.getByLabelText("Confirm password"), "abc");
    await user.click(screen.getByRole("button", { name: "Sign up" }));
    expect(screen.getByText("Password must be at least 4 characters long")).toBeInTheDocument();

    await user.clear(screen.getByLabelText("Password"));
    await user.type(screen.getByLabelText("Password"), "secret");
    await user.clear(screen.getByLabelText("Confirm password"));
    await user.type(screen.getByLabelText("Confirm password"), "secret");
    await user.click(screen.getByRole("button", { name: "Sign up" }));

    expect(
      screen.queryByText("Password must be at least 4 characters long"),
    ).not.toBeInTheDocument();
  });

  it("toggles both password fields independently", async () => {
    const { user } = setup();
    const password = screen.getByLabelText("Password");
    const confirm = screen.getByLabelText("Confirm password");

    const [showPassword, showConfirm] = screen.getAllByTitle("Show password");
    await user.click(showPassword);

    expect(password).toHaveAttribute("type", "text");
    expect(confirm).toHaveAttribute("type", "password");

    await user.click(showConfirm);
    expect(confirm).toHaveAttribute("type", "text");
  });

  it("shows the backend error message", () => {
    setup({ errorMessage: "Username already taken." });

    expect(screen.getByText("Username already taken.")).toBeInTheDocument();
  });

  it("switches back to the login view", async () => {
    const { onSwitchToLogin, user } = setup();

    await user.click(screen.getByRole("button", { name: "Sign in" }));

    expect(onSwitchToLogin).toHaveBeenCalledTimes(1);
  });

  it("disables the form while registration is in progress", () => {
    setup({ isLoading: true });

    expect(screen.getByLabelText("Username")).toBeDisabled();
    expect(screen.getByLabelText("Password")).toBeDisabled();
    expect(screen.getByLabelText("Confirm password")).toBeDisabled();
    expect(screen.getByText("Creating account...")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Creating account/ })).toBeDisabled();
  });
});
