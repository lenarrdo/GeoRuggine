# Try to login using an unregistered user
POST {{base_url}}/api/v1/auth/login
{
    "username": "Unregistered",
    "password": "asdf"
}
HTTP 401
