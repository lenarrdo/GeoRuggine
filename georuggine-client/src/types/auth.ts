export interface UserSession {
  username: string;
}

export interface AuthResult {
  success: boolean;
  message: string;
}

export interface RegisterFormData {
  username: string;
  password: string;
  confermaPassword: string;
}

export interface LoginFormData {
  username: string;
  password: string;
}
