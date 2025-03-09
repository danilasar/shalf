trait Request {
    
}

trait Headers {
    fn new() -> Self;
    fn append(&self, name: &str, value: &str) -> Result<(), ()>;
}

enum ResponseType {
    Basic = 0,
    Cors = 1,
    Default = 2,
    Error = 3,
    Opaque = 4,
    Opaqueredirect = 5,
}

trait Response {
    /// Getter for the `url` field of this object.
    fn url(&self) -> String;

    /// Getter for the `redirected`` field of this object.
    fn redirected(&self) -> bool;

    fn status(&self) -> u16;

    fn ok(&self) -> bool;

    fn status_text(&self) -> String;

    
}
