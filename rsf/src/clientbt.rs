use std::fs::File;
use std::io::{Read, Write, Seek};
use std::net::TcpStream;
use std::env;
use std::thread;
use std::time::Duration;
use std::process::Command;

const CHUNK_SIZE: usize = 4096;

fn envoyer_signal_bluetooth() {
    println!("Envoi du signal Bluetooth...");

    // Exécute une commande système pour envoyer le signal via Bluetooth
    let _ = Command::new("rfcomm")
        .arg("connect")
        .arg("hci0")
        .arg("00:11:22:33:44:55")  // Remplace par l'adresse MAC du serveur
        .arg("1")
        .output()
        .expect("Échec de l'envoi Bluetooth");

    thread::sleep(Duration::from_secs(2));

    let _ = Command::new("echo")
        .arg("EXECUTER")
        .arg(">")
        .arg("/dev/rfcomm0")  // Port série Bluetooth
        .output()
        .expect("Échec de l'envoi Bluetooth");

    println!("Signal Bluetooth envoyé !");
}

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

    // Récupérer la progression du transfert
    stream.write_all(b"TAILLE_FICHIER")?;
    let mut response = [0; 8];
    stream.read_exact(&mut response)?;
    let bytes_deja_recus = usize::from_ne_bytes(response);
    println!("Reprise du transfert à partir de {} octets", bytes_deja_recus);

    // Ouvrir et lire le fichier
    let mut file = File::open(file_path)?;
    file.seek(std::io::SeekFrom::Start(bytes_deja_recus as u64))?;

    // Envoyer le nom du fichier sur 128 octets
    let mut file_name_buf = [0; 128];
    let file_name = file_path.split('/').last().unwrap();
    file_name_buf[..file_name.len()].copy_from_slice(file_name.as_bytes());
    stream.write_all(&file_name_buf)?;

    // Envoyer le fichier par morceaux
    let mut buffer = [0; CHUNK_SIZE];
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        stream.write_all(&buffer[..bytes_read])?;
    }

    println!("Fichier envoyé : {}", file_path);

    // Envoi du signal Bluetooth pour exécuter le fichier
    envoyer_signal_bluetooth();

    Ok(())
}
