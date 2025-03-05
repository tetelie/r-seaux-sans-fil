
# Activate sudo
if [ "$(id -u)" -ne 0 ]; then
	sudo "$0" "$@"
	exit
fi

# desactivate NetworkManager
sudo systemctl NetworkManager.service

# setup adhoc mode
sudo ip link set wlan0 down
sudo iwconfig wlan0 mode ad-hoc
sudo iwconfig wlan0 essid "adhoc"
sudo iwconfig wlan0 channel 6
sudo ip addr add 129.168.1.1/24 dev wlan0 # change the ip address
sudo ip link set wlan0 up
