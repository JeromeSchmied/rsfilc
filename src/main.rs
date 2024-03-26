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
    eprintln!("\ngot api urls...");
    // println!("{:#?}", apiurls);

    let schools = School::get_from_refilc().await?;
    eprintln!("\ngot schools...");
    // println!("{:#?}", schools);

    let access_token = user.get_token().await?;
    eprintln!("\ngot access_token...");
    // println!("{}", access_token);

    let info = user.get_info().await?;
    eprintln!("\ngot user info...");
    // println!("{}", info);

    let messages = user.get_messages(MessageKind::Beerkezett).await?;
    eprintln!("\ngot messages...");
    // println!("{}", messages);

    let evals = user.get_evals().await?;
    eprintln!("\ngot evals...");
    // println!("{}", evals);

    let timetable = user
        .get_timetable(Time::new(2024, 3, 18), Time::new(2024, 3, 24))
        .await?;
    eprintln!("\ngot timetable...");
    // println!("{}", timetable);

    let announced = user.get_announced(None).await?;
    eprintln!("\ngot announced...");
    // println!("{}", announced);

    let absences = user.get_absencies().await?;
    eprintln!("\ngot absences...");
    // println!("{}", absences);

    Ok(())
}
