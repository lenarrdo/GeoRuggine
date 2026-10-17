export interface UserSession {
  username: string;
}

export interface AuthResult {
  success: boolean;
  message: string;
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
