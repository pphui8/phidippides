-- ALTER USER 'root'@'localhost' IDENTIFIED BY '123212321';
-- drop database myblog;
CREATE DATABASE myblog;
use myblog;

CREATE TABLE IF NOT EXISTS `blog`(
   `id` INT UNSIGNED AUTO_INCREMENT,
   `name` VARCHAR(128) NOT NULL,
   `descript` VARCHAR(256) NOT NULL,
   `article` VARCHAR(256),
   `tag` VARCHAR(128),
   PRIMARY KEY ( `id` )
)ENGINE=InnoDB DEFAULT CHARSET=utf8;

INSERT INTO blog (name, descript, article, tag)
VALUES ('pphui8', 'Ciallo～(∠・ω< )⌒★ ', 'pphui8', 'test');

INSERT INTO blog (name, descript, article, tag)
VALUES ('前端学习笔记', '我的前端学习笔记，从html、css、js到jquery、react，写得比较乱哈。。（学的也比较乱）', 'HTML_study_note', 'note');

-- delete from blog
-- where name="aaa";

CREATE TABLE IF NOT EXISTS `comment`(
   `id` INT UNSIGNED AUTO_INCREMENT,
   `username` VARCHAR(128),
   `url` VARCHAR(256),
   `value` VARCHAR(256),
   `time` VARCHAR(128),
   PRIMARY KEY ( `id` )
)ENGINE=InnoDB DEFAULT CHARSET=utf8;

INSERT INTO comment (username, url, value, time)
VALUES ('pphui8', 'pphui8.me', 'comment test', '2022/4/7');