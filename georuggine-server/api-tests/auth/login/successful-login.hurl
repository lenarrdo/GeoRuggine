POST {{base_url}}/api/v1/auth/login
{
    "username": "test1",
    "password": "pwdTest1"
}

HTTP 200
[Asserts]
jsonpath "$.message" == "success"
jsonpath "$.token" exists
