docker pull mysql:8.0.28

docker run --name mysql8 -e TZ=Asia/Shanghai -p 3306:3306 -e MYSQL_ROOT_PASSWORD=fy123456  -v C:\\Users\\tom\\docker\\volums\\mysql_data:/var/lib/mysql -d mysql:8.0.28


CREATE DATABASE `cf` CHARACTER SET 'utf8mb4';

show variables like '%time_zone%';

set time_zone='+08:00';

CREATE USER 'cf_user'@'%' IDENTIFIED BY 'Cf2021&#';
GRANT ALL privileges ON cf.* TO 'cf_user'@'%';
grant process on *.* to cf_user;
flush privileges;

