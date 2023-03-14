use clap::Parser;
use curl::easy::Easy;

mod links;
mod verbose;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    url: String,
    #[clap(short, long, value_parser)]
    verbose: bool,
    #[clap(short, long, value_parser)]
    directory: Option<String>,
}

fn parse(body: String, dpath: Option<String>, v: bool) -> u32 {
    let verbose = verbose::Verbose::new(v);
    match links::download_links(
        links::get_links(links::get_link_lines(body), verbose),
        dpath,
    ) {
        Ok(d) => d,
        Err(e) => {
            panic!("{}", &e);
        }
    }
}

fn go() {
    let mut easy = Easy::new();
    easy.url(&Args::parse().url).unwrap();
    easy.write_function(|data| {
        parse(
            std::str::from_utf8(data).unwrap().to_string(),
            Args::parse().directory,
            Args::parse().verbose,
        );
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}

fn main() {
    go();
}
