POST {{base_url}}/api/v1/auth/register
{
    "username": "test1"
}
HTTP 400
[Asserts]
jsonpath "$.message" == "Both username and password are required."

