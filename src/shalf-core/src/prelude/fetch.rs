use std::collections::HashMap;

pub enum ReferrerPolicy {
    None = 0,
    NoReferrer = 1,
    NoReferrerWhenDowngrade = 2,
    Origin = 3,
    OriginWhenCrossOrigin = 4,
    UnsafeUrl = 5,
    SameOrigin = 6,
    StrictOrigin = 7,
    StrictOriginWhenCrossOrigin = 8,
}

pub enum RequestMode {
    SameOrigin = 0,
    NoCors = 1,
    Cors = 2,
    Navigate = 3,
}

pub enum RequestCredentials {
    Omit = 0,
    SameOrigin = 1,
    Include = 2,
}

pub enum RequestCache {
    Default = 0,
    NoStore = 1,
    Reload = 2,
    NoCache = 3,
    ForceCache = 4,
    OnlyIfCached = 5,
}

pub enum RequestRedirect {
    Follow = 0,
    Error = 1,
    Manual = 2,
}

pub enum Value {
    String(String),
    Binary(Vec<u8>),
    None
}

pub enum ValueType {
    String, Binary, None
}

/// Differs from Fetch API's RequestInit by presence URL and absence abort signal and observer
/// methods.
pub trait RequestInit: Clone {
    // NOTE: observer and abort signal not declared
    fn body_used(&self) -> bool;
    fn get_body(&self) -> Value;
    fn set_body(&mut self, value: &Value);
    fn get_cache(&self) -> Option<RequestCache>;
    fn set_cache(&mut self, cache: RequestCache);
    fn get_credentials(&self) -> RequestCredentials;
    fn set_credentials(&mut self, credentials: RequestCredentials);
    fn get_headers(&self) -> impl Headers;
    fn set_headers(&mut self, headers: &impl Headers);
    fn get_integrity(&self) -> Option<String>;
    fn set_integrity(&mut self, integrity: &str);
    fn get_method(&self) -> Option<String>;
    fn set_method(&mut self, method: &str);
    fn get_mode(&self) -> Option<RequestMode>;
    fn set_mode(&mut self, mode: RequestMode);
    fn get_redirect(&self) -> Option<RequestRedirect>;
    fn set_redirect(&mut self, val: RequestRedirect);
    fn get_referrer(&self) -> Option<String>;
    fn set_referrer(&mut self, val: &str);
    fn get_referrer_policy(&self) -> Option<ReferrerPolicy>;
    fn set_referrer_policy(&mut self, val: ReferrerPolicy);
    fn get_url(&self) -> Option<String>;
    fn set_url(&mut self, url: &str);
}

pub trait Request : Clone { 
    fn new(init: &impl RequestInit) -> Result<Self, ()>;
    fn get_info(&self) -> &impl RequestInit;
    // async fn form_data(&self) -> Result<HashMap<String, String>, ()>;
}

pub trait Headers : Clone {
    fn new() -> Self;
    fn append(&self, name: &str, value: &str) -> Result<(), ()>;
    fn delete(&self, name: &str) -> Result<(), ()>;
    fn entries(&self) -> impl std::iter::Iterator;
    fn get(&self, name: &str) -> Result<String, ()>;
    fn has(&self, name: &str) -> bool;
    fn keys(&self) -> impl std::iter::Iterator;
    fn set(&self, name: &str, value: &str) -> Result<(), ()>;
    fn values(&self) -> impl std::iter::Iterator;
}

pub enum ResponseType {
    Basic = 0,
    Cors = 1,
    Default = 2,
    Error = 3,
    Opaque = 4,
    Opaqueredirect = 5,
}

pub trait Response : Clone {
    fn url(&self) -> String;
    fn redirected(&self) -> bool;
    fn status(&self) -> u16;
    fn ok(&self) -> bool;
    fn status_text(&self) -> String;
    fn headers(&self) -> impl Headers;
    fn body_used(&self) -> bool;
    async fn form_data(&self) -> Result<HashMap<String, String>, ()>;
    async fn text(&self) -> Result<String, ()>;
    async fn blob(&self) -> Result<String, ()>;
}
