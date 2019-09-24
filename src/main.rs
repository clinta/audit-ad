use clap::{App, Arg};
use csv;

use std::collections::HashMap;
use std::error::Error;

fn main() {
    let matches = App::new("NTLM Password Checker")
        .version("0.1")
        .author("Clint Armstrong <clint@clintarmstrong.net>")
        .about("Checks an ntml password dump against HIBP hashes")
        .arg(
            Arg::with_name("dump")
                .short("d")
                .long("dump")
                .value_name("FILE")
                .help("ad password dump")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hashes")
                .short("h")
                .long("hashes")
                .value_name("FILE")
                .help("hibp hashes")
                .takes_value(true),
        )
        .get_matches();
    let dump = matches.value_of("dump").unwrap_or("dump.txt");
    println!("Dump file: {}", dump);
    let hashes = matches
        .value_of("hashes")
        .unwrap_or("pwned-passwords-ntlm-ordered-by-count-v5.txt");
    println!("hashes file: {}", hashes);

    run(dump, hashes).unwrap();
}

fn hash_dump(dump: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let mut ret = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_path(dump)?;
    for result in rdr.records() {
        let record = result?;
        let user = record.get(0).ok_or("no user from dump")?;
        let hash = record.get(3).ok_or("no hash from dump")?;
        ret.entry(hash.to_owned())
            .or_insert(Vec::new())
            .push(user.to_owned());
    }
    Ok(ret)
}

fn run(dump: &str, hashes: &str) -> Result<(), Box<dyn Error>> {
    let dump = hash_dump(dump)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_path(hashes)?;
    for result in rdr.records() {
        let record = result?;
        let hash = record.get(0).ok_or("no hash from hashes")?;
        if let Some(users) = dump.get(hash) {
            let count = record.get(1).ok_or("no count from hash")?;
            for user in users {
                println!("{}:{}:{}", user, hash, count);
            }
        }
    }

    Ok(())
}
