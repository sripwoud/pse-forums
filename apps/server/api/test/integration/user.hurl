GET http://localhost:3000/rspc/user.read?input=%22user1%22
HTTP 200
[Asserts]
jsonpath "$.result.data.id" == 1
jsonpath "$.result.data.email" == "user1@example.com"
jsonpath "$.result.data.username" == "user1"
jsonpath "$.result.data.encrypted_password" not exists
jsonpath "$.result.data.salt" not exists

POST http://localhost:3000/rspc/user.delete
{
  "username": "user1"
}
HTTP 200
[Asserts]
jsonpath "$.result.data.code" == 401
jsonpath "$.result.data.message" == "No Bearer token provided"
