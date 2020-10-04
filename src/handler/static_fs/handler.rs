use crate::file_explorer::FileExplorer;
use crate::handler::build_html;
use ascii::AsciiString;
use std::fs::{read_dir, File};
use tiny_http::{Request, Response};

/// Creates a path by merging the URL params and the `root_dir`.
/// Then reads the file system entries in the resulting path.
///
/// If the resulting path is a directory, enumerates every file system entry
/// and responds with an HTML with the entry listing.
///
/// Otherwise retrieves the provided file.
pub fn static_fs(request: Request, file_explorer: &FileExplorer) {
    match file_explorer.read(request.url().as_ref()) {
        Ok(entry) => {
            if entry.is_file {
                let mime_type = mime_guess::from_path(&entry.path)
                    .first_or_octet_stream()
                    .to_string();
                let mime_type = AsciiString::from_ascii(mime_type.as_bytes()).unwrap();
                let file = File::open(entry.path).unwrap();
                let response = Response::from_file(file).with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: mime_type,
                });

                request.respond(response).unwrap();
            } else {
                let dirpath = entry.path.clone();
                let dirpath = dirpath.to_str().unwrap();
                let dirname = &dirpath[file_explorer.root_dir_string.len()..];

                let entries = read_dir(entry.path).unwrap();

                let html = build_html(dirname, &file_explorer.root_dir_string, &file_explorer, entries);
                let mime_type_value: AsciiString = AsciiString::from_ascii("text/html").unwrap();

                request
                    .respond(
                        Response::from_string(html)
                            .with_status_code(200)
                            .with_header(tiny_http::Header {
                                field: "Content-Type".parse().unwrap(),
                                value: mime_type_value,
                            }),
                    )
                    .unwrap()
            }
        }
        Err(_) => request
            .respond(Response::from_string("Not Found").with_status_code(404))
            .unwrap(),
    }
}
