use serde::{Deserialize, Serialize};
use base64_url::encode;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub enum ReservedClaims<'b> {
    #[serde(rename = "exp")]
    ExpirationTime,
    #[serde(rename = "iss")]
    Issuer,
    #[serde(rename = "sub")]
    Subject,
    #[serde(rename = "aud")]
    Audience,
    Other(&'b str)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PayloadBytes<'b>(&'b str);

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload<'b> {
    pub sub: PayloadBytes<'b>,
    pub name: &'b str,
    pub admin: bool
}

impl<'b> Payload<'b> {
    pub fn payload(&self) -> String {
        encode(&serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn can_parse_payload() {
        let payload = serde_json::from_str::<Payload>(r#"{
            "sub": "1234567890",
            "name": "John Doe",
            "admin": true
          }"#).unwrap();
        
        println!("{:?}", payload.payload());
    }
}