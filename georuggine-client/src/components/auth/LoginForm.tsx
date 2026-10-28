import React, { useState } from "react";
import { Card, Form, Button, Alert, InputGroup, Spinner } from "react-bootstrap";
import { User, Lock, Eye, EyeOff } from "lucide-react";
import { LoginFormData } from "../../types/auth";

interface LoginFormProps {
  onSubmit: (data: LoginFormData) => Promise<void>;
  onSwitchToRegister: () => void;
  isLoading: boolean;
  errorMessage: string | null;
  successMessage?: string | null;
}

export const LoginForm: React.FC<LoginFormProps> = ({
  onSubmit,
  onSwitchToRegister,
  isLoading,
  errorMessage,
  successMessage,
}) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!username.trim() || !password) return;
    await onSubmit({ username: username.trim(), password });
  };

  return (
    <Card className="bg-dark text-light border-secondary shadow-lg p-3 auth-form-card">
      <Card.Body>
        <div className="auth-form-header mb-4 border-bottom border-secondary pb-3">
          <Card.Title as="h2" className="h4 fw-bold font-monospace text-light mb-1 auth-title">
            Accedi al tuo account
          </Card.Title>
          <Card.Subtitle className="text-secondary small font-monospace auth-subtitle">
            Inserisci le tue credenziali per continuare
          </Card.Subtitle>
        </div>

        {successMessage && (
          <Alert variant="success" className="py-2 px-3 small font-monospace d-flex align-items-center mb-3">
            <span>{successMessage}</span>
          </Alert>
        )}

        {errorMessage && (
          <Alert variant="danger" className="py-2 px-3 small font-monospace d-flex align-items-center mb-3">
            <span>{errorMessage}</span>
          </Alert>
        )}

        <Form className="auth-form" onSubmit={handleSubmit}>
          {/* Campo Username */}
          <Form.Group className="mb-3" controlId="login-username">
            <Form.Label className="small text-secondary font-monospace mb-1">
              Username
            </Form.Label>
            <InputGroup>
              <InputGroup.Text className="bg-dark border-secondary text-secondary">
                <User size={18} />
              </InputGroup.Text>
              <Form.Control
                type="text"
                className="bg-dark text-light border-secondary font-monospace"
                placeholder="es. utente_1"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
                autoComplete="username"
                disabled={isLoading}
              />
            </InputGroup>
          </Form.Group>

          {/* Campo Password con Toggle Visibilità */}
          <Form.Group className="mb-4" controlId="login-password">
            <Form.Label className="small text-secondary font-monospace mb-1">
              Password
            </Form.Label>
            <InputGroup>
              <InputGroup.Text className="bg-dark border-secondary text-secondary">
                <Lock size={18} />
              </InputGroup.Text>
              <Form.Control
                type={showPassword ? "text" : "password"}
                className="bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                autoComplete="current-password"
                disabled={isLoading}
              />
              <Button
                variant="outline-secondary"
                className="border-secondary text-secondary"
                onClick={() => setShowPassword(!showPassword)}
                tabIndex={-1}
                title={showPassword ? "Nascondi password" : "Mostra password"}
              >
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </Button>
            </InputGroup>
          </Form.Group>

          {/* Bottone Accedi */}
          <Button
            type="submit"
            variant="primary"
            className="w-100 py-2 font-monospace fw-semibold shadow-sm mb-3 auth-submit-btn"
            disabled={isLoading || !username.trim() || !password}
          >
            {isLoading ? (
              <span className="d-flex align-items-center justify-content-center gap-2">
                <Spinner animation="border" size="sm" />
                Accesso in corso...
              </span>
            ) : (
              "Accedi"
            )}
          </Button>

          {/* Link verso Registrazione */}
          <div className="text-center font-monospace small text-secondary">
            <span>Non hai un account? </span>
            <Button
              variant="link"
              className="text-info p-0 font-monospace text-decoration-none fw-semibold"
              onClick={onSwitchToRegister}
              disabled={isLoading}
            >
              Registrati
            </Button>
          </div>
        </Form>
      </Card.Body>
    </Card>
  );
};
