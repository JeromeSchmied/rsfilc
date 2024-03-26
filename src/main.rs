use refilc::{kreta_api::*, AnyErr};

#[tokio::main]
async fn main() -> AnyErr<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        println!("Usage: <username> <password> <school_id>");
        return Ok(());
    }

    let username = &args[1];
    let password = &args[2];
    let school_id = &args[3];
    let user = User::new(username, password, school_id);

    let apiurls = ApiUrls::get().await?;
    println!("\ngot api urls...");
    // println!("{:#?}", apiurls);

    let schools = School::get_from_refilc().await?;
    println!("\ngot schools...");
    // println!("{:#?}", schools);

    let access_token = user.get_token().await?;
    println!("\ngot access_token...");
    // println!("{:?}", access_token);

    let info = user.get_info().await?;
    println!("\ngot user info...");
    // println!("{:?}", info);

    let messages = user.get_messages(MessageKind::Beerkezett).await?;
    println!("\ngot messages...");
    // println!("{:?}", messages);

    let evals = user.get_evals().await?;
    println!("\ngot evals...");
    // println!("{:?}", evals);

    let timetable = user
        .get_timetable(Time::new(2024, 3, 18), Time::new(2024, 3, 24))
        .await?;
    println!("\ngot timetable...");
    // println!("{:?}", timetable);

    Ok(())
}
