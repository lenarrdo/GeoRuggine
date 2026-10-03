# User tries to login without providing its password
POST {{base_url}}/api/v1/auth/login
{
    "username": "test1"
}
HTTP 400
