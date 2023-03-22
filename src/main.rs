// MIT License
//
// Copyright (c) 2023 Sergio Arroutbi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
    #[clap(short, long, value_parser)]
    test: bool,
}

fn parse(body: String) -> u32 {
    let verbose = verbose::Verbose::new(Args::parse().verbose);
    match links::download_links(
        links::get_links(links::get_link_lines(body), verbose),
        links::DownloadData::new(Args::parse().directory, Args::parse().test),
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
        parse(std::str::from_utf8(data).unwrap().to_string());
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}

fn main() {
    go();
}
