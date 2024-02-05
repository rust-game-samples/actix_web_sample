# Actix Web - MongoDB User JWT Auth API

Combines [simple JWT authentication](https://github.com/rust-game-samples/actix_web_sample/tree/main/jwt_auth) with a [user operation API](https://github.com/rust-game-samples/actix_web_sample/tree/main/mongodb).

Simple example of MongoDB usage with Actix Web. For more information on the MongoDB Rust driver, visit the [documentation](https://docs.rs/mongodb/2.0.0/mongodb/index.html) and [source code](https://github.com/mongodb/mongo-rust-driver).

## Basic Usage

### Install MongoDB

Visit the [MongoDB Download Center](https://www.mongodb.com/try) for instructions on how to use MongoDB Atlas or set up MongoDB locally.

### Install MongoDB with Homebrew

To install MongoDB with Homebrew, follow these steps.

```shell
brew tap mongodb/brew
```
```shell
brew install mongodb-community
```

**Verification of Versions**

```shell
mongod --version
```
**start/stop method**

```shell
brew services start mongodb-community
```
```shell
brew services stop mongodb-community
```

### Set an environment variable

The example code creates a client with the URI set by the MONGODB_URI environment variable. The default URI for a standalone mongod running on localhost is "mongodb://localhost:27017". For more information on MongoDB URIs, visit the connection string entry in the MongoDB manual.

### Run the example

```shell
cargo run
```

## Create a new User

### Request

`POST /register`

```shell
curl -X POST http://127.0.0.1:8080/register -H "Content-Type: application/json" -d '{"email":"webcyou@webcyou.com", "password": "1234"}'
```

```json
{
  "email":"webcyou@webcyou.com",
  "password": "1234"
}
```

### Response

```json
{
  "message":"Signup successfully",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjg0MTE0NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiOGNhZDk2MmItYWVhYy00MmMzLWFjNTgtOGYwNTdkODg0YmQzIiwicmVmcmVzaCI6ZmFsc2V9.mm1hUxevMWoWaNhSCfzKEmry6117Fc355AMxnSZ6E6A",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjkyNzQ4NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiZWMxMmRhZTItZWE1Ni00NGZjLWI3ZmQtNTk4NDkyMjc3YmExIiwicmVmcmVzaCI6dHJ1ZX0.OLyYaST_mkIMbZYUU6-QCfT6dYT3URmoUERGQJ5Kwl4"
  }
}
```

## Login User

### Request

`POST /login`

```shell
curl -X POST http://127.0.0.1:8080/login -H "Content-Type: application/json" -d '{"email":"webcyou@webcyou.com", "password": "1234"}'
```

```json
{
  "email": "webcyou@webcyou.com",
  "password": "1234"
}
```

### Response

```json
{
  "message": "Login successfully",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjg0MTE0NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiOGNhZDk2MmItYWVhYy00MmMzLWFjNTgtOGYwNTdkODg0YmQzIiwicmVmcmVzaCI6ZmFsc2V9.mm1hUxevMWoWaNhSCfzKEmry6117Fc355AMxnSZ6E6A",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjkyNzQ4NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiZWMxMmRhZTItZWE1Ni00NGZjLWI3ZmQtNTk4NDkyMjc3YmExIiwicmVmcmVzaCI6dHJ1ZX0.OLyYaST_mkIMbZYUU6-QCfT6dYT3URmoUERGQJ5Kwl4"
  }
}
```

## Get a specific User

### Request

`GET /user/uuid`

```shell
curl -H GET http://127.0.0.1:8080/user/{uuid} -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]'
```

### Response

```shell
{
  "message": "ok",
  "data": { 
    "uuid": "4b19e13e-73b4-4974-9016-eaa047fb3a63",
    "first_name": "daisuke",
    "last_name": "takayama",
    "username": "takayama_daisuke",
    "email": "webcyou@webcyou.com"
  }
}
```

## Change a User

`PUT /user/uuid`

```shell
curl -X PUT -H "Content-Type: application/json" -d '{"first_name": "daisuke", "last_name": "takayama", "username": "takayama_daisuke", "email": "webcyou@webcyou.com"}' http://127.0.0.1:8080/user/{uuid}
```

### Response

```shell

```

## DELETE a User

`PUT /user/uuid`

```shell
curl -X DELETE http://127.0.0.1:8080/user/{uuid} -H "Content-Type: application/json"
```

### Response

```shell
{"uuid":"4b19e13e-73b4-4974-9016-eaa047fb3a63"}
```

## Refresh Token
### Request

```POST /token/refresh```

```shell
curl -X POST http://127.0.0.1:8080/token/refresh -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (refresh_token)]'
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

## Article URL


## Author

**Daisuke Takayama**
* [@webcyou](https://twitter.com/webcyou)
* [@panicdragon](https://twitter.com/panicdragon)
* <https://github.com/webcyou>
* <https://github.com/webcyou-org>
* <https://github.com/panicdragon>
* <https://www.webcyou.com/>

