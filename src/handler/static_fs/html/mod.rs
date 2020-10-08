use crate::file_explorer::{Entry, FileExplorer};
use std::fs::ReadDir;

pub const HTML_DOCUMENT_START: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>HTTP Server | File Explorer</title>
  <style>
    body {
      background-color: #EFEFEF;
      color: #171B1F;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
      margin: 0;
      padding: 0;
    }

    .primary {
      color: #7DBDA3;
    }

    .secondary {
      color: #437CB0;
    }

    .danger {
      color: #DD6272;
    }

    .warning {
      color: #E6A04C;
    }

    #current-directory {
      background-color: #F7F7F7;
      box-sizing: border-box;
      color: #89909A;
      padding: 1rem .5rem;
    }

    #current-directory #container {
      /* display: grid; */
      /* grid-template-columns: minmax(200px, 30%) minmax(400px, 70%); */
      margin: 0 auto;
      width: 95%;
    }

    #current-directory #container #dirname {
      /* grid-column: 1 / 1; */
    }

    #current-directory #container #dirname h2 {
      margin: 0;
      margin-bottom: 1rem;
      padding: 0;
      text-align: left;
    }

    .code {
      background-color: #EFEFEF;
      color: #DD6272;
      border-radius: .25rem;
      margin: 0;
      padding: .3rem .6rem;
      text-align: left;
      letter-spacing: .1rem;
    }

    #current-directory #container #dirname code {
      margin-bottom: 1rem;
    }

    #current-directory #container #toolbar {
      /* grid-column: 2 / 2; */
    }

    #file-table {
      border-collapse: collapse;
      margin: 0 auto;
      width: 95%;
    }

    #file-table thead {
      text-align: left;
    }

    #file-table thead th {
      box-sizing: border-box;
      color: #7c7c7c;
      font-weight: 300;
      padding: 1rem;
    }

    #file-table tbody {
      background-color: #ffffff;
    }

    #file-table tbody tr td {
      box-sizing: border-box;
      padding: 1rem;
    }

    #file-table tbody tr td a {
      color: #437CB0;
      cursor: pointer;
      text-decoration: underline;
    }

    #file-table tbody tr:hover {
      background-color: #f8f8f8;
    }

    #fs-footer {
      box-sizing: border-box;
      margin: 0 auto;
      padding: 1rem;
      text-align: center;
      width: 95%;
    }

    #fs-footer small {
      color: #89909A;
    }

    #fs-footer small {
      color: #89909A;
    }
  </style>  
</head>
<body>"#;

pub const MAIN_INIT: &str = r##"<main>
<table id="file-table">
  <thead>
    <th width="35px">&nbsp;</th>
    <th>Name</th>
  </thead>
  <tbody>"##;
pub const TABLE_END: &str = r#"</tbody></table></main>"#;
pub const HTML_FOOTER: &str =
    r##"<footer id="fs-footer"><code class="code">http-server</code></footer>"##;
pub const HTML_DOCUMENT_END: &str = r##"</body></html>"##;
pub const FOLDER_ICON: &str = r##"<svg height='20px' width='30px'  fill="#437CB0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" x="0px" y="0px"><g data-name="22"><path d="M21,7H12.72L12,4.68A1,1,0,0,0,11,4H3A1,1,0,0,0,2,5V19a1,1,0,0,0,1,1H21a1,1,0,0,0,1-1V8A1,1,0,0,0,21,7Z"></path></g></svg>"##;
pub const FILE_ICON: &str = r##"<svg height='20px' width='30px'  fill="#437CB0" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.2" baseProfile="tiny" x="0px" y="0px" viewBox="0 0 80 80" xml:space="preserve"><polygon points="65,7.5 65,25 82.5,25"></polygon><polygon points="17.5,7.5 17.5,92.5 82.5,92.5 82.5,30 60,30 60,7.5"></polygon></svg>"##;

fn make_header(dirname: &str, root_dir: &str) -> String {
    let dirname = if dirname == "" { "/" } else { dirname };

    format!(
        r##"<header id="current-directory">
  <div id="container">
    <article id="dirname">
      <h2>{dirname}</h2>
      <span>🏠</span>&nbsp;<code class="code">{root_dir}</code>
    </article>
    <ul id="toolbox"></ul>
  </div>
</header>"##,
        dirname = dirname,
        root_dir = root_dir
    )
}

fn make_html_table_row(fsystem: &FileExplorer, fs_entry: &Entry) -> String {
    let icon = if fs_entry.is_file {
        FILE_ICON
    } else {
        FOLDER_ICON
    };

    let full_path = fs_entry.path.to_str().unwrap();
    let mut link_text = full_path;

    if let Some(last_slash_index) = fs_entry.path.to_str().unwrap().rfind('/') {
        link_text = &link_text[last_slash_index + 1..];
    }

    format!(
        r##"<tr><td width="35px">{icon}</td><td><a href="{filepath}">{filename}</a></td></tr>"##,
        icon = icon,
        filepath = fsystem.to_relative_path(full_path).unwrap(),
        filename = link_text,
    )
}

pub fn build_html(
    dirname: &str,
    root_dir: &str,
    fsystem: &FileExplorer,
    entries: ReadDir,
) -> String {
    let mut html = String::from(HTML_DOCUMENT_START);

    html.push_str(make_header(dirname, root_dir).as_str());
    html.push_str(MAIN_INIT);

    let mut entries = entries
        .map(|dir_entry| Entry::from(dir_entry.unwrap()))
        .collect::<Vec<Entry>>();

    entries.sort();

    entries.into_iter().for_each(|entry| {
        html.push_str(make_html_table_row(fsystem, &entry).as_str());
    });

    html.push_str(TABLE_END);
    html.push_str(HTML_FOOTER);
    html.push_str(HTML_DOCUMENT_END);

    html
}
