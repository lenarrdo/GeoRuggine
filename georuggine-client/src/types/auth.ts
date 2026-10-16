export interface UserSession {
  username: string;
  token: string;
}

export interface AuthResponse {
  message: string;
  token?: string | null;
}

export interface RegisterFormData {
  nome: string;
  cognome: string;
  email: string;
  password: string;
  confermaPassword: string;
}

export interface LoginFormData {
  email: string;
  password: string;
}
