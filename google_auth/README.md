# Actix Web - Google Auth API

This is a simple demo of logging in with a google account.

## Basic Usage

### Set an environment variable

Set google client id and google client secret to environment variables.

```shell
GOOGLE_CLIENT_ID="your google client id"
GOOGLE_CLIENT_SECRET="your google client secret"
```

Add http://127.0.0.1:8080/auth/google/callback to the approved redirect URIs.


### Run the example

```shell
cargo run
```

## Login User

Open http://127.0.0.1:8080/auth/google/login in your browser.

### Request

`GET /auth/google/login`

The Google OAuth consent screen will appear.

Callback will retrieve your email and display it on the screen.

### Response

Receive Callback with the following API URI.

`GET /auth/google/callback`

Show email of logged in account.

```shell
email: "email address"
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

