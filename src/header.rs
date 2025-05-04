use serde::{Deserialize, Serialize};
use base64_url::encode;

use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header<'b> {
    #[serde(borrow)]
    alg: Algorithm<'b>,
    typ: TokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Algorithm<'b>(&'b str);

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "JWT")]
    Jwt
}

impl<'b> Header<'b> {
    pub fn header(&self) -> Result<String, Error> {
        Ok(encode(&serde_json::to_string(self).map_err(|error| Error::DeserializationError(error))?))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use serde_json;

    #[test]
    fn can_parse_header() -> () {
        let header: Header = serde_json::from_str(r#"{"typ":"JWT","alg":"MS123"}"#).unwrap();
        println!("header => {:?}", header.header());
        ()
    }
}