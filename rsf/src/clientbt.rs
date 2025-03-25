use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::env;
use std::thread;
use std::time::Duration;
use std::process::{Command, Stdio};

fn envoyer_signal_bluetooth() {

 
    println!("Envoi du signal Bluetooth...");

    // Exécute une commande système pour envoyer le signal via Bluetooth
    let mut child = Command::new("rfcomm")
        .arg("connect")
        .arg("hci0")
        .arg("B8:27:EB:B3:EC:7C")  // Remplace par l'adresse MAC du serveur
        .arg("1")
        .spawn()
        .expect("Échec de l'envoi Bluetooth");

    thread::sleep(Duration::from_secs(2));

	match OpenOptions::new().write(true).open("/dev/rfcomm0") {
		Ok(mut file) => {
			if let Err(e) = file.write_all(b"EXECUTER") {
				eprintln!("Erreur lors de l'envoir du signal : {}",e);
			}
			else {
				println!("Signal envoye");
			}
		}
		Err(e) => {
			eprintln!("Impossible d'ouvrir /dev/rfcomm0: {}", e);
		}
	}


    println!("Signal Bluetooth envoyé !");
   thread::sleep(Duration::from_secs(2));
   let _ = child.kill();
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
    
stream.shutdown(std::net::Shutdown::Both)?;
    println!("presh Enter to continue");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
    envoyer_signal_bluetooth();

    Ok(())
}
