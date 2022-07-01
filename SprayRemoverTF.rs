use std::fs::{self, ReadDir};
use std::io::Error;
use std::process::Command;


 
fn main() {
 
    let demo_dir = fs::read_dir("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Team Fortress 2\\tf\\materials\\temp");
    println!("Deleting Sprays");
    delete_dir_contents(demo_dir);


let _child = Command::new("cmd.exe")
.arg("/C").arg("start").arg("").arg("steam://rungameid/440")
.spawn().expect("failed to launch TF2");

}

fn delete_dir_contents(read_dir_res: Result<ReadDir, Error>) {
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };

}


