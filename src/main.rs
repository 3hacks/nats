use std::process::{Command, Stdio};
use std::env;
use std::fs::File;
use std::fs;
use std::io::{Write, Read};
extern crate rand;
mod encrypter;
fn exec(command: &str) {
    let mut process = Command::new("bash").arg("-c").arg(command).stdout(Stdio::piped()).spawn().expect("failed to execute");
    let _ = process.wait();
}

fn random_bytes(n: i64) -> Vec<u8>{
    return (0..n).map( |_| {
        rand::random::<u8>()
    }).collect();
}

fn load(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("failed to load");
    let metadata = fs::metadata(filename).expect("failed to load");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("failed to load");
    return buffer;
}

fn save(filename: &str, data: Vec<u8>) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&data).expect("failed to write");
    file.flush().expect("failed to flush");
}

fn first_offset() -> Vec<u8> {
    return random_bytes(173);
}

fn last_offset() -> Vec<u8> {
    return random_bytes(135);
}

fn pack(command: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = command.clone();
    result.extend(&first_offset());
    result.extend(&data);
    result.extend(&last_offset());
    return result;
}


fn encrypt(filename: &str) {
    encrypter::encrypt_and_save_key(filename);
}

fn nats_in() {
    let args: Vec<String> = env::args().collect();
    encrypter::encrypt_one_file(&args[3]);
    let command = load(&args[2]);
    let data = load(&args[3]);
    let packed = pack(command, data);
    save(&format!("{}.dm", args[2]), packed);
}

fn nats_out() {
    let args: Vec<String> = env::args().collect();
    let src = load(&args[2]);
    let from: i64 = args[3].parse::<i64>().unwrap() + 173;
    let len: i64 = (src.len() as i64) - from - 135;
    let mut out: Vec<u8> = Vec::new();
    for i in from..len {
        out.push(src[i as usize]);
    }
    save("nats.out", out);
    encrypter::decrypt_one_file("nats.out", &args[4]);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "in" {
        nats_in();
    } else if args[1] == "out" {
        nats_out();
    }
}
