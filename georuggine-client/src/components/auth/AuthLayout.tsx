import React from "react";
import { MapPin } from "lucide-react";
import authBg from "../../assets/auth-bg.jpg";

interface AuthLayoutProps {
  children: React.ReactNode;
}

export const AuthLayout: React.FC<AuthLayoutProps> = ({ children }) => {
  return (
    <div className="container-fluid p-0 vh-100 d-flex auth-container">
      <div className="row g-0 h-100 w-100">
        {/* Colonna Sinistra: Hero visivo con sfondo montano e branding */}
        <div
          className="col-md-5 d-none d-md-flex align-items-center justify-content-center position-relative auth-hero-pane"
          style={{ backgroundImage: `url(${authBg})` }}
        >
          <div className="auth-hero-overlay position-absolute top-0 start-0 w-100 h-100"></div>
          <div className="auth-hero-content position-relative z-1 text-center p-4">
            <div className="auth-brand d-flex align-items-center justify-content-center gap-3 mb-2">
              <div className="auth-brand-icon-wrapper rounded-3 p-2">
                <MapPin size={28} className="auth-pin-icon" />
              </div>
              <h1 className="auth-brand-title mb-0">GeoRuggine</h1>
            </div>
            <p className="auth-brand-tagline mb-0 font-monospace">Traccia. Comunica. Analizza.</p>
          </div>
        </div>

        {/* Colonna Destra: Modulo di input (Login o Registrazione) */}
        <div className="col-12 col-md-7 d-flex align-items-center justify-content-center p-4 auth-form-pane bg-dark">
          <div className="auth-form-wrapper w-100" style={{ maxWidth: "420px" }}>
            {children}
          </div>
        </div>
      </div>
    </div>
  );
};
