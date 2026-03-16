use packages_crates::authenticate;
use packages_crates::Credentials;


fn main() {

    let creds = Credentials {
        username: String::from("sgarg"),
        password: String::from("password"),
    };

    authenticate(creds);
}
