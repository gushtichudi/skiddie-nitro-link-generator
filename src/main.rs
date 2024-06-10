// use rand::{distributions::Alphanumeric, Rng};
use std::io;
use std::io::Write;
use std::time::Duration;
use std::thread;
use std::process::Command;
use rand::RngCore;
use base64::Engine as _;

fn validate() {
    let url = input("Url: ");
    Command::new("python3")
        .arg("validator/if_valid.py")
        .arg(url)
        .spawn()
        .expect("can't launch validator");
}

fn input(prompt: &str) -> String {
    // print prompt 
    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut output = String::new();

    io::stdin()
        .read_line(&mut output)
        .expect("error");

    output
}

fn parse_string(input_str: &String) -> Vec<&str> {
    let split_str: Vec<&str> = input_str
        .trim()
        .split_whitespace()
        .collect();

    split_str
}
/*
fn generate(times: u32) -> String {
    let mut nitro_url: String = "https://discord.gift/".to_owned();

    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let dat: String = s.to_owned();

    
    let final_link: String = format!("{nitro_url}{dat}");
    final_link
}
*/


fn generate() -> String {
    let mut random_bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut random_bytes); 
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);    
    format!("https://discord.gift/{}", encoded)
}

fn manage_command(tokens: Vec<&str>) {
    if tokens.len() != 1 {
        println!("erm");
    }

    // let z: &str = tokens[1];

    match tokens[0] {
        "generate" => {
            let y: u32 = tokens[1].parse().unwrap();
            if tokens.len() != 2 {
                println!("bro :(");
            }
            if tokens[1..].len() > 1 {
                println!("generate: only need one argument");
            }
            if tokens.len() == 1 {
                println!("generate: give one more argument");
            }
            match y {
                0 => {
                    println!("wtf?");
                },
                _ => {
                    if y > 15 {
                    println!("[!] \x1b[38;2;255;235;112mwarning\x1b[0m: number bigger than 15 inputted, delaying generations by 0.25 second");
                    }
                    for _n in 1..=y {
                        if y > 15 {
                            let link = generate();
                            // thread::sleep(Duration::from_millis(250));
                            println!("{}", link);
                        } else {
                            let link = generate();
                            println!("{}", link);
                        }
                    }
                },
            }
        },
        "validate" => {
            /*
            if tokens[1..].len() > 1 {
                println!("validate: only need one argument");
            } */

            validate();
        },
        _ => {
            println!("command doesnt exist");
        }
    }
}

fn main() {
    println!("│ \x1b[38;2;112;145;255mdiscord nitro generator and validator\x1b[0m");
    println!("│ \x1b[38;2;128;128;128mwritten by\x1b[0m  gushtichudi");
    println!("│ \x1b[38;2;112;145;255m\x1b[0m : lvinkjhor.  : journalctl__ 󰊫 : chudaomaa00@gmail.com");
    println!("\ntype `help` for help.\n");

    loop {
        let field = input(" ");
        let eh = parse_string(&field);

        // println!(" \x1b[38;2;0;183;255;112m{:?}\x1b[0m", eh);
        manage_command(eh);
    }
}
