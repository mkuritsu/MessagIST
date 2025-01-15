sudo cp ./network/alice /etc/network/interfaces

sudo iptables -P INPUT DROP
sudo iptables -P OUTPUT ACCEPT

sudo iptables -F INPUT
sudo iptables -F OUTPUT

sudo iptables -A INPUT -i lo -j ACCEPT
sudo iptables -A OUTPUT -o lo -j ACCEPT

sudo iptables -A INPUT -m state --state ESTABLISHED -j ACCEPT
sudo iptables -A OUTPUT -m state --state ESTABLISHED -j ACCEPT

sudo sh -c 'iptables-save > /etc/iptables/rules.v4'
sudo sh -c 'ip6tables-save > /etc/iptables/rules.v6'
sudo systemctl enable netfilter-persistent.service

sudo ifconfig eth0 192.168.0.1/24 up
sudo systemctl restart NetworkManager
