# Actix Web - MongoDB API

## Basic Usage

```
$ cargo run
$ curl http://127.0.0.1:8080/user/{username}
{"first_name":"daisuke","last_name":"takayama","username":"takayama_daisuke","email":"webcyou@webcyou.com"}
```

## Create a new User

### Request

`POST /user`

```
curl -X POST -H "Content-Type: application/json" -d '{"first_name": "daisuke", "last_name": "takayama", "username": "takayama_daisuke", "email": "webcyou@webcyou.com"}' http://127.0.0.1:8080/user
```

```json
{
  "first_name":"daisuke",
  "last_name":"takayama",
  "username":"takayama_daisuke",
  "email":"webcyou@webcyou.com"
}
```

### Response


## Article URL


## Author

**Daisuke Takayama**
* [@webcyou](https://twitter.com/webcyou)
* [@panicdragon](https://twitter.com/panicdragon)
* <https://github.com/webcyou>
* <https://github.com/webcyou-org>
* <https://github.com/panicdragon>
* <https://www.webcyou.com/>

