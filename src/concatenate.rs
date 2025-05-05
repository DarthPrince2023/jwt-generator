use crate::{errors::Error, header::Header};
use crate::payload::Payload;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex::ToHex;
use dotenv::dotenv;

type HMacSha256 = Hmac<Sha256>; 

pub fn build_jwt<'b>(header: Header<'b>, payload: Payload<'b>) -> Result<String, Error> {
    match dotenv().ok() {
        _ => ()
    };
    let master_key = match std::env::var("MASTER_KEY") {
        Ok(key) => key,
        Err(error) => format!("Error Occurred => {error:?}")
    };
    println!("MASTER KEY => {master_key}");
    let mut mac = HMacSha256::new_from_slice(master_key.as_bytes())
        .map_err(|error| Error::InvalidDigestLength(error))?;
    let data = format!("{:?}.{:?}.", header.header(), payload.payload());
    let _ = mac.update(&data.as_bytes());
    let mut result = mac.finalize().into_bytes();
    let jwt: String = ToHex::encode_hex(&mut result);
    
    Ok(format!("{}.{}.{}", header.header()?, payload.payload(), base64_url::encode(&jwt)))
}

#[cfg(test)]
pub mod test {
    use super::*;
    use serde_json;

    #[test]
    fn can_get_jwt() {
        let payload = serde_json::from_str::<Payload>(r#"{
            "sub": "1234567890",
            "name": "John Doe",
            "admin": true
          }"#).unwrap();
        let header: Header = serde_json::from_str(r#"{"typ":"JWT","alg":"HS256"}"#).unwrap();
        println!("header => {:?}", &build_jwt(header, payload).unwrap());
    }
}