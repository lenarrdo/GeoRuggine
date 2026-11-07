# Try to login using an unregistered user
POST {{base_url}}/api/v1/auth/login
{
    "username": "test1",
    "password": "pwdtest1"
}
HTTP 401
