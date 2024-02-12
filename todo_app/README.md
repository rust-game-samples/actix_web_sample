# Actix Web - MongoDB Todo App API

This is a sample of a simple todo application that includes authentication and authorization using JWT, and CRUD processing for each user and todo.

Combines [simple JWT authentication](https://github.com/rust-game-samples/actix_web_sample/tree/main/jwt_auth) with a [user operation API](https://github.com/rust-game-samples/actix_web_sample/tree/main/mongodb).

Simple example of MongoDB usage with Actix Web. For more information on the MongoDB Rust driver, visit the [documentation](https://docs.rs/mongodb/2.0.0/mongodb/index.html) and [source code](https://github.com/mongodb/mongo-rust-driver).

For more information on JWT specifications, please click [here](https://jwt.io/).

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

**request body example**
```json
{
  "email": "webcyou@webcyou.com",
  "password": "1234"
}
```

### Response

```json
{
  "message": "Signup successfully",
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

**request body example**

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

`GET /user/`

```shell
curl -H GET http://127.0.0.1:8080/user/ -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]'
```

### Response

```json
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

`PUT /user/`

```shell
curl -X PUT http://127.0.0.1:8080/user/ -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT Token (token)]' -d '{"first_name": "daisuke", "last_name": "takayama", "username": "takayama_daisuke", "email": "webcyou@webcyou.com"}'
```

### Response

```json
{
  "message": "ok",
  "data": {
    "uuid": "0b957f89-99ec-4153-a872-888763f9bf2d",
    "first_name": "daisuke",
    "last_name": "takayama",
    "username": "takayama_daisuke",
    "email": "webcyou@webcyou.com"
  }
}
```

## DELETE a User

`PUT /user/`

```shell
curl -X DELETE http://127.0.0.1:8080/user/ -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT Token (token)]'
```

### Response

```json
{
  "message":"ok",
  "data": "4b19e13e-73b4-4974-9016-eaa047fb3a63"
}
```

## Refresh Token
### Request

`POST /token/refresh`

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
  "message": "ok",
  "data": {
    "token": "ejdwqjdoqw ...."
  }
}
```

## Create a new Todo
### Request

`POST /todos/`

```shell
curl -X POST http://127.0.0.1:8080/todos/ -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT Token (token)]' -d '{"title":"my_first_todo"}'
```

**request body example**
```json
{
  "title": "my_first_todo"
}
```

### Response

```json
{
  "message": "ok",
  "data": {
    "uuid": "1fff5f11-c91b-4b59-bc3a-58254a661ccc",
    "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
    "title": "my_first_todo",
    "state": "NotStarted"
  }
}
```

## Get Todo list
### Request

`GET /todos/`

```shell
curl -X GET http://127.0.0.1:8080/todos/ -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]'
```

**request body example（optional）**
```json
{
  "page": 2,
  "page_size": 30
}
```

* Page and page_size are fine without them, and the default settings are 1 for page and 30 for page_size.

### Response

```json
{
    "message": "ok",
    "data": [
        {
            "uuid": "c1e7bf36-c631-4bde-9ae9-9cd939bbcecc",
            "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
            "title": "test_todo1",
            "state": "Completed"
        },
        {
            "uuid": "2278dc90-4ced-4e30-ad46-64961c8e3f15",
            "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
            "title": "test_todo2",
            "state": "NotStarted"
        },
        {
            "uuid": "1fff5f11-c91b-4b59-bc3a-58254a661ccc",
            "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
            "title": "my_first_todo",
            "state": "NotStarted"
        }
    ]
}
```

## Get a specific Todo
### Request

`GET /todos/{uuid}`

```shell
curl -X GET http://127.0.0.1:8080/todos/{uuid} -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]'
```

### Response

```json
{
    "message": "ok",
    "data": {
        "uuid": "c1e7bf36-c631-4bde-9ae9-9cd939bbcecc",
        "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
        "title": "test_todo1",
        "state": "Completed"
    }
}
```

## Change a Todo
### Request

`PUT /todos/{uuid}`

```shell
curl -X PUT http://127.0.0.1:8080/todos/{uuid} -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]' -d '{"uuid": "c1e7bf36-c631-4bde-9ae9-9cd939bbcecc", "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f", "title": "test_todo1_a", "state": "Completed"}'
```

**request body example**
```json
{
    "uuid": "c1e7bf36-c631-4bde-9ae9-9cd939bbcecc",
    "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
    "title": "test_todo1_a",
    "state": "Completed"
}
```

### Response

```json
{
    "message": "ok",
    "data": {
        "uuid": "c1e7bf36-c631-4bde-9ae9-9cd939bbcecc",
        "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
        "title": "test_todo1_a",
        "state": "Completed"
    }
}
```

## Change a Todo State
### Request

`PUT /todos/{uuid}/state`

```shell
curl -X PUT http://127.0.0.1:8080/todos/{uuid} -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]' -d '{"state": "InProgress"}'
```

**request body example**
```json
{
    "state": "InProgress"
}
```

### Response

```json
{
    "message": "ok",
    "data": {
        "uuid": "c1e7bf36-c631-4bde-9ae9-9cd939bbcecc",
        "user_id": "e65bfa19-3b71-497d-abda-978ede36b30f",
        "title": "test_todo1_a",
        "state": "InProgress"
    }
}
```

## DELETE a Todo
### Request

`DELETE /todos/{uuid}`

```shell
curl -X DELETE http://127.0.0.1:8080/todos/{uuid} -H 'Content-Type: application/json' -H 'Authorization: Bearer [JWT Token (token)]'
```

### Response

```json
{
    "message": "ok",
    "data": null
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

