fn main() {
    //structs are used to hold multiple related values
    //they're similar to object's data attributes in object-oriented languages

    let mut user1 = User {
        active : true,
        username : String::from("someusername123"),
        email : String::from("someone@example.com"),
        sign_in_count : 1,
    };

    user1.email = String::from("another@example.com");
    //we need to make the whole struct mut to modify any attribute

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    //A function to build a user struct given a mail and a username
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
