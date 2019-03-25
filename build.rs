use std::fs::canonicalize;
use std::path::Path;
use std::process::Command;

fn main() {
    // Run `npm_install` if necessary
    if !Path::new("./web/node_modules").exists() {
        Command::new("npm")
        .current_dir(canonicalize("./client/").expect("Could not canonicalize web folder path. Does it exist?"))
        .arg("install")
        .spawn()
        .expect("Failed to install node_modules. run `npm install` in the web/ folder for more info.")
        .wait().expect("Failed to wait for child process to finish");
    }

    // Build the project with `npm run build`
    Command::new("npm")
        .current_dir(
            canonicalize("./client/").expect("Could not canonicalize web folder path. Does it exist?"),
        )
        .args(&["run", "build"])
        .spawn()
        .expect("Failed build VueJS bundle. run `npm run build` in the web/ folder for more info.");
}
