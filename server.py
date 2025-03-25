import socket
import os
import subprocess
import bluetooth

# Fonction pour recevoir un fichier via TCP
def receive_file_via_tcp(conn, file_name):
    with open(file_name, 'wb') as f:
        while True:
            data = conn.recv(1024)
            if not data:
                break
            f.write(data)
    print(f"Fichier {file_name} reçu avec succès.")

# Fonction pour exécuter un fichier via Bluetooth
def execute_file_via_bluetooth(file_name):
    # Simuler l'exécution du fichier via Bluetooth (ou le lancement à distance)
    print(f"Exécution du fichier via Bluetooth: {file_name}")
    subprocess.run(["chmod", "+x", file_name])  # Rendre le fichier exécutable
    subprocess.run([f"./{file_name}"])  # Exécution du fichier

# Serveur TCP pour réception du fichier
def start_tcp_server():
    HOST = '0.0.0.0'  # Écoute sur toutes les interfaces
    PORT = 12345       # Port d'écoute

    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((HOST, PORT))
    server_socket.listen(1)

    print(f"Serveur en attente de connexion TCP sur {HOST}:{PORT}...")

    conn, addr = server_socket.accept()
    print(f"Connexion établie avec {addr}")

    file_name = conn.recv(1024).decode()  # Recevoir le nom du fichier
    print(f"Réception du fichier {file_name} via TCP...")

    receive_file_via_tcp(conn, file_name)  # Réception du fichier via TCP

    # Fermeture de la connexion TCP
    conn.close()
    server_socket.close()

    return file_name

# Serveur Bluetooth pour exécution du fichier
def start_bluetooth_server(file_name):
    # Configuration du serveur Bluetooth
    server_sock = bluetooth.BluetoothSocket(bluetooth.RFCOMM)
    server_sock.bind(("", bluetooth.PORT_ANY))
    server_sock.listen(1)

    print("Serveur en attente de connexion Bluetooth...")

    client_sock, client_info = server_sock.accept()
    print(f"Connexion Bluetooth établie avec {client_info}")

    # Exécution du fichier via Bluetooth (simulation)
    execute_file_via_bluetooth(file_name)

    client_sock.close()
    server_sock.close()

# Fonction principale
def main():
    # Étape 3 : Recevoir un fichier via TCP (Wi-Fi)
    file_name = start_tcp_server()

    # Étape 4 : Lancer l'exécution via Bluetooth
    start_bluetooth_server(file_name)

if __name__ == "__main__":
    main()
