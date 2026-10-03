# Create the same user twice.
# First attempt is successful.
POST {{base_url}}/api/v1/auth/register
{
    "username": "test1",
    "password": "pwdTest1"
}
HTTP 201

# But the second one returns a conflict.
POST {{base_url}}/api/v1/auth/register
{
    "username": "test1",
    "password": "pwdTest1"
}
HTTP 409
