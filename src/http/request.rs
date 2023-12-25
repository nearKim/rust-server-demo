use super::method::{Method, MethodError};
use std::str::Utf8Error;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}


impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        // match str::from_utf8(value).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }
        // ?는 Ok인 경우 변수에 값을 저장하고 Err인 경우 return 한다
        // from_utf8 내부에서 던지는 Utf8Error를 ParseError로 바꾸기 위해서는 From trait을 아래처럼 구현해야 한다.

        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest)
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // 아래 코드는 valid하지만 None인 경우에 아무것도 안하는데 코드를 놔두는게 맘에 들지 않는다
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // 아래 코드는 valid하지만 추가적인 variable을 생성해야 하는 불편함이 있다.
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut i = request.chars();
    // loop {
    //     let item = i.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // 일반으로 1을 단순히 더하는 것은 utf-8 때문에 위험하지만 빈 공백은 무조건 1 byte이기 때문에 이 경우는 안전하다
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        // Utf8Error가 던져지면 그 종류에 상관없이 무조건 InvalidEncoding으로 변환한다
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}