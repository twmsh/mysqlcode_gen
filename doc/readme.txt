docker pull mysql:8.0.28

docker run --name mysql8 -e MYSQL_ROOT_PASSWORD=fy123456  -v C:\\Users\\tom\\docker\\volums\\mysql_data:/var/lib/mysql -d mysql:8.0.28
