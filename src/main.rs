use clap::Parser;
use curl::easy::Easy;
use std::fs::File;
use std::io::prelude::*;

const LINK_HTML: &str = "<a href";
const LINK_HTML_EQUAL: &str = "<a href=";
const FEDORA_PROJECT: &str = "fedoraproject.org";
const DOWNLOAD_REDHAT: &str = "download.eng.bos.redhat.com";
const DOWNLOAD_KOJIHUB: &str = "kojihub.stream.rdu2.redhat.com";
const RPM_EXTENSION: &str = ".rpm";
static mut VERBOSE: bool = false;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    url: String,
    #[clap(short, long, value_parser)]
    verbose: bool,
}

fn get_link_lines(body: String) -> Vec<String> {
    let split = body.split('\n');
    let mut lines = Vec::new();
    for s in split {
	dump_verbose(&("S LINK LINE:".to_owned() + &s));
	if s.contains(LINK_HTML) && s.contains (RPM_EXTENSION) && ( s.contains(FEDORA_PROJECT) ||
								    s.contains(DOWNLOAD_REDHAT) ||
								    s.contains(DOWNLOAD_KOJIHUB) ) {
	    lines.push(s.to_string());
	}
    }
    lines
}

fn dump_verbose(s: &String) {
    unsafe {
	if VERBOSE {
	    println!("{}", s);
	}
    }
}

fn get_links(link_lines: Vec<String>) -> Vec<String> {
    let mut links = Vec::new();
    for s in link_lines {
	dump_verbose(&("LINK LINE:".to_owned() + &s));
	let fields: Vec<&str> = s.split(LINK_HTML_EQUAL).collect();
	if fields.len() > 2 {
	    let fields2: Vec<&str> = fields[2].split('>').collect();
	    links.push(fields2[0].to_string().replace(&['\"'][..], ""));
	}
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
    fields[fields.len()-1].to_string().replace(&['\"'][..], "")
}

fn download_links(links: Vec<String>) -> Result<u32, &'static str> {
    let mut downloaded = 0;
    for l in &links {
	let lname = get_link_name(l);
	println!("Downloading file:{} name:{}", l, lname);
	download_file(l, lname).expect("Error on file download");
	downloaded += 1;
    }
    if !links.is_empty() && 0 == downloaded {
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

fn go() {
    let mut easy = Easy::new();
    easy.url(&Args::parse().url).unwrap();
    unsafe { VERBOSE = Args::parse().verbose; }
    easy.write_function(|data| {
	parse(std::str::from_utf8(data).unwrap().to_string());
	Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn main() {
    go();
}
