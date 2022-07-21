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

#### step3

- get account balance
- Method: **GET**
- URL: ```/api/balance?account_name=example@gmail.com```
- Headers： Authorization: token