use clap::Parser;
use curl::easy::Easy;
use std::fs::File;
use std::io::prelude::*;

const LINK_HTML: &str = "<a href";
const FEDORA_PROJECT: &str = "fedoraproject.org";
const RPM_EXTENSION: &str = ".rpm";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    url: String,
}

fn get_link_lines(body: String) -> Vec<String> {
    let split = body.split('\n');
    let mut lines = Vec::new();
    for s in split {
	if s.contains(LINK_HTML) && s.contains(FEDORA_PROJECT) && s.contains (RPM_EXTENSION) {
	    lines.push(s.to_string());
	}
    }
    lines
}

fn get_links(link_lines: Vec<String>) -> Vec<String> {
    let mut links = Vec::new();
    for s in link_lines {
	let fields: Vec<&str> = s.split('"').collect();
	links.push(fields[1].to_string());
    }
    links
}

pub fn download_file(url: &str, path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut curl = Easy::new();
    curl.url(url)?;
    File::create(&path)?;
    let mut file = File::options().write(true).append(true).open(&path)?;
    curl.write_function(move |data| {
	if let Err(e) = file.write_all(data) {
	    println!("{}", &e);
	    panic!();
	}
	Ok(data.len())
    })?;
    curl.perform()?;
    Ok(())
}

fn get_link_name(link: &str) -> String {
    let fields: Vec<&str> = link.split('/').collect();
    fields[fields.len()-1].to_string()
}

fn download_links(links: Vec<String>) -> Result<u32, &'static str> {
    let mut downloaded = 0;
    for l in &links {
	let lname = get_link_name(l);
	println!("Downloading file:{} name:{}", l, lname);
	download_file(l, lname).expect("Error on file download");
	downloaded += 1;
    }
    if links.is_empty() && 0 == downloaded {
	return Err("Unable to download any link");
    }
    Ok(downloaded)
}

fn parse(body: String) -> u32 {
    match download_links(get_links(get_link_lines(body))) {
	Ok(d) => d,
	Err(e) => {
	    panic!("{}", &e);
	},
    }
}

fn go(url: String) {
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.write_function(|data| {
	parse(std::str::from_utf8(data).unwrap().to_string());
	Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn main() {
    go(Args::parse().url);
}
