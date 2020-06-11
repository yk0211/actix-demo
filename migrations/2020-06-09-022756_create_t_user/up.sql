CREATE TABLE `t_user` (
  `uuid` char(36) NOT NULL,
  `account` varchar(64) NOT NULL,
  `password` varchar(64) NOT NULL,
  `nickname` varchar(32) NOT NULL,
  `gender` tinyint NOT NULL DEFAULT '0',
  `phone_number` char(11) NOT NULL,
  `head_image` varchar(128) NOT NULL,
  `create_at` timestamp NOT NULL,
  `last_login_at` timestamp NOT NULL,
  PRIMARY KEY (`uuid`),
  KEY `IDX_ACCOUNT_PASSWORD` (`account`,`password`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8;