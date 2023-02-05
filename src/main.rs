#[derive(Debug)]
struct Github {
    owner: String,
    repo: String,
}

#[derive(Debug)]
struct Gitlab {
    user: String,
    repo: String,
}

fn main() {
    let github = Github {
        owner: "rust-lang".to_string(),
        repo: "rust".to_string(),
    };
    let gitlab = Gitlab {
        user: "rust-lang".to_string(),
        repo: "rust".to_string(),
    };

    build(github);
    build(gitlab);
}

trait Repo {
    fn get_clone_url(&self) -> String;
}

impl Repo for Github {
    fn get_clone_url(&self) -> String {
        format!("https://github.com/{}/{}.git", self.owner, self.repo)
    }
}
impl Repo for Gitlab {
    fn get_clone_url(&self) -> String {
        format!("https://gitlab.com/{}/{}.git", self.user, self.repo)
    }
}

fn build<T: Repo>(repo: T) {
    let url = repo.get_clone_url();
    println!("Cloning {}", url);
}
