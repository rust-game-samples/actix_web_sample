pub const DB_NAME: &str = "todoApp";

pub const COLL_NAME_USERS: &str = "users";
pub const COLL_NAME_TODO: &str = "todos";
pub const GOOGLE_CLIENT_ID_ENV: &str = "GOOGLE_CLIENT_ID";
pub const GOOGLE_CLIENT_SECRET_ENV: &str = "GOOGLE_CLIENT_SECRET";

pub const BASE_URL: &str = "http://localhost:8080";

// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";

pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_REFRESH_TOKEN_ERROR: &str = "Refresh tokens are not allowed";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_GOOGLE_CLIENT_ID_MISSING: &str =
    "Missing the GOOGLE_CLIENT_ID environment variable.";
pub const MESSAGE_GOOGLE_CLIENT_SECRET_MISSING: &str =
    "Missing the GOOGLE_CLIENT_SECRET environment variable.";

pub const MESSAGE_USER_INFORMATION_REQUEST_ERROR: &str =
    "User Information Request Error, please login again";

// Misc
pub const EMPTY: &str = "";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";
pub const MESSAGE_BAD_REQUEST: &str = "Bad Request";

// Headers
pub const AUTHORIZATION: &str = "Authorization";
