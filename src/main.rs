use std::{env, fs};
use std::fs::{metadata, File};
use std::io::Read;
//use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use fmodeparser::PermStrParser;

fn file_content_read(file_path: &Path) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn check_file_perms(file_path: &Path) -> std::io::Result<()> {
    let meta = metadata(file_path)?;
    let permission = meta.convert_permission_to_string().unwrap();
    println!("Permissions: {permission}");
    Ok(())
}

fn main() {
    if !cfg!(unix) {
        println!("chkprm is only available on Unix systems.");
        return;
    }
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    //println!("{}", &args[1]);
    let path = Path::new(&args[1]);
    match file_content_read(path) {
        Ok(content) => check_file_perms(path).unwrap(),
        Err(e) => eprintln!("Error: {}", e)
    }
    
    //println!("File contents:\n{:?}", fs::read_to_string(path));
}
