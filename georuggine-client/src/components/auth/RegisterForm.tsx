import React, { useState } from "react";
import { Card, Form, Button, Alert, InputGroup, Spinner } from "react-bootstrap";
import { User, Lock, Eye, EyeOff } from "lucide-react";
import { RegisterFormData } from "../../types/auth";

interface RegisterFormProps {
  onSubmit: (data: RegisterFormData) => Promise<void>;
  onSwitchToLogin: () => void;
  isLoading: boolean;
  errorMessage: string | null;
}

export const RegisterForm: React.FC<RegisterFormProps> = ({
  onSubmit,
  onSwitchToLogin,
  isLoading,
  errorMessage,
}) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");

  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setValidationError(null);

    if (!username.trim()) {
      setValidationError("Username is required");
      return;
    }

    if (password.length < 4) {
      setValidationError("Password must be at least 4 characters long");
      return;
    }

    if (password !== confirmPassword) {
      setValidationError("Passwords do not match");
      return;
    }

    await onSubmit({
      username: username.trim(),
      password,
      confirmPassword,
    });
  };

  const displayedError = validationError || errorMessage;

  return (
    <Card className="bg-dark text-light border-secondary shadow-lg p-3 auth-form-card">
      <Card.Body>
        <div className="auth-form-header mb-4 border-bottom border-secondary pb-3">
          <Card.Title as="h2" className="h4 fw-bold font-monospace text-light mb-1 auth-title">
            Create your account
          </Card.Title>
          <Card.Subtitle className="text-secondary small font-monospace auth-subtitle">
            Fill in the fields below to sign up
          </Card.Subtitle>
        </div>

        {displayedError && (
          <Alert variant="danger" className="py-2 px-3 small font-monospace d-flex align-items-center mb-3">
            <span>{displayedError}</span>
          </Alert>
        )}

        <Form className="auth-form" onSubmit={handleSubmit}>
          {/* Username field */}
          <Form.Group className="mb-3" controlId="register-username">
            <Form.Label className="small text-secondary font-monospace mb-1">
              Username
            </Form.Label>
            <InputGroup>
              <InputGroup.Text className="bg-dark border-secondary text-secondary">
                <User size={16} />
              </InputGroup.Text>
              <Form.Control
                type="text"
                className="bg-dark text-light border-secondary font-monospace"
                placeholder="e.g. user_1"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
                autoComplete="username"
                disabled={isLoading}
              />
            </InputGroup>
          </Form.Group>

          {/* Password field */}
          <Form.Group className="mb-3" controlId="register-password">
            <Form.Label className="small text-secondary font-monospace mb-1">
              Password
            </Form.Label>
            <InputGroup>
              <InputGroup.Text className="bg-dark border-secondary text-secondary">
                <Lock size={16} />
              </InputGroup.Text>
              <Form.Control
                type={showPassword ? "text" : "password"}
                className="bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                autoComplete="new-password"
                disabled={isLoading}
              />
              <Button
                variant="outline-secondary"
                className="border-secondary text-secondary"
                onClick={() => setShowPassword(!showPassword)}
                tabIndex={-1}
                title={showPassword ? "Hide password" : "Show password"}
              >
                {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
              </Button>
            </InputGroup>
          </Form.Group>

          {/* Confirm password field */}
          <Form.Group className="mb-4" controlId="register-confirm-password">
            <Form.Label className="small text-secondary font-monospace mb-1">
              Confirm password
            </Form.Label>
            <InputGroup>
              <InputGroup.Text className="bg-dark border-secondary text-secondary">
                <Lock size={16} />
              </InputGroup.Text>
              <Form.Control
                type={showConfirmPassword ? "text" : "password"}
                className="bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                required
                autoComplete="new-password"
                disabled={isLoading}
              />
              <Button
                variant="outline-secondary"
                className="border-secondary text-secondary"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                tabIndex={-1}
                title={showConfirmPassword ? "Hide password" : "Show password"}
              >
                {showConfirmPassword ? <EyeOff size={16} /> : <Eye size={16} />}
              </Button>
            </InputGroup>
          </Form.Group>

          {/* Sign up button */}
          <Button
            type="submit"
            variant="primary"
            className="w-100 py-2 font-monospace fw-semibold shadow-sm mb-3 auth-submit-btn"
            disabled={isLoading || !username.trim() || !password || !confirmPassword}
          >
            {isLoading ? (
              <span className="d-flex align-items-center justify-content-center gap-2">
                <Spinner animation="border" size="sm" />
                Creating account...
              </span>
            ) : (
              "Sign up"
            )}
          </Button>

          {/* Link back to login */}
          <div className="text-center font-monospace small text-secondary">
            <span>Already have an account? </span>
            <Button
              variant="link"
              className="text-info p-0 font-monospace text-decoration-none fw-semibold"
              onClick={onSwitchToLogin}
              disabled={isLoading}
            >
              Sign in
            </Button>
          </div>
        </Form>
      </Card.Body>
    </Card>
  );
};
