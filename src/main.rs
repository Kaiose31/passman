use clap::Parser;
mod osenv;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Command {
    name: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

fn get_details(value: String) -> HashMap<String, String> {
    let mut details: HashMap<String, String> = HashMap::new();
    let data: Vec<&str> = value.split(" ").collect();
    details.insert("username".to_string(), data[0].to_string());
    details.insert("password".to_string(), data[1].to_string());

    details
}

fn main() {
    let args = Command::parse();
    if let None = args.name {
        println!("Account Details\n****************\n");
        for (key, data) in osenv::get_all() {
            let detail = get_details(data);
            println!(
                "Name: {}\nUsername: {}\nPassword: {}\n****************",
                key, detail["username"], detail["password"]
            );
        }
    }

    if let Some(ref key) = args.name {
        if let None = args.username {
            if let None = args.password {
                match osenv::get_one(&key) {
                    Ok(val) => {
                        let detail = get_details(val);
                        println!("Account Details\n****************\nName: {}\nUsername: {}\nPassword: {}\n****************",
                        key,detail["username"],detail["password"])
                    }
                    Err(_) => println!("Details for {} not found", key),
                }
            }
        }
    }

    if let Some(account) = args.name {
        if let Some(username) = args.username {
            if username == "delete" {
                osenv::delete_account(&account);
            } else {
                if let Some(password) = args.password {
                    osenv::insert_account(&account, &username, &password);
                    println!("Successfully Inserted Details for {}", account);
                }
            }
        }
    }
}
