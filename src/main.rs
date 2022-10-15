extern crate colored;
extern crate dirs;
extern crate fs_extra;
#[macro_use]
extern crate maplit;

use colored::Colorize;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn Error>> {
    let home_directory =
        Rc::new(dirs::home_dir().expect("Home directory is not defined for this system."));
    let dotfiles_directory = Path::new("..");
    let map = hashmap! {
        Path::new(".editorconfig") => home_directory.clone(),
        Path::new(".gemrc") => home_directory.clone(),
        Path::new("helix/") => Rc::new(home_directory.join(".config/")),
        Path::new(".ideavimrc") => home_directory.clone(),
        Path::new(".inputrc") => home_directory.clone(),
    };

    println!("{}", "Setting up your dotfiles...".blue());
    let _ = map
        .into_iter()
        .fold(Ok(()), |success, (file, target_directory)| {
            success?;
            let mut copy_options = CopyOptions::new();
            copy_options.overwrite = true;
            let source_paths = vec![dotfiles_directory.join(file)];
            let source_filename = file
                .to_str()
                .expect(&"Source filename contains invalid Unicode.".red())
                .yellow();
            let target_filename = target_directory
                .to_str()
                .expect(&"Target filename contains invalid Unicode.".red() as &str)
                .yellow();
            println!(
                "Copying {} {} over to {} ...",
                if source_filename.ends_with("/") {
                    "directory"
                } else {
                    "file"
                },
                source_filename,
                target_filename,
            );
            copy_items(&source_paths, &target_directory as &Path, &copy_options).map(|_| ())
        })
        .expect(&"Copying over dotfiles failed!".red() as &str);

    println!("{}", "Dotfile setup success!".green());

    Ok(())
}
