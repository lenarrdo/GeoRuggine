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
  const [confermaPassword, setConfermaPassword] = useState("");

  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setValidationError(null);

    if (!username.trim()) {
      setValidationError("Lo username è obbligatorio");
      return;
    }

    if (password.length < 4) {
      setValidationError("La password deve contenere almeno 4 caratteri");
      return;
    }

    if (password !== confermaPassword) {
      setValidationError("Le password non coincidono");
      return;
    }

    await onSubmit({
      username: username.trim(),
      password,
      confermaPassword,
    });
  };

  const displayedError = validationError || errorMessage;

  return (
    <Card className="bg-dark text-light border-secondary shadow-lg p-3 auth-form-card">
      <Card.Body>
        <div className="auth-form-header mb-4 border-bottom border-secondary pb-3">
          <Card.Title as="h2" className="h4 fw-bold font-monospace text-light mb-1 auth-title">
            Crea il tuo account
          </Card.Title>
          <Card.Subtitle className="text-secondary small font-monospace auth-subtitle">
            Compila i campi qui sotto per registrarti
          </Card.Subtitle>
        </div>

        {displayedError && (
          <Alert variant="danger" className="py-2 px-3 small font-monospace d-flex align-items-center mb-3">
            <span>{displayedError}</span>
          </Alert>
        )}

        <Form className="auth-form" onSubmit={handleSubmit}>
          {/* Campo Username */}
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
                placeholder="es. utente_1"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
                autoComplete="username"
                disabled={isLoading}
              />
            </InputGroup>
          </Form.Group>

          {/* Campo Password */}
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
                title={showPassword ? "Nascondi password" : "Mostra password"}
              >
                {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
              </Button>
            </InputGroup>
          </Form.Group>

          {/* Campo Conferma Password */}
          <Form.Group className="mb-4" controlId="register-confirm-password">
            <Form.Label className="small text-secondary font-monospace mb-1">
              Conferma password
            </Form.Label>
            <InputGroup>
              <InputGroup.Text className="bg-dark border-secondary text-secondary">
                <Lock size={16} />
              </InputGroup.Text>
              <Form.Control
                type={showConfirmPassword ? "text" : "password"}
                className="bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={confermaPassword}
                onChange={(e) => setConfermaPassword(e.target.value)}
                required
                autoComplete="new-password"
                disabled={isLoading}
              />
              <Button
                variant="outline-secondary"
                className="border-secondary text-secondary"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                tabIndex={-1}
                title={showConfirmPassword ? "Nascondi password" : "Mostra password"}
              >
                {showConfirmPassword ? <EyeOff size={16} /> : <Eye size={16} />}
              </Button>
            </InputGroup>
          </Form.Group>

          {/* Bottone Registrati */}
          <Button
            type="submit"
            variant="primary"
            className="w-100 py-2 font-monospace fw-semibold shadow-sm mb-3 auth-submit-btn"
            disabled={isLoading || !username.trim() || !password || !confermaPassword}
          >
            {isLoading ? (
              <span className="d-flex align-items-center justify-content-center gap-2">
                <Spinner animation="border" size="sm" />
                Creazione account...
              </span>
            ) : (
              "Registrati"
            )}
          </Button>

          {/* Link verso Login */}
          <div className="text-center font-monospace small text-secondary">
            <span>Hai già un account? </span>
            <Button
              variant="link"
              className="text-info p-0 font-monospace text-decoration-none fw-semibold"
              onClick={onSwitchToLogin}
              disabled={isLoading}
            >
              Accedi
            </Button>
          </div>
        </Form>
      </Card.Body>
    </Card>
  );
};
