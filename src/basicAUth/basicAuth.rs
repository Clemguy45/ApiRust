
struct BasicAuth{
     username: String,
     password: String
}

impl BasicAuth{
     fn from_auth_header(header : &str) -> Option<BasicAuth>{
         let split = header.split_ascii_whitespace().collect::<Vec<&str>>();
         if split.len() != 2 || split[0] != "Basic"{
             return None;
         }
          Self::from_base64(split[1])
     }

     fn from_base64(base64_string : &str) -> Option<BasicAuth>{
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
