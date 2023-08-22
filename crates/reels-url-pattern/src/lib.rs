use std::{error, fmt};

use serde::{Deserialize, Serialize};
use unicode_xid::UnicodeXID;
use url::Url;
use urlencoding::encode;

#[derive(Debug)]
pub enum InvalidUrlPattern {
    MissingForwardSlash,
    InvalidIdentifier,
    MissingClosingAngleBracket,
    InvalidKleenePosition,
}

impl fmt::Display for InvalidUrlPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidUrlPattern::MissingForwardSlash => {
                f.write_str("Url pattern is missing leading forward slash")
            }
            InvalidUrlPattern::InvalidIdentifier => {
                f.write_str("Url capture is not a valid identifier")
            }
            InvalidUrlPattern::MissingClosingAngleBracket => {
                f.write_str("Url pattern is missing closing `>` for capture")
            }
            InvalidUrlPattern::InvalidKleenePosition => {
                f.write_str("Wildcard kleene capture can only be the last segment pattern")
            }
        }
    }
}

impl error::Error for InvalidUrlPattern {}

pub type PathCapture<'a> = Vec<SegmentPatternValue<'a>>;

/// Url Pattern for routing
///
/// - fixed path
///   "/a/b/c"
/// - dynamic path with capture
///   "/a/\<name\>/c"
/// - capture rest of the path
///   "/a/b/\<rest..\>"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlPattern {
    pattern: Vec<SegmentPattern>,
}

impl UrlPattern {
    pub fn parse(s: &str) -> Result<Self, InvalidUrlPattern> {
        let mut segments = s.split('/');
        (segments.next() == Some(""))
            .then_some(0)
            .ok_or(InvalidUrlPattern::MissingForwardSlash)?;

        let mut patterns = Vec::new();
        while let Some(segment) = segments.next() {
            let seg = SegmentPattern::parse(segment)?;
            if matches!(seg, SegmentPattern::WildcardKleene(_)) && segments.next().is_some() {
                // make sure at most one WildcardKleene exist and it is the last pattern
                return Err(InvalidUrlPattern::InvalidKleenePosition);
            } else {
                patterns.push(seg);
            }
        }

        Ok(UrlPattern { pattern: patterns })
    }

    pub fn match_url<'a>(&self, url: &'a Url) -> Option<Vec<SegmentPatternValue<'a>>> {
        let segments = url.path_segments()?;
        self.match_iter(segments)
    }

    pub fn match_str<'a>(&self, path: &'a str) -> Option<Vec<SegmentPatternValue<'a>>> {
        let segments = path.split('/').skip(1);
        self.match_iter(segments)
    }

    fn match_iter<'a, I>(&self, mut segments: I) -> Option<Vec<SegmentPatternValue<'a>>>
    where
        I: Iterator<Item = &'a str>,
    {
        let patterns = self.pattern.iter();
        let mut matched_values = Vec::new();
        for pattern in patterns {
            match pattern {
                SegmentPattern::Fixed(pat) => {
                    let segment = segments.next()?;
                    if segment != pat {
                        // eprintln!("Pattern does not match {:?} != {:?}", segment, pattern);
                        return None;
                    }
                }
                SegmentPattern::Wildcard(_) => {
                    let segment = segments.next()?;
                    matched_values.push(SegmentPatternValue::Wildcard(segment));
                }
                SegmentPattern::WildcardKleene(_) => {
                    let segments = segments.collect();
                    matched_values.push(SegmentPatternValue::WildcardKleene(segments));
                    return Some(matched_values);
                }
            }
        }
        if segments.next().is_some() {
            // have additional segments not matched in pattern
            // eprintln!("more segments than pattern");
            None
        } else {
            Some(matched_values)
        }
    }
}

impl TryFrom<&str> for UrlPattern {
    type Error = InvalidUrlPattern;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UrlPattern::parse(value)
    }
}

impl TryFrom<String> for UrlPattern {
    type Error = InvalidUrlPattern;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        UrlPattern::parse(&value)
    }
}

impl fmt::Display for UrlPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.pattern {
            write!(f, "/{}", segment)?
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum SegmentPatternValue<'a> {
    Wildcard(&'a str),
    WildcardKleene(Vec<&'a str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentPattern {
    Fixed(String),
    Wildcard(Ident),
    WildcardKleene(Ident),
}

impl SegmentPattern {
    fn parse(segment: &str) -> Result<Self, InvalidUrlPattern> {
        // fixed: start with not "<"
        // Wildcard: start with "<", valid ident in the middle, closing with ">"
        // WildcardMulti: start with "<", valid ident in the middle, closing with "..>"
        if segment.starts_with('<') {
            let n = segment.len();
            if segment.ends_with("..>") {
                let ident = Ident::parse(&segment[1..n - 3])
                    .map_err(|_| InvalidUrlPattern::InvalidIdentifier)?;
                Ok(SegmentPattern::WildcardKleene(ident))
            } else if segment.ends_with('>') {
                let ident = Ident::parse(&segment[1..n - 1])
                    .map_err(|_| InvalidUrlPattern::InvalidIdentifier)?;
                Ok(SegmentPattern::Wildcard(ident))
            } else {
                Err(InvalidUrlPattern::MissingClosingAngleBracket)
            }
        } else {
            let s = encode(segment).to_string();
            Ok(SegmentPattern::Fixed(s))
        }
    }
}

impl fmt::Display for SegmentPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentPattern::Fixed(s) => f.write_str(s),
            SegmentPattern::Wildcard(s) => write!(f, "<{}>", s),
            SegmentPattern::WildcardKleene(s) => write!(f, "<{}..>", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ident(String);

impl Ident {
    fn parse(s: &str) -> Result<Self, ()> {
        let mut chars = s.chars();
        let start = chars.next().ok_or(())?;
        if start != '_' && !UnicodeXID::is_xid_start(start) {
            return Err(());
        }
        if chars.all(UnicodeXID::is_xid_continue) {
            Ok(Ident(s.to_owned()))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_and_to_string() {
        let pat: UrlPattern = "/a/<b>/<c..>".try_into().unwrap();
        assert_eq!(pat.to_string(), "/a/<b>/<c..>");
    }

    #[test]
    fn parse_root() {
        let pat: UrlPattern = "/".try_into().unwrap();
        assert!(pat.match_str("/").is_some());
        assert!(pat.match_str("/a").is_none());
        assert!(pat.match_str("//").is_none());
    }

    #[test]
    fn fixed_patterns() {
        let pat: UrlPattern = "/a/b/c".try_into().unwrap();
        assert!(pat.match_str("/a/b/c").is_some());
        assert!(pat.match_str("/a/b/d").is_none());
        assert!(pat.match_str("/b/b/c").is_none());
        assert!(pat.match_str("/a/b").is_none());
        assert!(pat.match_str("/").is_none());
    }

    #[test]
    fn wildcard_patterns() {
        let pat: UrlPattern = "/a/b/<capture>".try_into().unwrap();
        assert!(pat.match_str("/a/b").is_none());
        assert!(pat.match_str("/a/b/c/d").is_none());
        let values = pat.match_str("/a/b/c").unwrap();
        if let [SegmentPatternValue::Wildcard(capture)] = &values[..] {
            assert_eq!(*capture, "c");
        } else {
            panic!("Invalid value");
        }

        let pat: UrlPattern = "/a/<capture>/c".try_into().unwrap();
        assert!(pat.match_str("/a/b").is_none());
        let values = pat.match_str("/a/b/c").unwrap();
        if let [SegmentPatternValue::Wildcard(capture)] = &values[..] {
            assert_eq!(*capture, "b");
        } else {
            panic!("Invalid value");
        }

        let pat: UrlPattern = "/<capture>/b/c".try_into().unwrap();
        assert!(pat.match_str("//b/c").is_some());
        let values = pat.match_str("/a/b/c").unwrap();
        if let [SegmentPatternValue::Wildcard(capture)] = &values[..] {
            assert_eq!(*capture, "a");
        } else {
            panic!("Invalid value");
        }

        let pat: UrlPattern = "/<cap1>/<cap2>/<cap3>".try_into().unwrap();
        let values = pat.match_str("/a/b/c").unwrap();
        if let [SegmentPatternValue::Wildcard(cap1), SegmentPatternValue::Wildcard(cap2), SegmentPatternValue::Wildcard(cap3)] =
            &values[..]
        {
            assert_eq!(*cap1, "a");
            assert_eq!(*cap2, "b");
            assert_eq!(*cap3, "c");
        } else {
            panic!("Invalid value");
        }
    }

    #[test]
    fn kleene_patterns() {
        let pat: UrlPattern = "/<capture..>".try_into().unwrap();
        let values = pat.match_str("/a/b/c").unwrap();
        if let [SegmentPatternValue::WildcardKleene(capture)] = &values[..] {
            assert_eq!(
                *capture,
                vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
            );
        } else {
            panic!("Invalid value");
        }

        let values = pat.match_str("/").unwrap();
        if let [SegmentPatternValue::WildcardKleene(capture)] = &values[..] {
            assert_eq!(capture.len(), 1);
            assert_eq!(capture[0], "");
        } else {
            panic!("Invalid value");
        }
    }

    #[test]
    fn mixed_patterns() {
        let pat: UrlPattern = "/<capture..>".try_into().unwrap();
        let values = pat.match_str("/a/b/c").unwrap();
        if let [SegmentPatternValue::WildcardKleene(capture)] = &values[..] {
            assert_eq!(
                *capture,
                vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
            );
        } else {
            panic!("Invalid value");
        }

        let values = pat.match_str("/").unwrap();
        if let [SegmentPatternValue::WildcardKleene(capture)] = &values[..] {
            assert_eq!(capture.len(), 1);
            assert_eq!(capture[0], "");
        } else {
            panic!("Invalid value");
        }
    }
}
