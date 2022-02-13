use std::{env, path::Path, error::Error};
use trim_image::*;

fn ptrim(path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);

    let file_name = path.file_name().ok_or("err")?.to_str().ok_or("err")?;
    let parent = path.parent().ok_or("err")?.as_os_str().to_str().ok_or("err")?;
    
    let img = load_image(&path)?;
    let trimmed = trim(&img)?;
    save_image(&format!("{parent}\\trimmed_{file_name}"), &trimmed)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        if let Err(err) = ptrim(&args[i]) {
            println!("Image {}, Error {:?}", args[i], err);
        }
    }
}