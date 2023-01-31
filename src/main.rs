#[derive(Debug)]
struct URL {
    protocol: String,
    hostname: String,
    pathname: String,
}

impl URL {
    fn toString(&self) -> String {
        format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
    }

    fn from(url: &str) -> Self {
        let string = String::from(url);
        let vec: Vec<&str> = string.split("://").collect();
        let protocol = String::from(vec[0]);
        let rest = String::from(vec[1]);
        let vec2: Vec<&str> = rest.split("/").collect();
        let hostname = String::from(vec2[0]);
        let pathname: String = String::from(vec[1]);

        URL {
            protocol,
            hostname,
            pathname,
        }
    }
}

fn main() {
    let app = URL {
        protocol: String::from("https"),
        hostname: String::from("app.rust-for-js.dev"),
        pathname: String::from("posts/07-structs-and-methods/"),
    };

    let strc = URL::from("https://cloudflare.com/people");

    println!("The strc {:?}", strc);

    println!("{}", app.toString())
}
