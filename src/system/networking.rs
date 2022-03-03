#[derive(Debug,Serialize)]
pub struct Networking {
    pub hostname: String
}

impl Networking {
    pub fn new () -> Networking {
        let mut networking = Networking {
            hostname: String::new()
        };

        match hostname::get() {
            Ok(hostname) => {
                let hn_ref = hostname.to_string_lossy();
                networking.hostname = String::from(hn_ref)
            },
            Err(err) => eprintln!("Hostname Error: {0}", err),
        }

        networking
    }
}

impl Default for Networking {
    fn default() -> Self {
        Self { hostname: String::default() }
    }
}