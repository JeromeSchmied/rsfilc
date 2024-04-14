use rsfilc::{self, user::User};

#[tokio::test]
async fn new_user() {
    // let user = User::create();
    let user = User::new("", "", "");
}
