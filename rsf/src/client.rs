use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Utilisation : client <IP> <fichier>");
        std::process::exit(1);
    }

    let server_ip = &args[1];
    let file_path = &args[2];

    let mut stream = TcpStream::connect(format!("{}:7878", server_ip))?;
    println!("Connecté à {}!", server_ip);

    // Ouvrir et lire le fichier
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Envoyer le nom du fichier sur 128 octets (rempli avec des '\0')
    let mut file_name_buf = [0; 128];
    let file_name = file_path.split('/').last().unwrap();
    file_name_buf[..file_name.len()].copy_from_slice(file_name.as_bytes());
    stream.write_all(&file_name_buf)?;

    // Envoyer le fichier
    stream.write_all(&buffer)?;
    println!("Fichier envoyé : {}", file_path);

    Ok(())
}
