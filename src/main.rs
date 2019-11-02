const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use std::io::Read;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.is_empty() {
        print_help();
        return Ok(());
    }

    let a = match args.get(1) {
        Some(s) => String::from(s),
        None => panic!("args but no args? sanity check failed")
    };

    match a.as_ref() {
        "t" | "tag" => {
            match args.get(2) {
                Some(s) => return tag(s, args.split_at(3).1),
                None => return Err(String::from("no filename supplied to tag"))
            };
        },
        "r" | "remove" => println!("remove"),
        "s" | "search" => println!("search"),
        "h" | "help" => {
            print_help();
            return Ok(());
        },
        "v" | "version" => {
            print_version();
            return Ok(());
        }
        _ => println!("bad")
    };

    Ok(())

}

fn tag(filename: &str, tags: &[String]) -> Result<(), String> {
    let dir = curr_dir();
    println!("file: {}, tags: {:?}", filename, tags);
    let mut file = match open_suki(&dir) {
        Ok(f) => f,
        Err(e) => panic!(e)
    };

    let mut buf = String::new();
    file.read_to_string(&mut buf);
    let mut label_list: Vec<&str> = vec!();
    let mut line_no = 0;
    for l in buf.split_terminator('\n') {
        line_no += 1;
        if l.starts_with('\t') {
            continue;
        }
        if l.ends_with(':') {
            label_list.push(l)
        } else {
            return Err(format!("bad syntax at line {} - missing ':' at end of label descriptor", line_no))
        }
    }
    println!("{:?}", label_list);

    Ok(())
}

fn print_version() {
    println!("{} version {} - the simple unique krap itemizer", bin_name(), VERSION);
}

fn print_help() {
    print_version();
    println!("commands:");
    println!("\t<t | tag> [flags] <filename> [tags]      adds file to the specified tags");
    println!("\t<r | remove> [flags] <filename> [tags]   removes the tag(s) from the file specified");
    println!("\t<s | search> [flags] [tags]              searches the tag database for files with the corresponding tag(s)");
    println!("\t<h | help>                               displays this help");
    println!("\t<v | version>                            displays version");
    println!("flags:");
    println!("\t-r                                       recursive search");
}

fn suki_path(path: &str) -> String {
    let mut path = String::from(path);
    path.push_str("/.suki");

    path
}

fn open_suki(path: &str) -> std::io::Result<std::fs::File>{
    let path = suki_path(path);
    std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(std::path::Path::new(&path))
}

fn bin_name() -> String {
    match std::env::current_exe() {
        Ok(p) => match p.file_stem() {
            Some(s) => match s.to_str() {
                Some(st) => String::from(st),
                None => panic!("Path stem invalid unicode from '{:?}'", s)
            },
            None => panic!("Unable to resolve file stem from '{:?}'", p)
        }
        Err(e) => panic!("{}", e)
    }
}

fn curr_dir() -> String {
    match std::env::current_dir() {
        Ok(p) => match p.to_str() {
            Some(s) => String::from(s),
            None => panic!("unable to resolve dir {:?} to string", p)
        },
        Err(e) => panic!("{}", e)
    }
}