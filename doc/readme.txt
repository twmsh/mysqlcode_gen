
# mysql docker启动
docker pull mysql:8.0.28
docker run --name mysql8 -e TZ=Asia/Shanghai -p 3306:3306 -e MYSQL_ROOT_PASSWORD=fy123456  -v C:\\Users\\tom\\docker\\volums\\mysql_data:/var/lib/mysql -d mysql:8.0.28

# 创建mysql 用户
CREATE USER 'cf_user'@'%' IDENTIFIED BY 'cf123456';
GRANT ALL privileges ON cf_rs.* TO 'cf_user'@'%';
grant process on *.* to cf_user;
flush privileges;

# 创建数据库
CREATE DATABASE `cf_rs` CHARACTER SET 'utf8mb4';

create table be_user
(
    id           INTEGER PRIMARY KEY AUTO_INCREMENT,
    name         varchar(50),
    login_name   varchar(20) not null,
    password     varchar(50) not null,
    salt         varchar(20) not null,
    token        varchar(100),
    phone        varchar(50),
    email        varchar(100),
    service_flag SMALLINT default 1,
    ref_count    SMALLINT default 0,
    last_login   datetime,
    token_expire datetime,
    memo         varchar(100),
    gmt_create   datetime(3)    not null,
    gmt_modified datetime(3)    not null
);
create unique index idx_beuser_loginname on be_user (login_name);

insert into be_user(name,login_name,password,salt,gmt_create,gmt_modified)
values("aaa","aaa","aaa1","aaa2","2022-03-29 16:42:01","2022-03-29 16:42:01");

insert into be_user(name,login_name,password,salt,gmt_create,gmt_modified)
values("bbb","bbb","bbb1","bbb",now(),now());

insert into be_user(name,login_name,password,salt,gmt_create,gmt_modified)
values("ccc","ccc","ccc1","ccc2",now(),now());

insert into be_user(name,login_name,password,salt,gmt_create,gmt_modified,last_login)
values("kkk","kkk","kkk","kkk",now(),now(),now());




# 显示 mysql time_zone
show variables like '%time_zone%';
set time_zone='+08:00';


# 测试数据