/*
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated with help by: https://github.com/swagger-api/swagger-codegen.git
 */

use base64::encode;
use cookie::{Cookie, CookieJar};
use futures;
use hyper;
use hyper::header::{HeaderName, HeaderValue};
use reqwest;

use super::{custom_query, Error};
use std::collections::HashMap;
use std::io::Read;

pub struct Configuration<C: hyper::client::connect::Connect> {
    pub base_path: String,
    pub client: hyper::Client<C>,
    cookie_jar: CookieJar,
    server: String,
    // If current time is > session_timeout we need to login again
    session_timeout: Option<::std::time::Instant>,
    basic_auth: bool,
    user: String,
    password: String,
    ssl_cert: Option<::std::path::PathBuf>,
}

impl<C: hyper::client::connect::Connect + 'static + std::marker::Sync + std::marker::Send + Clone> Configuration<C> {
    /// The Isilon docs say basic authorization is slow and resource intensive.  
    /// The default here is to use session tokens but you can fall back on basic
    /// authorization if need be by setting basic_authorization to true
    pub fn new(
        client: hyper::Client<C>,
        server: &str,
        basic_authorization: bool,
        user: &str,
        password: &str,
        ssl_cert: Option<&::std::path::Path>,
    ) -> Configuration<C> {
        Configuration {
            base_path: format!("https://{}:8080", server),
            client: client,
            cookie_jar: CookieJar::new(),
            server: server.to_string(),
            basic_auth: basic_authorization,
            user: user.to_string(),
            password: password.to_string(),
            ssl_cert: ssl_cert.and_then(|cert| Some(cert.to_path_buf())),
            session_timeout: None,
        }
    }

    // Returns true for timed out or false otherwise
    fn timed_out(&self) -> bool {
        // We never set the session so yes it's expired
        if self.session_timeout.is_none() {
            return false;
        }

        // Check if the current time exceeds the timeout
        let now = ::std::time::Instant::now();
        if now > self.session_timeout.unwrap() {
            return true;
        }
        return false;
    }

    // Get the session cookie and set it on the request
    pub(crate) fn set_session<B>(&self, req: &mut hyper::Request<B>) -> Result<(), Error> {
        // If we're going to use basic auth then set that
        if self.basic_auth {
            self.set_login_header(req);
        } else {
            // Else use the session cookies

            // Check expiration of session tokens and login if they're old
            if self.timed_out() {
                //TODO: I can't modify self without creating tons of problems
                return Err(Error::SessionExpired);
            }

            // Set the isisessid if available
            match self.cookie_jar.get("isisessid") {
                Some(t) => {
                    let isi_cookie = format!("{}={}", t.name(), t.value());
                    debug!("Setting cookie: {}", isi_cookie);
                    req.headers_mut().insert(
                        hyper::header::COOKIE,
                        HeaderValue::from_str(&isi_cookie).unwrap(),
                    );
                }
                None => return Err(Error::E("isisessid cookie missing.  cannot set".into())),
            };

            // Possible Cross Site Request Forgery cookie will be set
            match self.cookie_jar.get("isicsrf") {
                Some(t) => {
                    let csrf_cookie = format!("{}:{}", t.name(), t.value());
                    debug!("Setting csrf-token: {}", csrf_cookie);
                    req.headers_mut().insert(
                        HeaderName::from_static("X-CSRF-Token"),
                        HeaderValue::from_str(&t.value()).unwrap(),
                    );
                }
                None => {
                    // Older versions of the API don't support this so skip it
                }
            };
            req.headers_mut().insert(
                HeaderName::from_static("Origin"),
                HeaderValue::from_str(&self.server).unwrap(),
            );
            req.headers_mut().insert(
                hyper::header::REFERER,
                HeaderValue::from_static("https://localhost"),
            );
        }

        Ok(())
    }

    fn set_login_header<B>(&self, req: &mut hyper::Request<B>) {
        let auth = format!("{}:{}", self.user, self.password);
        let header_value = format!("basic {}", encode(&auth));
        req.headers_mut().insert(
            hyper::header::AUTHORIZATION,
            HeaderValue::from_str(&header_value).unwrap(),
        );
    }

    fn set_session_timeout(&mut self, val: &::serde_json::Value) -> Result<(), Error> {
        match val.get("timeout_absolute") {
            Some(timeout_json) => {
                if !timeout_json.is_number() {
                    // timeout_absolute is something other than a number
                    return Err(Error::E(format!(
                        "timeout_absolute response from server isn't a number: {:?}",
                        val
                    )));
                }
                let now = ::std::time::Instant::now();
                let timeout = ::std::time::Duration::from_secs(timeout_json.as_u64().unwrap());
                self.session_timeout = Some(now + timeout);
            }
            None => {
                // Response is missing the timeout value
                return Err(Error::E(
                    "Server response is missing timeout value".to_string(),
                ));
            }
        };

        Ok(())
    }

    /// Synchronous login function
    pub fn login(&mut self) -> Result<(), Error> {
        let client = match self.ssl_cert {
            Some(ref cert_path) => {
                let mut buf = Vec::new();
                ::std::fs::File::open(cert_path)?.read_to_end(&mut buf)?;
                let cert = reqwest::Certificate::from_der(&buf)?;

                reqwest::Client::builder()
                    .add_root_certificate(cert)
                    .danger_accept_invalid_certs(true)
                    .danger_accept_invalid_hostnames(true)
                    .timeout(::std::time::Duration::from_secs(300))
                    .build()?
            }
            None => reqwest::Client::builder()
                .timeout(::std::time::Duration::from_secs(300))
                .build()?,
        };

        // Parse the uri
        let body = json!({
            "username": self.user,
            "password": self.password,
            "services": ["platform", "namespace"],
        });

        let mut s = client
            .post(&format!("{}/session/1/session", self.base_path))
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?;

        let response: ::serde_json::Value = s.json()?;
        self.set_session_timeout(&response)?;

        // Step 2. Obtain the isisessid value from the Set-Cookie header.
        // From here we should get back a cookie
        let cookie_headers = s.headers().get_all(reqwest::header::SET_COOKIE);
        for cookie_header in cookie_headers.iter() {
            // Save the cookie
            let parsed = Cookie::parse(cookie_header.to_str().unwrap().to_string())?;
            self.cookie_jar.add(parsed);
        }
        Ok(())
    }

    pub fn logout(&self) -> Result<(), Error>{
        let mut headers: HashMap<String, String> = HashMap::new();
        let uri_str = format!("{}/session/1/session", self.base_path);

        // Set the isisessid if available
        match self.cookie_jar.get("isisessid") {
            Some(t) => {
                let isi_cookie = format!("{}={}", t.name(), t.value());
                headers.insert("Cookie".into(), isi_cookie.into());
            }
            None => {
                return 
                    Err(Error::E(
                        "Unable to find isisessid cookie from isilon server".to_string(),
                    ));
            }
        };
        custom_query(&self, &uri_str, &"", hyper::Method::DELETE, headers)
    }
}
