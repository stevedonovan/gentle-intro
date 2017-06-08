// file10.rs
use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let file = env::args().skip(1).next().unwrap_or("file10.rs".to_string());
    let path = Path::new(&file);
    match path.metadata() {
        Ok(data) => {
            println!("type {:?}",data.file_type());
            println!("len {}",data.len());
            println!("perm {:o}",data.permissions().mode());
            
            println!("modified {:?}",data.modified());
        },
        Err(e) => println!("error {:?}",e)
    }
}
