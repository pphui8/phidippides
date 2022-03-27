-- ALTER USER 'root'@'localhost' IDENTIFIED BY '123212321';
CREATE DATABASE myblog;
use mylog;

CREATE TABLE IF NOT EXISTS `blog`(
   `id` INT UNSIGNED AUTO_INCREMENT,
   `name` VARCHAR(128) NOT NULL UNIQUE,
   `descript` VARCHAR(256) NOT NULL,
   `article` VARCHAR(256),
   PRIMARY KEY ( `id` )
)ENGINE=InnoDB DEFAULT CHARSET=utf8;

INSERT INTO blog (name, descript,  article)
VALUES ('pphui8', 'hello world', 'https://api.pphui8.me');

delete from blog
where name="aaa";

-- delete from blog
-- where name="test";

CREATE TABLE IF NOT EXISTS `blog_comment`(
   `id` VARCHAR(50),
   `blog_id` INT UNSIGNED NOT NULL,
   `value` VARCHAR(256) NOT NULL,
   PRIMARY KEY ( `id` ),
   FOREIGN KEY blog_comment(blog_id)
   REFERENCES blog(id)
   ON UPDATE CASCADE
   ON DELETE RESTRICT
)ENGINE=InnoDB DEFAULT CHARSET=utf8;

INSERT INTO blog_comment (id, blog_id,  value)
VALUES (UUID(), 1, 'hello world');