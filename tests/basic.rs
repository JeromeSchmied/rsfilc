use rsfilc::{self, user::User};

#[tokio::main]
async fn main() -> rsfilc::AnyErr<()> {
    let user = User::create();
    for eval in user.evals(None, None).await? {
        println!("{}", eval);
    }

    Ok(())
}
