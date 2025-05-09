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
#[path = "../src/links.rs"]
mod links;
#[path = "../src/verbose.rs"]
mod verbose;

const DST_DIR: &str = "/tmp";

const BODY: &str = "<html>
<head>Head</head>
<body>
<a href=\"https://kojipkgs.fedoraproject.org/packages/pykickstart/3.45/1.fc39/src/pykickstart-3.45-1.fc39.src.rpm\">download</a>
</body>
</html>";

const BODY2: &str = "<html>
<head>Head</head>
<body>
<a href=\"https://kojipkgs.fedoraproject.org/packages/pykickstart/3.45/1.fc39/src/pykickstart-3.45-1.fc39.src.rpm\">download</a>
<a href=\"https://kojipkgs.fedoraproject.org/packages/pykickstart/3.62/1.fc42/src/pykickstart-3.62-1.fc42.src.rpm\">download</a>
</body>
</html>";

const FILTER2: &str = "/3.62/";

const UNEXISTING_FILTER: &str = "/99.999.9999.99999/";

const BODY_NOT_DOWNLOADABLE: &str = "<html>
<head>Head</head>
<body>
<a href=\"https://1.2.3.does.not.exist/this_package_does_not_exist.rpm\">download</a>
</body>
</html>";

#[test]
fn links_downloadable_link_test() {
    verbose::is_verbose(false);
    match links::download_links(
        links::get_links(links::get_link_lines(String::from(BODY))),
        links::DownloadData::new(
            std::option::Option::Some::<String>(DST_DIR.to_string()),
            false,
            std::option::Option::None::<String>,
            true,
        ),
    ) {
        Ok(d) => assert_eq!(d, 1),
        Err(_e) => assert_eq!(0, 1),
    }
}

#[test]
fn links_downloadable_link_test_filter() {
    verbose::is_verbose(false);
    match links::download_links(
        links::get_links(links::get_link_lines(String::from(BODY2))),
        links::DownloadData::new(
            std::option::Option::Some::<String>(DST_DIR.to_string()),
            false,
            std::option::Option::Some::<String>(FILTER2.to_string()),
            true,
        ),
    ) {
        Ok(d) => assert_eq!(d, 1),
        Err(_e) => assert_eq!(0, 1),
    }
    match links::download_links(
        links::get_links(links::get_link_lines(String::from(BODY2))),
        links::DownloadData::new(
            std::option::Option::Some::<String>(DST_DIR.to_string()),
            false,
            std::option::Option::Some::<String>(UNEXISTING_FILTER.to_string()),
            true,
        ),
    ) {
        Ok(d) => assert_eq!(d, 0),
        Err(_e) => assert_eq!(0, 1),
    }
}

#[test]
fn links_not_dowloadable_link_test() {
    verbose::is_verbose(false);
    match links::download_links(
        links::get_links(links::get_link_lines(String::from(BODY_NOT_DOWNLOADABLE))),
        links::DownloadData::new(
            std::option::Option::Some::<String>(DST_DIR.to_string()),
            false,
            std::option::Option::None::<String>,
            true,
        ),
    ) {
        Ok(d) => assert_eq!(d, 0),
        Err(_e) => assert_eq!(0, 1),
    }
}
