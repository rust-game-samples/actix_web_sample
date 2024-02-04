# Actix Web - MongoDB API

Refactoring based on [this](https://github.com/actix/examples/tree/master/databases/mongodb) sample.

Simple example of MongoDB usage with Actix Web. For more information on the MongoDB Rust driver, visit the [documentation](https://docs.rs/mongodb/2.0.0/mongodb/index.html) and [source code](https://github.com/mongodb/mongo-rust-driver).

## Basic Usage

### Install MongoDB

Visit the [MongoDB Download Center](https://www.mongodb.com/try) for instructions on how to use MongoDB Atlas or set up MongoDB locally.

### Set an environment variable

The example code creates a client with the URI set by the MONGODB_URI environment variable. The default URI for a standalone mongod running on localhost is "mongodb://localhost:27017". For more information on MongoDB URIs, visit the connection string entry in the MongoDB manual.


curl -X POST 'http://127.0.0.1:8080/register' -H "Content-Type: application/json" -d '{"email": "webcyou@webcyou.com", "password": "1234"}'


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
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjg0MTE0NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiOGNhZDk2MmItYWVhYy00MmMzLWFjNTgtOGYwNTdkODg0YmQzIiwicmVmcmVzaCI6ZmFsc2V9.mm1hUxevMWoWaNhSCfzKEmry6117Fc355AMxnSZ6E6A",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MDY4NDEwODUsImV4cCI6MTcwNjkyNzQ4NSwibmJmIjoxNzA2ODQxMDg1LCJzdWIiOiIxIiwianRpIjoiZWMxMmRhZTItZWE1Ni00NGZjLWI3ZmQtNTk4NDkyMjc3YmExIiwicmVmcmVzaCI6dHJ1ZX0.OLyYaST_mkIMbZYUU6-QCfT6dYT3URmoUERGQJ5Kwl4"
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
  "email":"webcyou@webcyou.com",
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

## Get a specific User

### Request

`GET /user/uuid`

```shell
curl http://127.0.0.1:8080/user/{uuid}
```

### Response

```shell
{"uuid":"4b19e13e-73b4-4974-9016-eaa047fb3a63","first_name":"daisuke","last_name":"takayama","username":"takayama_daisuke","email":"webcyou@webcyou.com"}
```

## Change a User

`PUT /user/uuid`

```shell
curl -X PUT -H "Content-Type: application/json" -d '{"first_name": "daisuke", "last_name": "takayama", "username": "takayama_daisuke", "email": "webcyou@webcyou.com"}' http://127.0.0.1:8080/user/{uuid}
```

### Response

```shell
{"uuid":"4b19e13e-73b4-4974-9016-eaa047fb3a63"}
```

## DELETE a User

`PUT /user/uuid`

```shell
curl -X DELETE -H "Content-Type: application/json" http://127.0.0.1:8080/user/{uuid}
```

### Response

```shell
{"uuid":"4b19e13e-73b4-4974-9016-eaa047fb3a63"}
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

