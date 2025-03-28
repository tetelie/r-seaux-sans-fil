use std::fs::{File, OpenOptions};
use std::io::{Read, Write, self};
use std::net::TcpListener;
use std::process::Command;
use std::thread;
use std::time::Duration;

const CHUNK_SIZE: usize = 4096;

fn attendre_signal_bluetooth() {
    println!("En attente du signal Bluetooth...");

    let mut child = Command::new("rfcomm")
        .arg("watch")
        .arg("hci0")
        .arg("B8:27:EB:17:58:EB")  // Remplace par l'adresse MAC du client
	.spawn()
        .expect("Échec de la liaison Bluetooth");

    println!("Connexion réussie!");
	
    thread::sleep(Duration::from_secs(5));
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("failed to read line");
    let mut buffer = [0; 256];
    let mut file = File::open("/dev/rfcomm0").expect("Échec de l'ouverture de rfcomm0");
    loop {
	
        if let Ok(bytes_read) = file.read(&mut buffer) {
            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Signal Bluetooth reçu ! Exécution du fichier...");
            break;
        }

        //thread::sleep(Duration::from_secs(1));
    }
	let _ = child.kill();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    println!("Serveur en attente de connexion...");

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Connexion acceptée!");

        // Lire le nom du fichier
        let mut file_name_buf = [0; 128];
        stream.read_exact(&mut file_name_buf)?;
        let file_name = String::from_utf8_lossy(&file_name_buf).trim_end_matches('\0').to_string();

        // Vérifier la taille du fichier existant pour reprise
        let taille_recue = match std::fs::metadata(&file_name) {
            Ok(meta) => meta.len() as usize,
            Err(_) => 0,
        };

        // Renvoyer la taille reçue au client
        stream.write_all(&taille_recue.to_ne_bytes())?;

        // Ouvrir le fichier en mode append
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
	    .truncate(true)
            .open(&file_name)?;

        let mut buffer = [0; CHUNK_SIZE];

        // Recevoir les morceaux du fichier
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read])?;
        }

        println!("Fichier reçu : {}", file_name);
		
	io::stdout().flush().unwrap();
	io::stdout().flush().expect("Failed to flush stdout");

        drop(file);

        // Rendre le fichier exécutable
        let _ = Command::new("chmod").arg("+x").arg(&file_name).output();

        // Attendre le signal Bluetooth avant d'exécuter le fichie
        attendre_signal_bluetooth();
	

        // Exécuter le fichier
        thread::sleep(Duration::from_millis(500));
        let output = Command::new(format!("./{}", file_name)).output().expect("Échec de l'exécution");

        println!("Sortie du programme : {}", String::from_utf8_lossy(&output.stdout));
	println!("Fin du programme.");
    	break;
	}
    Ok(())
}
