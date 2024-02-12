# Actix Web - Simple JWT Auth

This is a simple demo of handling Actix Web and JWT and creating it as an API.

For more information on JWT specifications, please click [here](https://jwt.io/).

## Basic Usage
### Run the example

```shell
cargo run
```

## Create a new Token
### Request

`POST /token/`

```shell
curl -X POST 'http://127.0.0.1:8080/token/' -H "Content-Type: application/json" -d '{"username": "daisuke", "password": "1234"}'
```

```json
{
  "username": "daisuke",
  "password": "1234"
}
```

### Response

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjg0MTE0NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiOGNhZDk2MmItYWVhYy00MmMzLWFjNTgtOGYwNTdkODg0YmQzIiwicmVmcmVzaCI6ZmFsc2V9.mm1hUxevMWoWaNhSCfzKEmry6117Fc355AMxnSZ6E6A",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjkyNzQ4NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiZWMxMmRhZTItZWE1Ni00NGZjLWI3ZmQtNTk4NDkyMjc3YmExIiwicmVmcmVzaCI6dHJ1ZX0.OLyYaST_mkIMbZYUU6-QCfT6dYT3URmoUERGQJ5Kwl4"
}
```

## Refresh Token
### Request

```POST /token/refresh```

```shell
curl -X POST 'http://127.0.0.1:8080/token/refresh' -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (refresh_token)]'
````

*Headers*

```
Authorization [JWT token（refresh_token）]
```

### Response

```json
{
  "token": "ejdwqjdoqw ...."
}
```

## Authorized Access

`GET /hello`

```shell
curl -H GET 'http://127.0.0.1:8080/hello' -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]'
```

*Headers*
```
Authorization [JWT token （token）]
```

### Response

```
Authorized Access Success! Hello World!
```

## Author

**Daisuke Takayama**
* [@webcyou](https://twitter.com/webcyou)
* [@panicdragon](https://twitter.com/panicdragon)
* <https://github.com/webcyou>
* <https://github.com/webcyou-org>
* <https://github.com/panicdragon>
* <https://www.webcyou.com/>
