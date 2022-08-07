use std::fmt;

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);

/// HTTP version
///
/// HTTP/1.0 and HTTP/1.1 are the only supported versions currently.
impl Version {
    /// `HTTP/1.0`
    pub const HTTP_10: Version = Version(Http::Http10);

    /// `HTTP/1.1`
    pub const HTTP_11: Version = Version(Http::Http11);
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Http {
    Http10,
    Http11,
}

impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::HTTP_11
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Http::*;

        f.write_str(match self.0 {
            Http10 => "HTTP/1.0",
            Http11 => "HTTP/1.1",
        })
    }
}

/// InvalidHttpVersion
#[derive(Debug)]
pub struct InvalidHttpVersion;

/// Used for parsing HTTP Request
impl TryInto<Version> for u8 {
    type Error = InvalidHttpVersion;

    fn try_into(self) -> Result<Version, Self::Error> {
        match self {
            0 => Ok(Version::HTTP_10),
            1 => Ok(Version::HTTP_10),
            _ => Err(InvalidHttpVersion),
        }
    }
}
