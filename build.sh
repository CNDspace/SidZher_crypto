curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install mysql-server mysql-client -y
echo "[mysqld]" >> /etc/mysql/my.cnf
echo "plugin-load-add=auth_socket.so" >> /etc/mysql/my.cnf
service mysql restart

echo "connect to DB and write this commands:"
echo "CREATE USER '$USER'@'localhost' IDENTIFIED WITH auth_socket;"
echo "CREATE DATABASE users;"
echo "CREATE TABLE users( id INT NOT NULL AUTO_INCREMENT, user VARCHAR(100) NOT NULL, pass VARCHAR(200) NOT NULL, PRIMARY KEY (id));"


# todo: create normal build script
