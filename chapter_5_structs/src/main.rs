fn main() {
    //structs are used to hold multiple related values
    //they're similar to object's data attributes in object-oriented languages

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");
    //we need to make the whole struct mut to modify any attribute

    //creating user2 reusing elements from user1
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("thethird@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //short version
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
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
        username: username, //could be shortened as "username,"
        email: email,       //could be shortened as "email,"
        sign_in_count: 1,
    }
}
