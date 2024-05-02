use crate::commons::exception::api_exception::ApiException;

use super::{cookie::Cookie, same_site::SameSite};

pub(crate) struct BuilderCookie {
}

impl BuilderCookie {
    
    pub(crate) fn make(cookie_string: &str) -> Result<Cookie, ApiException> {
        let mut parts: Vec<&str> = cookie_string.split(';').collect();

        let code_value = parts.remove(0).trim()
            .split('=')
            .map(|f| String::from(f.trim()))
            .collect::<Vec<String>>();
        if code_value.len() != 2 {
            let message = String::from("Invalid cookie format");
            return Err(ApiException::new(301, message));
        }

        let code = code_value.get(0).cloned().unwrap();
        let value = code_value.get(1).cloned().unwrap();

        let mut cookie = Cookie::new(code, value);

        for part in parts {
            let key_value: Vec<String> = part.trim()
                .split('=')
                .map(|f| String::from(f.trim()))
                .collect::<Vec<String>>();
   
            let key = &key_value[0];
            let value = key_value.get(1).cloned();

            match key.to_lowercase().as_str() {
                "secure" => cookie.secure = Some(true),
                "httponly" => cookie.http_only = Some(true),
                "expires" => cookie.expiration = Some(value.unwrap_or_default()),
                "domain" => cookie.domain = Some(value.unwrap_or_default()),
                "path" => cookie.path = Some(value.unwrap_or_default()),
                "max-age" => cookie.max_age = {
                    let string_max_age = value.unwrap_or_default();
                    let max_age: Result<u32, _>  = string_max_age.parse();
                    if max_age.is_err() {
                        let exception = ApiException::new(301, max_age.unwrap_err().to_string());
                        return Err(exception);
                    }
                    Some(max_age.unwrap())
                },
                "samesite" => cookie.same_site = {
                    let string_samesite= value.unwrap_or_default();
                    let samesite = SameSite::from_string(&string_samesite.clone());
                    if samesite.is_none() {
                        let message = String::from(format!("Unknown Same Site value: '{}'", string_samesite));
                        return Err(ApiException::new(301, message));
                    }
                    Some(samesite.unwrap())
                },
                _ => {
                    let message = String::from(format!("Unknown field code: '{}'", key));
                    return Err(ApiException::new(301, message));
                }
            }
        }

        Ok(cookie)
    }

}