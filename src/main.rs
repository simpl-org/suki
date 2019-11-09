const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod suki;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        print_help();
        return Ok(());
    }

    let cmd = match args.get(1) {
        Some(s) => String::from(s),
        None => panic!("args but no args? sanity check failed"),
    };

    match cmd.as_ref() {
        "t" | "tag" => 
            // Match the filename argument in an attempt to see if we've been
            // given a valid tag command.
            match args.get(2) {
                Some(s) => tag(s, args.split_at(3).1),
                None => Err(String::from("no filename supplied to tag")),
            }
        
        "r" | "remove" => match args.get(2) {
            Some(s) => remove(s, args.split_at(3).1),
            None => Err(String::from("no filename supplied to remove")),
        }
        "s" | "search" => Err(format!("unimplemented cmd - search")),
        "h" | "help" => {
            print_help();
            Ok(())
        }
        "v" | "version" => {
            print_version();
            Ok(())
        }
        s => Err(format!("unknown command {}", s))
    }

}

fn tag(filename: &str, tags: &[String]) -> Result<(), String> {
    let dir = curr_dir();
    println!("file: {}, tags: {:?}", filename, tags);

    let mut file = suki::SukiFile::new(&dir)?;

    if !tags.is_empty() {
        for t in tags {
            let mut found = false;
            for st in &mut file.tags {
                if t.eq(&st.tag) {
                    st.files.push(String::from(filename));
                    found = true;
                    break;
                }
            } 
            if !found {
                let mut new_tag = suki::SukiTag::new(t);
                new_tag.files.push(String::from(filename));
                file.tags.push(new_tag);
            }
        }
    }

    file.serialize("contrib")
}

fn remove(filename: &str, tags: &[String]) -> Result<(), String> {
    let dir = curr_dir();
    println!("file: {}, tags: {:?}", filename, tags);

    let mut file = suki::SukiFile::new(&dir)?;

    if !tags.is_empty() {
        for t in tags {
            for st in &mut file.tags {
                if t.eq(&st.tag) {
                    st.files.retain(|f| !f.eq(filename));
                    break;
                }
            } 
        }
    }

    file.serialize("contrib")
}

fn print_version() {
    println!(
        "{} version {} - the simple unique krap itemizer",
        bin_name(),
        VERSION
    );
}

fn print_help() {
    print_version();
    println!("commands:");
    println!("\t<t | tag> [flags] <filename> [tags]      adds file to the specified tags");
    println!(
        "\t<r | remove> [flags] <filename> [tags]   removes the tag(s) from the file specified"
    );
    println!("\t<s | search> [flags] [tags]              searches the tag database for files with the corresponding tag(s)");
    println!("\t<h | help>                               displays this help");
    println!("\t<v | version>                            displays version");
    println!("flags:");
    println!("\t-r                                       recursive search");
}

fn bin_name() -> String {
    match std::env::current_exe() {
        Ok(p) => match p.file_stem() {
            Some(s) => match s.to_str() {
                Some(st) => String::from(st),
                None => panic!("Path stem invalid unicode from '{:?}'", s),
            },
            None => panic!("Unable to resolve file stem from '{:?}'", p),
        },
        Err(e) => panic!("{}", e),
    }
}

fn curr_dir() -> String {
    match std::env::current_dir() {
        Ok(p) => match p.to_str() {
            Some(s) => String::from(s),
            None => panic!("unable to resolve dir {:?} to string", p),
        },
        Err(e) => panic!("{}", e),
    }
}
