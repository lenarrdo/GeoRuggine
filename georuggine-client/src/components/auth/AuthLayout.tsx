import React from "react";
import { Container, Row, Col } from "react-bootstrap";
import { MapPin } from "lucide-react";
import authBg from "../../assets/auth-bg.jpg";

interface AuthLayoutProps {
  children: React.ReactNode;
}

export const AuthLayout: React.FC<AuthLayoutProps> = ({ children }) => {
  return (
    <Container fluid className="p-0 vh-100 d-flex auth-container">
      <Row className="g-0 h-100 w-100">
        {/* Left column: adaptive visual hero with mountain background and branding */}
        <Col
          xs={12}
          md={5}
          lg={5}
          className="d-none d-md-flex align-items-center justify-content-center position-relative auth-hero-pane"
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
            <p className="auth-brand-tagline mb-0 font-monospace">Track. Communicate. Analyze.</p>
          </div>
        </Col>

        {/* Right column: adaptive input form (Login or Registration) */}
        <Col
          xs={12}
          md={7}
          lg={7}
          className="d-flex align-items-center justify-content-center p-4 auth-form-pane bg-dark"
        >
          <div className="auth-form-wrapper w-100" style={{ maxWidth: "450px" }}>
            {children}
          </div>
        </Col>
      </Row>
    </Container>
  );
};
