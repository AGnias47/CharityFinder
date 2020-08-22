extern crate clap;
extern crate yaml_rust;
extern crate reqwest;

use std::io::Read;
use text_io::read;

fn main() {
    println!("Which issue matters the most to you?");
    println!("1. Criminal Justice Reform");
    println!("2. Climate Change");
    println!("3. Healthcare");
    let choice: i32 = read!();
    let params;
    if choice == 1 {
        params = format!("&search={}", "criminal%20justice");
    } else if choice == 2 { 
        params = format!("&search={}", "climate%20change");
    } else if choice == 3 {
        params = format!("&search={}", "healthcare");
    } else {
        std::process::exit(0x0100);
    }

    let base_url: &str = "https://api.data.charitynavigator.org/v2";
    let search_url: String = format!("{}/Organizations", base_url);
    let auth_yaml_fname: &str = "data/keys.yaml";
    let mut auth_yaml_file = std::fs::File::open(auth_yaml_fname).expect(&format!("Can't open {}", auth_yaml_fname));
    let mut yaml_content = String::new();
    auth_yaml_file.read_to_string(&mut yaml_content).expect(&format!("Can't read {}", auth_yaml_fname));
    let auth_yamls = yaml_rust::YamlLoader::load_from_str(&yaml_content).unwrap();
    let auth_yaml = &auth_yamls[0];
    let app_id: &str = auth_yaml["AppID"].as_str().unwrap();
    let app_key: &str = auth_yaml["APIKey"].as_str().unwrap();
    let auth_search_url = format!("{}?app_id={}&app_key={}", search_url, app_id, app_key);
    let url = format!("{}{}", auth_search_url, params);

    // ?: Unpack Result if okay, else return error
    println!("{}", url);
    let mut response = reqwest::blocking::get(&url).expect("Failed");
    let mut body = String::new();
    response.read_to_string(&mut body).expect("Failed");

    println!("Status: {}", response.status());
    println!("Body: {}", body);
}
