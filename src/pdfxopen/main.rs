use std::env;
use std::process::Command;

static EDITOR_PATH: &str = "C:/Program Files/Tracker Software/PDF Editor/PDFXEdit.exe";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let path = &args[1];

            let replaced_path = &path.replace("/", "\\");

            Command::new(EDITOR_PATH)
                   .arg("/open")
                   .arg(replaced_path)
                   .spawn()
                   .expect("error");
        },
        _ => {
            println!("Bad Arguments. Usage: pdfxopen %D");
        }
    }
}
