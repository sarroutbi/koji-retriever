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

use curl::easy::Easy;
use std::fs::File;
use std::io::Write;

use super::verbose;

const DOWNLOAD_REDHAT: &str = "download.eng.bos.redhat.com";
const DOWNLOAD_KOJIHUB: &str = "kojihub.stream.rdu2.redhat.com";
const FEDORA_PROJECT: &str = "fedoraproject.org";
const LINK_HTML: &str = "<a href";
const LINK_HTML_EQUAL: &str = "<a href=";
const RPM_EXTENSION: &str = ".rpm";

pub struct DownloadData {
    directory: Option<String>,
    test: bool,
}

impl DownloadData {
    pub fn directory(&self) -> Option<String> {
        self.directory.as_ref().map(|x| x.to_string())
    }
    pub fn test(&self) -> bool {
        self.test
    }
    pub fn new(directory: Option<String>, test: bool) -> DownloadData {
        DownloadData { directory, test }
    }
}

pub fn get_link_lines(body: String) -> Vec<String> {
    let split = body.split('\n');
    let mut lines = Vec::new();
    for s in split {
        verbose::dump_verbose(&("get_link_lines: PARSING LINE:".to_owned() + s));
        if s.contains(LINK_HTML)
            && s.contains(RPM_EXTENSION)
            && (s.contains(FEDORA_PROJECT)
                || s.contains(DOWNLOAD_REDHAT)
                || s.contains(DOWNLOAD_KOJIHUB))
        {
            verbose::dump_verbose(&("get_link_lines: LINK LINE:".to_owned() + s));
            lines.push(s.to_string());
        }
    }
    lines
}

pub fn get_links(link_lines: Vec<String>) -> Vec<String> {
    let mut links = Vec::new();
    for s in link_lines {
        verbose::dump_verbose(&("get_links: LINK LINE:".to_owned() + &s));
        let fields: Vec<&str> = s.split(LINK_HTML_EQUAL).collect();
        if fields.len() > 2 {
            let fields2: Vec<&str> = fields[2].split('>').collect();
            links.push(fields2[0].to_string().replace(&['\"'][..], ""));
        }
    }
    links
}

pub fn download_links(links: Vec<String>, ddata: DownloadData) -> Result<u32, &'static str> {
    let mut downloaded = 0;
    for l in &links {
        let lname = get_link_name(l);
        let mut download_path: String = "".to_string();
        if let Some(ref x) = ddata.directory() {
            download_path.push_str(x);
            if &download_path[download_path.len() - 1..] != "/" {
                download_path.push('/');
            }
        }
        download_path.push_str(&lname.to_owned());
        if ddata.test() {
            println!("Test mode: file:{} path:{}", l, download_path);
        } else {
            println!("Download file:{} Download path:{}", l, download_path);
            download_file(l, download_path).expect("Error on file download");
            downloaded += 1;
        }
    }
    if !links.is_empty() && 0 == downloaded && !ddata.test {
        return Err("Unable to download any link");
    }
    Ok(downloaded)
}

fn download_file(url: &str, path: String) -> Result<(), Box<dyn std::error::Error>> {
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
    fields[fields.len() - 1]
        .to_string()
        .replace(&['\"'][..], "")
}
