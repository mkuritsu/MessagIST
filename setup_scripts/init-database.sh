echo ">> Setup database SSL"
sudo cp "../certs/ca.crt" /etc/ssl/certs/messagist_ca.crt
sudo cp "../certs/database.crt" /etc/ssl/certs/messagist_db.crt
sudo cp "../certs/database.key" /etc/ssl/private/messagist_db.key
sudo chown postgres:postgres /etc/ssl/certs/messagist_ca.crt
sudo chown postgres:postgres /etc/ssl/certs/messagist_db.crt
sudo chown postgres:postgres /etc/ssl/private/messagist_db.key
sudo chmod 0600 /etc/ssl/private/messagist_db.key
sudo cp "../database/pg_hba.conf" /etc/postgresql/17/main/pg_hba.conf
sudo cp "../database/postgresql.conf" /etc/postgresql/17/main/postgresql.conf

echo ">> Start database services"
sudo systemctl enable --now postgresql.service
sudo systemctl start postgresql.service

echo ">> Setup initial tables"
chmod ugo+r "../database/init.sql"
su postgres <<EOF
psql -c "create database messagist;"
psql -c "create user messagist_server with encrypted password '2Rk4M4LQGbrZB2j';"
psql -c "grant all privileges on database messagist to messagist_server;"
psql -d messagist -c "\ir ../database/init.sql;"
psql -d messagist -c "grant all privileges on table users to messagist_server;"
psql -d messagist -c "grant all privileges on table inmessages to messagist_server;"
psql -d messagist -c "grant all privileges on table outmessages to messagist_server;"
psql -d messagist -c "grant all privileges on table inmessages_id_seq to messagist_server;"
psql -d messagist -c "grant all privileges on table outmessages_id_seq to messagist_server;"
EOF

echo ">> Configuring network interfaces"
sudo cp ./network/database /etc/network/interfaces

echo ">> Configuring iptables"
sudo iptables -P INPUT DROP
sudo iptables -P OUTPUT DROP

sudo iptables -F INPUT
sudo iptables -F OUTPUT

sudo iptables -A INPUT -i lo -j ACCEPT
sudo iptables -A OUTPUT -o lo -j ACCEPT

sudo iptables -A INPUT -m state --state ESTABLISHED -j ACCEPT
sudo iptables -A OUTPUT -m state --state ESTABLISHED -j ACCEPT

sudo iptables -A INPUT -s 192.168.1.1 -p tcp --dport 5432 -m state --state NEW -j ACCEPT

sudo sh -c 'iptables-save > /etc/iptables/rules.v4'
sudo sh -c 'ip6tables-save > /etc/iptables/rules.v6'
sudo systemctl enable netfilter-persistent.service

sudo ifconfig eth0 192.168.1.2/24 up
sudo systemctl restart NetworkManager

echo ">> Database setup finished!"
