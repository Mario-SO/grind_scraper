use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let path_to_write = "/Users/mario/Desktop/Homy/Dev/github/grind75/src/";

    let response =
        reqwest::blocking::get("https://www.techinterviewhandbook.org/grind75?grouping=none")
            .unwrap()
            .text()
            .unwrap();

    let document = scraper::Html::parse_document(&response);

    let name_selector = scraper::Selector::parse("a.text-indigo-600").unwrap();
    let names = document.select(&name_selector).map(|x| x.inner_html());

    let template = r#"pub fn problem_name() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {}
}
"#;
    let mut lib_content = String::new();
    names
        .zip(1..76) // start from 1 to 75
        .for_each(|(item, number)| {
            let item = item.replace(" ", "_");

            let file_name = format!("lc_{}_{}", number, item.to_lowercase()); // create the filename
            lib_content.push_str(&format!(
                "pub mod {};\npub use {}::*;\n\n",
                &file_name, &file_name
            ));

            let mut path = PathBuf::from(path_to_write);
            path.push(&file_name);
            path.set_extension("rs"); // set the file extension to .rs

            // Open a file in write-only mode, returns `io::Result<File>`
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", path.display(), why),
                Ok(file) => file,
            };

            // Write the template string to `file`, returns `io::Result<()>`
            match file.write_all(template.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
                Ok(_) => println!("successfully wrote to {}", path.display()),
            }
        });

    // Write the lib.rs file
    let mut path = PathBuf::from(path_to_write);
    path.push("lib");
    path.set_extension("rs"); // set the file extension to .rs
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    match file.write_all(lib_content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => println!("successfully wrote to {}", path.display()),
    }
}
