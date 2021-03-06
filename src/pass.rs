use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use std::time::Duration;
use std::error::Error;
use std::path::{PathBuf, Path};
use std::env;
use std::thread;
use std::fs::File;
use std::str;

use notify;
use gpgme;
use glob;

use notify::Watcher;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct PasswordEntry {
    pub name: String,
    pub meta: String,
    pub filename: String,
}

impl PasswordEntry {
    pub fn password(&self) -> Option<String> {
        let mut input = File::open(&self.filename).unwrap();

        // Decrypt password
        let mut ctx = gpgme::Context::from_protocol(gpgme::Protocol::OpenPgp).unwrap();
        let mut output = Vec::new();
        if let Err(e) = ctx.decrypt(&mut input, &mut output) {
            println!("decryption failed {:?}", e);
            return None;
        }
        let password = str::from_utf8(&output).unwrap();
        let firstline: String = password.split('\n').take(1).collect();
        Some(firstline)
    }
}

pub enum PasswordEvent {
    NewPassword,
}

pub type PasswordList = Arc<Mutex<Vec<PasswordEntry>>>;

pub fn search(l: &PasswordList, query: &str) -> Vec<PasswordEntry> {
    let passwords = l.lock().unwrap();
    fn normalized(s: &str) -> String {
        s.to_lowercase()
    };
    fn matches(s: &str, q: &str) -> bool {
        normalized(s).as_str().contains(normalized(q).as_str())
    };
    let matching = passwords.iter().filter(|p| matches(&p.name, query));
    matching.cloned().collect()
}

pub fn watch() -> Result<(Receiver<PasswordEvent>, PasswordList), Box<Error>> {

    let (password_tx, password_rx): (Sender<PasswordEntry>, Receiver<PasswordEntry>) = channel();
    let (event_tx, event_rx): (Sender<PasswordEvent>, Receiver<PasswordEvent>) = channel();

    let dir = password_dir()?;

    // Spawn watcher threads
    thread::spawn(move || {
        load_passwords(&dir, &password_tx).expect("failed loading passwords");
        watch_passwords(&dir, &password_tx).expect("failed watching password directory");
    });

    let passwords = Arc::new(Mutex::new(Vec::new()));
    let passwords_out = passwords.clone();

    // Spawn password list update thread
    thread::spawn(move || loop {
        let p = password_rx.recv().expect("password receiver channel failed");
        let mut passwords = passwords.lock().unwrap();
        passwords.push(p);
        event_tx.send(PasswordEvent::NewPassword).expect("password send channel failed")
    });
    Ok((event_rx, passwords_out))
}

fn to_name(base: &PathBuf, path: &PathBuf) -> String {
    path.strip_prefix(base)
        .unwrap()
        .to_string_lossy()
        .into_owned()
        .trim_right_matches(".gpg")
        .to_string()
}

fn to_password(base: &PathBuf, path: &PathBuf) -> PasswordEntry {
    PasswordEntry {
        name: to_name(base, path),
        filename: path.to_string_lossy().into_owned().clone(),
        meta: "".to_string(),
    }
}

/// Determine password directory
fn password_dir() -> Result<PathBuf, Box<Error>> {
    // If a directory is provided via env var, use it
    let pass_home = match env::var("PASSWORD_STORE_DIR") {
        Ok(p) => p,
        Err(_) => {
            env::home_dir()
                .unwrap()
                .join(".password-store")
                .to_string_lossy()
                .into()
        }
    };
    if !Path::new(&pass_home).exists() {
        return Err(From::from("failed to locate password directory"));
    }
    Ok(Path::new(&pass_home).to_path_buf())
}

fn load_passwords(dir: &PathBuf, tx: &Sender<PasswordEntry>) -> Result<(), SendError<PasswordEntry>> {
    let password_path_glob = dir.join("**/*.gpg");

    // Find all passwords
    let passpath_str = &password_path_glob.to_string_lossy();
    println!("path: {}", passpath_str);
    for entry in glob::glob(passpath_str).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => try!(tx.send(to_password(dir, &path))),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn watch_passwords(dir: &PathBuf, password_tx: &Sender<PasswordEntry>) -> Result<(), Box<Error>> {
    let (tx, rx) = channel();
    let mut watcher: notify::RecommendedWatcher = try!(notify::Watcher::new(tx, Duration::from_secs(2)));
    try!(watcher.watch(dir, notify::RecursiveMode::Recursive));

    loop {
        match rx.recv(){
            Ok(event) => {
                if let notify::DebouncedEvent::Create(path) = event {
                    try!(password_tx.send(to_password(dir, &path)));   
                }
            }
            Err(e) => println!("watch error: {:?}", e)
        }
    }
}
