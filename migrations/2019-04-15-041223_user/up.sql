DROP TABLE IF EXISTS `users`;
CREATE TABLE `users`
(
 `id`                    bigint unsigned NOT NULL AUTO_INCREMENT ,
 `email`                 varchar(64) NOT NULL ,
 `full_name`             varchar(255) NOT NULL ,
 `password`              varchar(255) NOT NULL ,
PRIMARY KEY (`id`),
UNIQUE KEY `user_email` (`email`)
);
