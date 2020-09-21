extern crate rusqlite;

use std::env::args;
use std::fmt::Formatter;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use rusqlite::{Connection, Result};

fn main() {
    let args = args().collect::<Vec<_>>();
    let path = PathBuf::from(args.get(1).expect("Failed to get first argument"))
        .canonicalize()
        .expect("Failed to get path");
    let conn = Connection::open(path).expect("Failed to connect");

    let mut stmt = conn
        .prepare(
            "\
             SELECT \
             url, keyword, short_name \
             FROM keywords \
             WHERE safe_for_autoreplace = 0\
             ",
        )
        .expect("Failed construction query");

    let entries: Vec<_> = stmt
        .query_map(&[], |row| {
            SearchEntry::new(row.get(0), row.get(1), row.get(2))
        })
        .expect("Failed to fetch rows")
        .map(|a| a.unwrap())
        .collect();

    println!(
        "
<!DOCTYPE NETSCAPE-Bookmark-file-1>
<!-- This is an automatically generated file.
     It will be read and overwritten.
     DO NOT EDIT! -->
<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">
<TITLE>Bookmarks</TITLE>
<H1>Chrome search</H1>

<DL><p>"
    );

    for entry in entries {
        println!("{}", entry);
    }

    println!("</DL>");
}

struct SearchEntry {
    url: String,
    keyword: String,
    title: String,
}

impl SearchEntry {
    fn new(url: String, keyword: String, title: String) -> Self {
        SearchEntry {
            url: url.replace("{searchTerms}", "%s"),
            keyword,
            title,
        }
    }
}

impl std::fmt::Display for SearchEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<DT><A HREF=\"{}\" SHORTCUTURL=\"{}\">{}</A>",
            self.url, self.keyword, self.title
        )
    }
}
