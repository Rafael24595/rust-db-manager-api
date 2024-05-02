use regex::Regex;

use crate::commons::exception::auth_exception::AuthException;

use super::{cookie::Cookie, jar::Jar};

pub(crate) struct BuilderJar {
    last: usize,
    buffer: Vec<String>,
    jar_string: String,
    jar: Jar
}

impl BuilderJar {
    
    pub(crate) fn make(jar_string: String) -> Result<Jar, AuthException> {
        let mut instance = Self {
            last: 0,
            buffer: Vec::new(),
            jar_string: jar_string,
            jar: Jar::new()
        };
        instance._make()
    }

    fn _make(&mut self) -> Result<Jar, AuthException> {
        let pattern = Regex::new(r";\s?[\w|.|-]+=").unwrap();

        for capture in pattern.captures_iter(&self.jar_string.clone()) {
            if let Some(index) = capture.get(0).map(|m| m.start()) {
                self.manage_cookie(index)?;
                self.last = index + 1;
            }
        }

        self.manage_cookie(self.jar_string.len())?;
        
        if !self.buffer.is_empty() {
            self.flush_buffer()?;
        }

        Ok(self.jar.clone())
    }

    fn manage_cookie(&mut self, index: usize) -> Result<(), AuthException> {
        let jar_string = self.jar_string.clone();
        let fragment = jar_string[self.last..index].trim();
        println!("{}", self.buffer.join("; "));
        if self.last > 0 && Self::is_cookie_definition(fragment) {
            self.flush_buffer()?;
        }

        self.buffer.push(fragment.to_string());

        Ok(())
    }

    fn flush_buffer(&mut self) -> Result<(), AuthException> {
        self.jar.cookies.push(Cookie::from_string(&self.buffer.join("; "))?);
        self.buffer.clear();
        Ok(())
    }

    fn is_cookie_definition(fragment: &str) -> bool {
        let f = fragment.to_lowercase();

        !f.starts_with("domain") && !f.starts_with("path") 
        && !f.starts_with("expires") && !f.starts_with("max-age") 
        && !f.starts_with("samesite") && !f.starts_with("secure")
        && !f.starts_with("httponly")
    }

}