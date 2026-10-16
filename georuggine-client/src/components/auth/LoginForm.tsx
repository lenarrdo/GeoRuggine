import React, { useState } from "react";
import { Mail, Lock, Eye, EyeOff, Loader2 } from "lucide-react";
import { LoginFormData } from "../../types/auth";

interface LoginFormProps {
  onSubmit: (data: LoginFormData) => Promise<void>;
  onSwitchToRegister: () => void;
  isLoading: boolean;
  errorMessage: string | null;
}

export const LoginForm: React.FC<LoginFormProps> = ({
  onSubmit,
  onSwitchToRegister,
  isLoading,
  errorMessage,
}) => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!email.trim() || !password) return;
    await onSubmit({ email, password });
  };

  return (
    <div className="card bg-dark text-light border-secondary shadow-lg p-3 auth-form-card">
      <div className="card-body">
        <div className="auth-form-header mb-4 border-bottom border-secondary pb-3">
          <h2 className="card-title h4 fw-bold font-monospace text-light mb-1 auth-title">
            Accedi al tuo account
          </h2>
          <p className="card-subtitle text-secondary small font-monospace auth-subtitle">
            Inserisci le tue credenziali per continuare
          </p>
        </div>

        {errorMessage && (
          <div className="alert alert-danger py-2 px-3 small font-monospace d-flex align-items-center mb-3">
            <span>{errorMessage}</span>
          </div>
        )}

        <form className="auth-form" onSubmit={handleSubmit}>
          {/* Campo Email */}
          <div className="mb-3">
            <label className="form-label small text-secondary font-monospace mb-1" htmlFor="login-email">
              Email
            </label>
            <div className="input-group">
              <span className="input-group-text bg-dark border-secondary text-secondary">
                <Mail size={18} />
              </span>
              <input
                id="login-email"
                type="text"
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

          {/* Campo Password con Toggle Visibilità */}
          <div className="mb-4">
            <label className="form-label small text-secondary font-monospace mb-1" htmlFor="login-password">
              Password
            </label>
            <div className="input-group">
              <span className="input-group-text bg-dark border-secondary text-secondary">
                <Lock size={18} />
              </span>
              <input
                id="login-password"
                type={showPassword ? "text" : "password"}
                className="form-control bg-dark text-light border-secondary font-monospace"
                placeholder="••••••••"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                autoComplete="current-password"
                disabled={isLoading}
              />
              <button
                type="button"
                className="btn btn-outline-secondary border-secondary text-secondary"
                onClick={() => setShowPassword(!showPassword)}
                tabIndex={-1}
                title={showPassword ? "Nascondi password" : "Mostra password"}
              >
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>
          </div>

          {/* Bottone Accedi */}
          <button
            type="submit"
            className="btn btn-primary w-100 py-2 font-monospace fw-semibold shadow-sm mb-3 auth-submit-btn"
            disabled={isLoading || !email.trim() || !password}
          >
            {isLoading ? (
              <span className="d-flex align-items-center justify-content-center gap-2">
                <Loader2 size={18} className="spinner" />
                Accesso in corso...
              </span>
            ) : (
              "Accedi"
            )}
          </button>

          {/* Link verso Registrazione */}
          <div className="text-center font-monospace small text-secondary">
            <span>Non hai un account? </span>
            <button
              type="button"
              className="btn btn-link text-info p-0 font-monospace text-decoration-none fw-semibold"
              onClick={onSwitchToRegister}
              disabled={isLoading}
            >
              Registrati
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
