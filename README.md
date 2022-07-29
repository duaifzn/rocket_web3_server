### seed
```
cargo run --example seed local
```
___
### sign up admin account
#### step1

- sign up a user permision account
- Method: **POST**
- URL: ```/api/signup```
- Body:
```
{
    "email" : "admin123@gmail.com",
    "password" : "admin123",
}
```
```
curl -d '{"email":"admin123@gmail.com", "password":"admin123"}' -H "Content-Type: application/json" -X POST http://211.73.81.185/api/signup
```
#### step2

- change role colume to 0 in database
___
### transfer money
#### step1

- get admin token
- Method: **POST**
- URL: ```/api/signin```
- Body:
```
{
    "email" : "admin123@gmail.com",
    "password" : "admin123",
}
```
```
curl -d '{"email":"admin123@gmail.com", "password":"admin123"}' -H "Content-Type: application/json" -X POST http://211.73.81.185/api/signin
```
#### step2

- transfer money by admin token
- Method: **POST**
- URL: ```/api/admin/money/transfer```
- Headers： Authorization: admin token
- Body:
```
{
  "to_account_name": "example@gmail.com"
}
```
```
curl -d '{"to_account_name":"admin123@gmail.com"}' -H "Content-Type: application/json" -H "Authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjEifQ.eyJleHAiOjE2NTg0NjYzMDd9.dJHHTHt729LBO5yOz_4dwQLpVBI4Y3epys6kApypJ2jiSBzphALqa1SdDIWXFUwmH9qFlJQfjV_Fv7aZbrg-ag" -X POST http://211.73.81.185/api/admin/money/transfer
```

#### step3

- get account balance
- Method: **GET**
- URL: ```/api/balance?account_name=example@gmail.com```
- Headers： Authorization: token
```
curl -H "Authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjEifQ.eyJleHAiOjE2NTg0NjU4MjR9.4_PsBrFk81PIQLeqHPfKCkw6fX2FOsrcRYMeAyOIF9R8DTOuKndE3RX8ziZntX5uagPWlRsHXhpTOOaxDOmGrQ" -X GET http://211.73.81.185/api/balance?account_name=admin123@gmail.com
```