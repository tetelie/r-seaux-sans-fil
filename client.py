import socket
import os
import bluetooth

# Fonction pour envoyer un fichier via TCP
def send_file_via_tcp(file_path, server_ip, server_port):
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect((server_ip, server_port))

    file_name = os.path.basename(file_path)

    # Envoi du nom du fichier
    client_socket.send(file_name.encode())

    # Envoi du fichier en morceaux
    with open(file_path, 'rb') as f:
        while (data := f.read(1024)):
            client_socket.send(data)

    print(f"Fichier {file_name} envoyé avec succès.")

    client_socket.close()

# Fonction pour envoyer un signal Bluetooth (facultatif, si nécessaire pour l'exécution)
def send_bluetooth_signal(server_address):
    sock = bluetooth.BluetoothSocket(bluetooth.RFCOMM)
    sock.connect((server_address, 1))  # Connexion à l'adresse MAC du serveur Bluetooth

    # Envoi d'un message ou signal (facultatif)
    sock.send("EXECUTE")  # Message de commande pour l'exécution

    sock.close()

# Fonction principale
def main():
    SERVER_IP = 'IP_RPI_B'  # Remplacez par l'IP de votre serveur RPi B
    SERVER_PORT = 12345     # Port du serveur

    file_path = "chemin/vers/le/fichier/mon_programme"  # Remplacez par le chemin du fichier

    # Étape 3 : Envoi du fichier via TCP (Wi-Fi)
    send_file_via_tcp(file_path, SERVER_IP, SERVER_PORT)

    # Optionnel : Si vous voulez envoyer un signal via Bluetooth pour exécuter le fichier
    SERVER_BT_ADDRESS = "XX:XX:XX:XX:XX:XX"  # Adresse MAC du serveur Bluetooth
    send_bluetooth_signal(SERVER_BT_ADDRESS)

if __name__ == "__main__":
    main()
