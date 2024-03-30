use clap::Parser;
use rsfilc::{
    api::*, api_urls::ApiUrls, messages::MessageKind, school_list::School, timetable, AnyErr,
};
use speedate::DateTime;

#[tokio::main]
async fn main() -> AnyErr<()> {
    let args = rsfilc::args::Args::parse();

    let user = if let Some(loaded_user) = User::load() {
        loaded_user
    } else {
        User::create()
    };
    user.greet().await;
    // let user = User::new(username, password, school_id);

    let apiurls = ApiUrls::api_urls().await?;
    eprintln!("\ngot api urls...\n");
    // println!("{:#?}", apiurls);

    let schools = School::get_from_refilc().await?;
    eprintln!("\ngot schools...\n");
    // println!("{:#?}", schools);

    let access_token = user.token().await?;
    eprintln!("\ngot access_token...\n");
    // println!("{:?}", access_token);

    let info = user.info().await?;
    eprintln!("\ngot user info...\n");
    // println!("{}", info);

    let messages = user.messages(MessageKind::Beerkezett).await?;
    eprintln!("\ngot messages...\n");
    // println!("{}", messages);

    let evals = user.evals().await?;
    eprintln!("\ngot evals...\n");
    // println!("{}", evals);

    let date = "2024-03-27";
    let from = DateTime::parse_str(&format!("{}T00:00", date)).expect("invalid date-time");
    let to = DateTime::parse_str(&format!("{}T18:00", date)).expect("invalid date-time");
    // let from = DateTime::parse_str("2024-03-22T00:00").expect("invalid date-time");
    // let to = DateTime::parse_str("2024-03-22T23:59").expect("invalid date-time");
    // let from = DateTime::now(0).expect("invalid date-time");
    // let from = DateTime::now(0).expect("invalid date-time");
    let mut timetable = user.timetable(from, to).await?;
    eprintln!("\ngot timetable...\n");
    timetable.sort_by(|a, b| a.from().partial_cmp(&b.from()).expect("couldn't compare"));
    timetable::Lesson::print_day(timetable);
    // println!("{}", timetable);

    let announced = user.announced(None).await?;
    eprintln!("\ngot announced...\n");
    // println!("{}", announced);

    let absences = user.absencies().await?;
    eprintln!("\ngot absences...\n");
    // println!("{}", absences);

    Ok(())
}
