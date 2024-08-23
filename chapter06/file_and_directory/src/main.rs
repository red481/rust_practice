use std::fs;
use std::fs::File;
use std::io::{ self, Read, Write };
use std::path::{ Path, PathBuf };
use std::time::SystemTime;

#[test]
fn test_using_fs() -> io::Result<()> {
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!")?;

    let mut file = File::open("example.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content);
    println!("{}", content);

    Ok(())
}

#[test]
fn test_fs_read_dir() -> std::io::Result<()>{
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        println!("{}", file_name_str);
    }

    Ok(())
}

#[test]
fn test_path_module() {
    let path = Path::new("/tmp/test.txt");

    if let Some(filename) = path.file_name() {
        println!("파일명: {:?}", filename);
    }

    if let Some(extension) = path.extension() {
        println!("확장자: {:?}", extension);
    }

    let mut path_buf = PathBuf::from("\\tmp\\foo");

    path_buf.push("example.txt");
    println!("전체 경로: {:?}", path_buf);
}


fn list_files_and_directories(path: &std::path::Path, depth: usize) -> io::Result<()> {
    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            let file_name = entry_path.file_name().and_then(|os_str|os_str.to_str()).unwrap_or("");
            println!("{:indent$}{}", "", file_name, indent = depth);

            if entry_path.is_dir() {
                list_files_and_directories(&entry_path, depth + 2);
            }
        }
    }

    Ok(())
}

#[test]
fn test_list_files_and_directories_excute() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    println!("{}", current_dir.display());
    list_files_and_directories(&current_dir, 0);

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
