use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StatusCode(u16);

impl StatusCode {
    pub const OK: StatusCode = StatusCode(200);
    pub const CREATED: StatusCode = StatusCode(201);
    pub const ACCEPTED: StatusCode = StatusCode(202);
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(206);
    pub const MULTIPLE_CHOICES: StatusCode = StatusCode(300);
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301);
    pub const FOUND: StatusCode = StatusCode(302);
    pub const BAD_REQUEST: StatusCode = StatusCode(400);
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);
    pub const FORBIDDEN: StatusCode = StatusCode(403);
    pub const NOT_FOUND: StatusCode = StatusCode(404);
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode(406);
    pub const LENGTH_REQUIRED: StatusCode = StatusCode(411);
    pub const IM_A_TEAPOT: StatusCode = StatusCode(418);
    pub const UPGRADE_REQUIRED: StatusCode = StatusCode(426);
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(429);
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(501);
    pub const BAD_GATEWAY: StatusCode = StatusCode(502);
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(504);
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::OK
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            200 => f.write_str("200 OK"),
            201 => f.write_str("201 CREATED"),
            202 => f.write_str("202 ACCEPTED"),
            206 => f.write_str("206 PARTIAL_CONTENT"),
            300 => f.write_str("300 MULTIPLE_CHOICES"),
            301 => f.write_str("301 MOVED_PERMANENTLY"),
            302 => f.write_str("302 FOUND"),
            400 => f.write_str("400 BAD_REQUEST"),
            401 => f.write_str("400 UNAUTHORIZED"),
            403 => f.write_str("403 FORBIDDEN"),
            404 => f.write_str("404 NOT_FOUND"),
            405 => f.write_str("405 METHOD_NOT_ALLOWED"),
            406 => f.write_str("406 NOT_ACCEPTABLE"),
            411 => f.write_str("411 LENGTH_REQUIRED"),
            418 => f.write_str("418 IM_A_TEAPOT"),
            426 => f.write_str("426 UPGRADE_REQUIRED"),
            429 => f.write_str("429 TOO_MANY_REQUESTS"),
            500 => f.write_str("500 INTERNAL_SERVER_ERROR"),
            501 => f.write_str("501 NOT_IMPLEMENTED"),
            502 => f.write_str("502 BAD_GATEWAY"),
            503 => f.write_str("503 SERVICE_UNAVAILABLE"),
            504 => f.write_str("504 GATEWAY_TIMEOUT"),
            _ => Err(fmt::Error),
        }
    }
}
