use std::{env};
use std::fs::{metadata, File};
use std::io::Read;
use std::os::unix::fs::MetadataExt;
//use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use fmodeparser::{FullPermission, PermStrParser};

fn file_content_read(file_path: &Path) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn check_file_perms(file_path: &Path) -> std::io::Result<()> {
    let meta = metadata(file_path);
    let mut full_perm = FullPermission::new(meta?.mode()).unwrap();
    let permission = metadata(file_path)?.convert_permission_to_string().unwrap();
    let usr = full_perm.get_user().to_string();
    let group = full_perm.get_group().to_string();
    let other = full_perm.get_other().to_string();
    //let mut usr_string = "No user permissions.";
    //let mut group_string = "No group permissions.";
    //let mut other_string = "No other permissions.";
    // for user
    if usr.chars().nth(0) == Some('r') && usr.chars().nth(1) == Some('w') && usr.chars().nth(2) == Some('x') {
        println!("User: Full (read-write-execute), {usr} ({permission})");
    } else if usr.chars().nth(0) == Some('r') && usr.chars().nth(1) == Some('w') {
        println!("User: Modifiable (read-write), {usr} ({permission})");
    } else if usr.chars().nth(0) == Some('r') {
        println!("User: Read-only (read), {usr} ({permission})");
    } else if usr.chars().nth(1) == Some('w') {
        println!("User: Write-only (write), {usr} ({permission})");
    } else if usr.chars().nth(2) == Some('x') {
        println!("User: Executable-only (execute), {usr} ({permission})");
    } else {
        println!("No user permissions.");
    }
    // for group
    if group.chars().nth(0) == Some('r') && group.chars().nth(1) == Some('w') && group.chars().nth(2) == Some('x') {
        println!("Group: Full (read-write-execute), {usr} ({permission})");
    } else if group.chars().nth(0) == Some('r') && group.chars().nth(1) == Some('w') {
        println!("Group: Modifiable (read-write), {usr} ({permission})");
    } else if group.chars().nth(0) == Some('r') {
        println!("Group: Read-only (read), {usr} ({permission})");
    } else if group.chars().nth(1) == Some('w') {
        println!("Group: Write-only (write), {usr} ({permission})");
    } else if group.chars().nth(2) == Some('x') {
        println!("Group: Executable-only (execute), {usr} ({permission})");
    } else {
        println!("No group permissions.");
    }
    // for others
    if other.chars().nth(0) == Some('r') && other.chars().nth(1) == Some('w') && other.chars().nth(2) == Some('x') {
        println!("Other: Full (read-write-execute), {usr} ({permission})");
    } else if other.chars().nth(0) == Some('r') && other.chars().nth(1) == Some('w') {
        println!("Other: Modifiable (read-write), {usr} ({permission})");
    } else if other.chars().nth(0) == Some('r') {
        println!("Other: Read-only (read), {usr} ({permission})");
    } else if other.chars().nth(1) == Some('w') {
        println!("Other: Write-only (write), {usr} ({permission})");
    } else if other.chars().nth(2) == Some('x') {
        println!("Other: Executable-only (execute), {usr} ({permission})");
    } else {
        println!("No other permissions.");
    }
    //println!("Full permission: {}", usr.to_string());
    //println!("Permissions: {permission}");
    Ok(())
}

fn main() {
    if !cfg!(unix) {
        println!("chkprm is only available on Unix systems.");
        return;
    }
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chkprm <file_path>");
        return;
    }
    //dbg!(&args);
    //println!("{}", &args[1]);
    let path = Path::new(&args[1]);
    match file_content_read(path) {
        Ok(_content) => check_file_perms(path).unwrap(),
        Err(e) => eprintln!("Error: {}", e)
    }
    
    //println!("File contents:\n{:?}", fs::read_to_string(path));
}
