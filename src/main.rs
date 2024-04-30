use dotenv::dotenv;
use reqwest;
use std::env;
use tokio;

#[tokio::main()]
async fn main() {
    dotenv().ok();

    let token: String = std::env::var("TOKEN").expect("Failed to get token.");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let query = &args[1];
        match query.as_str() {
            "enable" | "e" => enable(&token).await,
            "disable" | "d" => disable(args[2].parse::<u32>().unwrap(), &token).await,
            "help" | "h" | _ => help(),
        }
    } else {
        summary_raw(&token).await;
    }
}

async fn disable(duration: u32, token: &str) {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "http://pi.hole/admin/api.php?disable={}&auth={}",
            duration, token
        ))
        .send()
        .await;

    match res {
        Ok(_) => {
            println!("Successfully disabled for {} seconds", duration);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

async fn enable(token: &str) {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "http://pi.hole/admin/api.php?enable&auth={}",
            token
        ))
        .send()
        .await;

    match res {
        Ok(_) => {
            println!("Successfully enabled blocker");
        }
        Err(e) => {
            eprint!("Error: {}", e);
        }
    }
}

async fn summary_raw(token: &str) {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "http://pi.hole/admin/api.php?summaryRaw&auth={}",
            token
        ))
        .send()
        .await;

    match res {
        Ok(response) => {
            println!("{:?}", response);
        }
        Err(e) => {
            eprint!("Error: {}", e);
        }
    }
}

const HELP: &str = "Usage: pi [COMMAND] [ARGUMENTS]
Available commands:
    - e, enable
        Enable ad-block
    - d, disable
        Disable ad-block for provided seconds
        Example: pihole d 60
";

fn help() {
    println!("{}", HELP);
}
