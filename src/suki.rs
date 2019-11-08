use std::error::Error;
use std::io::{Read, Write};

/// SukiTags represent the tag -> filename collections that are contained by
/// SukiFiles.
#[derive(std::fmt::Debug)]
pub struct SukiTag {
    tag: String,
    files: Vec<String>,
}

/// SukiFiles are the primary operative component of suki, encapsulating all of
/// the tag -> filename relationships that `.suki` files encode for.
#[derive(std::fmt::Debug)]
pub struct SukiFile {
    tags: Vec<SukiTag>,
}

impl SukiFile {
    pub fn serialize(self, path: &str) -> Result<(), String> {
        let mut file = match open_and_clear_suki(path) {
            Ok(f) => f,
            Err(e) => return Err(String::from(e.description())) 
        };

        let mut buf = String::new();

        for tag in self.tags {
            buf.push_str(&format!("{}:\n", tag.tag));

            for f in tag.files {
                buf.push_str(&format!("\t{}\n", f))
            }
            
        }
        match file.write_all(buf.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(String::from(e.description()))
        }; 

        Ok(())
    }

    pub fn new(path: &str) -> Result<SukiFile, String> {

        let mut file_buffer = SukiFile { tags: Vec::new() };

        let mut txt_file = match open_suki(path) {
            Ok(f) => f,
            Err(e) => panic!(e),
        };

        let mut buf = String::new();
        match txt_file.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(e) => return Err(String::from(e.description())),
        }

        let mut line_no = 0;
        let mut tag_buffer: Option<SukiTag> = Option::None;
        for l in buf.split_terminator('\n') {
            line_no += 1;
            // Filenames are delimited with '\t', the tab character. This is
            // the primary trait that enforces a hierarchy of tags 1..* files.
            if l.starts_with('\t') {
                match tag_buffer.as_mut() {
                    Some(s) => s.files.push(String::from(&l[1..])),
                    None => return Err(format!("bad syntax - cannot start suki file with filename")),
                }
                continue;
            }

            // If the line ends with a colon, we assume it's intending to be a
            // tag.
            // TODO: This might require some refinement to enforce a more
            // concise and standard syntax, including ensuring that the line
            // starts with no whitespace.
            if l.ends_with(':') {
                // If we've already got a compiled tag on our plate, push it to
                // the file and make room for the new tag.
                match tag_buffer {
                    Some(s) => file_buffer.tags.push(s),
                    None => (),
                }
                // Set up the brand new tag with its label, and an empty vector
                // for its filenames.
                tag_buffer = Some(SukiTag {
                    // Right here we cut off the colon at the end of the line.
                    tag: String::from(&l[..l.len()-1]),
                    files: Vec::new(),
                });
                continue;
            } 

            // TODO: This is arbitrary. Pending removal/change of message?
            return Err(format!(
                "bad syntax at line {} - missing ':' at end of label descriptor",
                line_no
            ));
            
        }

        // At worse, we should have at least one tag left over that didn't get
        // a chance to be pushed. We push it here with a force-unwrap.
        // TODO: Fix this. It shouldn't force-unwrap, but for testing it's
        // good enough as a panic will tell us enough.
        file_buffer.tags.push(tag_buffer.unwrap());
        Ok(file_buffer)
    }
}

/// Opens a `.suki` file lying at the root of the given path.
/// 
/// Returns: an `std::io::Result` containing a handle to the file, or an I/O
/// error if something goes wrong.
fn open_suki(path: &str) -> std::io::Result<std::fs::File> {
    let path = suki_path(path);
    std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(std::path::Path::new(&path))
}

fn open_and_clear_suki(path: &str) -> std::io::Result<std::fs::File> {
    let path = suki_path(path);
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(std::path::Path::new(&path))
}

/// A convienince function for divining the actual literal path of a suki file
/// in a directory.
/// 
/// Returns: a `String` containing the path argument with `/.suki` appended.
fn suki_path(path: &str) -> String {
    let mut path = String::from(path);
    path.push_str("/.suki");

    path
}
