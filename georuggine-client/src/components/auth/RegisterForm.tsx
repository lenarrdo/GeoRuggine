import React, { useState } from "react";
import { User, Mail, Lock, Eye, EyeOff, Loader2 } from "lucide-react";
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
  const [nome, setNome] = useState("");
  const [cognome, setCognome] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confermaPassword, setConfermaPassword] = useState("");

  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setValidationError(null);

    if (!nome.trim() || !cognome.trim() || !email.trim()) {
      setValidationError("Tutti i campi sono obbligatori");
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
      nome: nome.trim(),
      cognome: cognome.trim(),
      email: email.trim(),
      password,
      confermaPassword,
    });
  };

  const displayedError = validationError || errorMessage;

  return (
    <div className="card bg-dark text-light border-secondary shadow-lg p-3 auth-form-card">
      <div className="card-body">
        <div className="auth-form-header mb-4 border-bottom border-secondary pb-3">
          <h2 className="card-title h4 fw-bold font-monospace text-light mb-1 auth-title">
            Crea il tuo account
          </h2>
          <p className="card-subtitle text-secondary small font-monospace auth-subtitle">
            Compila i campi qui sotto per registrarti
          </p>
        </div>

        {displayedError && (
          <div className="alert alert-danger py-2 px-3 small font-monospace d-flex align-items-center mb-3">
            <span>{displayedError}</span>
          </div>
        )}

        <form className="auth-form" onSubmit={handleSubmit}>
          {/* Nome e Cognome affiancati in row */}
          <div className="row g-2 mb-3">
            <div className="col-6">
              <label className="form-label small text-secondary font-monospace mb-1" htmlFor="register-nome">
                Nome
              </label>
              <div className="input-group">
                <span className="input-group-text bg-dark border-secondary text-secondary">
                  <User size={16} />
                </span>
                <input
                  id="register-nome"
                  type="text"
                  className="form-control bg-dark text-light border-secondary font-monospace"
                  placeholder="Nome"
                  value={nome}
                  onChange={(e) => setNome(e.target.value)}
                  required
                  autoComplete="given-name"
                  disabled={isLoading}
                />
              </div>
            </div>
            <div className="col-6">
              <label className="form-label small text-secondary font-monospace mb-1" htmlFor="register-cognome">
                Cognome
              </label>
              <div className="input-group">
                <span className="input-group-text bg-dark border-secondary text-secondary">
                  <User size={16} />
                </span>
                <input
                  id="register-cognome"
                  type="text"
                  className="form-control bg-dark text-light border-secondary font-monospace"
                  placeholder="Cognome"
                  value={cognome}
                  onChange={(e) => setCognome(e.target.value)}
                  required
                  autoComplete="family-name"
                  disabled={isLoading}
                />
              </div>
            </div>
          </div>

          {/* Campo Email */}
          <div className="mb-3">
            <label className="form-label small text-secondary font-monospace mb-1" htmlFor="register-email">
              Email
            </label>
            <div className="input-group">
              <span className="input-group-text bg-dark border-secondary text-secondary">
                <Mail size={16} />
              </span>
              <input
                id="register-email"
                type="email"
                className="form-control bg-dark text-light border-secondary font-monospace"
                placeholder="nome@dominio.it"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                autoComplete="email"
                disabled={isLoading}
              />
            </div>
          </div>

          {/* Campo Password */}
          <div className="mb-3">
            <label className="form-label small text-secondary font-monospace mb-1" htmlFor="register-password">
              Password
            </label>
            <div className="input-group">
              <span className="input-group-text bg-dark border-secondary text-secondary">
                <Lock size={16} />
              </span>
              <input
                id="register-password"
                type={showPassword ? "text" : "password"}
                className="form-control bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                autoComplete="new-password"
                disabled={isLoading}
              />
              <button
                type="button"
                className="btn btn-outline-secondary border-secondary text-secondary"
                onClick={() => setShowPassword(!showPassword)}
                tabIndex={-1}
                title={showPassword ? "Nascondi password" : "Mostra password"}
              >
                {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
              </button>
            </div>
          </div>

          {/* Campo Conferma Password */}
          <div className="mb-4">
            <label className="form-label small text-secondary font-monospace mb-1" htmlFor="register-confirm-password">
              Conferma password
            </label>
            <div className="input-group">
              <span className="input-group-text bg-dark border-secondary text-secondary">
                <Lock size={16} />
              </span>
              <input
                id="register-confirm-password"
                type={showConfirmPassword ? "text" : "password"}
                className="form-control bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={confermaPassword}
                onChange={(e) => setConfermaPassword(e.target.value)}
                required
                autoComplete="new-password"
                disabled={isLoading}
              />
              <button
                type="button"
                className="btn btn-outline-secondary border-secondary text-secondary"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                tabIndex={-1}
                title={showConfirmPassword ? "Nascondi password" : "Mostra password"}
              >
                {showConfirmPassword ? <EyeOff size={16} /> : <Eye size={16} />}
              </button>
            </div>
          </div>

          {/* Bottone Registrati */}
          <button
            type="submit"
            className="btn btn-primary w-100 py-2 font-monospace fw-semibold shadow-sm mb-3 auth-submit-btn"
            disabled={isLoading || !nome.trim() || !cognome.trim() || !email.trim() || !password}
          >
            {isLoading ? (
              <span className="d-flex align-items-center justify-content-center gap-2">
                <Loader2 size={18} className="spinner" />
                Creazione account...
              </span>
            ) : (
              "Registrati"
            )}
          </button>

          {/* Link verso Login */}
          <div className="text-center font-monospace small text-secondary">
            <span>Hai già un account? </span>
            <button
              type="button"
              className="btn btn-link text-info p-0 font-monospace text-decoration-none fw-semibold"
              onClick={onSwitchToLogin}
              disabled={isLoading}
            >
              Accedi
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
