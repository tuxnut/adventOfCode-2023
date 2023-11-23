use std::{fs, io};

use dotenv::dotenv;

fn create_directory(day: &str) {
    let path = format!("src/day{}", day);
    fs::create_dir(path).expect("Failed to create directory");
}

fn download_input(day: &str) {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);

    let session = dotenv::var("COOKIE_SESSION").expect("Failed to get session cookie");
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={session}"))
        .send()
        .expect("Failed to send request");
    let _ = fs::write(
        format!("src/day{}/input.txt", day),
        response.text().expect("Failed to parse response text"),
    );
}

fn copy_template(day: &str) {
    if std::path::Path::new(&format!("src/day{}/main.rs", day)).exists() {
        println!("Template already exists");
        return;
    }

    let mut template_content = fs::read_to_string("src/template.rs").expect("Template not found");
    template_content = template_content.replace(r#"{{DAY}}"#, &format!("day{}", day));
    fs::write(format!("src/day{}/main.rs", day), template_content)
        .expect("Failed to write template");
}

fn append_target_to_toml(day: &str) {
    let mut toml_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let target = format!("day{}", day);
    if toml_content.contains(&target) {
        println!("Target already exists");
        return;
    }

    toml_content = toml_content
        + &format!("\n[[bin]]")
        + &format!("\nname = \"day{day}\"")
        + &format!("\npath = \"src/day{day}/main.rs\"\n");

    fs::write("Cargo.toml", toml_content).expect("Failed to write Cargo.toml");
}

fn main() {
    dotenv().ok();
    println!("Input the day to prepare.");

    let mut day = String::new();

    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    day = day.trim().to_string();

    println!("Preparing environment for day {}s", day);

    create_directory(&day);
    download_input(&day);
    copy_template(&day);
    append_target_to_toml(&day);
}
