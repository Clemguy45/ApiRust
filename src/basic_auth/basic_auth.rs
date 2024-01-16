use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};

pub struct BasicAuth{
     username: String,
     password: String
}

impl BasicAuth{
    pub fn from_auth_header(header : &str) -> Option<BasicAuth>{
        let split = header.split_ascii_whitespace().collect::<Vec<&str>>();
        if split.len() != 2 || split[0] != "Basic"{
            return None;
        }
        Self::from_base64(split[1])
    }
     pub fn from_base64(base64_string : &str) -> Option<BasicAuth>{
         let decoded = base64::decode(base64_string).ok()?;
         let decoded_str = String::from_utf8(decoded).ok()?;
         let split = decoded_str.split(':').collect::<Vec<&str>>();

         if split.len() != 2{
             return None;
         }
         let (username, password) = (split[0].to_string(), split[1].to_string());

         Some(BasicAuth{
             username,
             password})
     }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header {
            if let Some(basic_auth) = Self::from_auth_header(auth_header) {
                return Outcome::Success(basic_auth);
            }
        }

        Outcome::Forward(Status::Unauthorized)
    }
}