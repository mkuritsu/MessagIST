sudo cp ./network/server /etc/network/interfaces

sudo iptables -P INPUT DROP
sudo iptables -P OUTPUT DROP

sudo iptables -F INPUT
sudo iptables -F OUTPUT

sudo iptables -A INPUT -i lo -j ACCEPT
sudo iptables -A OUTPUT -o lo -j ACCEPT

sudo iptables -A INPUT -m state --state ESTABLISHED -j ACCEPT
sudo iptables -A OUTPUT -m state --state ESTABLISHED -j ACCEPT

sudo iptables -A INPUT -p tcp --dport 8000 -m state --state NEW -j ACCEPT
sudo iptables -A OUTPUT -d 192.168.1.2 -p tcp --dport 5432 -m state --state NEW -j ACCEPT

sudo sh -c 'iptables-save > /etc/iptables/rules.v4'
sudo sh -c 'ip6tables-save > /etc/iptables/rules.v6'
sudo systemctl enable netfilter-persistent.service

sudo ifconfig eth0 192.168.0.3/24 up
sudo ifconfig eth1 192.168.1.1/24 up
sudo systemctl restart NetworkManager


