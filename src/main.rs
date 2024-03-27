use refilc::{api::*, api_urls::ApiUrls, messages::MessageKind, school_list::School, AnyErr};
use speedate::DateTime;

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

    let apiurls = ApiUrls::api_urls().await?;
    eprintln!("\ngot api urls...");
    // println!("{:#?}", apiurls);

    let schools = School::get_from_refilc().await?;
    eprintln!("\ngot schools...");
    // println!("{:#?}", schools);

    let access_token = user.token().await?;
    eprintln!("\ngot access_token...");
    // println!("{:?}", access_token);

    let info = user.info().await?;
    eprintln!("\ngot user info...");
    // println!("{}", info);

    let messages = user.messages(MessageKind::Beerkezett).await?;
    eprintln!("\ngot messages...");
    // println!("{}", messages);

    let evals = user.evals().await?;
    eprintln!("\ngot evals...");
    // println!("{}", evals);

    let from = DateTime::parse_str("2024-03-18T08:00").expect("invalid date-time");
    let to = DateTime::parse_str("2024-03-18T08:45").expect("invalid date-time");
    // let from = DateTime::now(0).expect("invalid date-time");
    // let from = DateTime::now(0).expect("invalid date-time");
    let timetable = user.timetable(from, to).await?;
    eprintln!("\ngot timetable...");
    println!("{}", timetable);

    let announced = user.announced(None).await?;
    eprintln!("\ngot announced...");
    // println!("{}", announced);

    let absences = user.absencies().await?;
    eprintln!("\ngot absences...");
    // println!("{}", absences);

    Ok(())
}
