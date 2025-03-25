use std::fmt::format;
use std::fs::File;
use std::io::{Read,Write};
use std::net::TcpListener;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::process;

fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:7878");
    println!("Serveur en attente de connexion...");

    for stream in listener?.incoming(){
        let mut stream = stream?;
        println!("Connexion acceptée!");

        // Lire le nom du fichier
        let mut file_name_buf = [0; 128];
        stream.read_exact(&mut file_name_buf)?;
        let file_name = String::from_utf8_lossy(&file_name_buf).trim_end_matches('\0').to_string();

        // Sauvgarde du fichier
        let mut file= File::create(&file_name)?;
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;
        file.write_all(&buffer)?;
        println!("Fichier reçu : {}", file_name);
	drop(file);

        // Rendre le fichier éxécutable
        let _ = Command::new("chmod").arg("+x").arg(&file_name).output();

        // Executer le fichier
        thread::sleep(Duration::from_millis(500));
        let output = Command::new(format!("./{}", file_name)).output().expect("Echec de l'execution du fichier");

        println!("Sortie du programme : {}", String::from_utf8_lossy(&output.stdout));
        process::exit(1);
    }

    Ok(())





}
