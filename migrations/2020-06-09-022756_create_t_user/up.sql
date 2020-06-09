CREATE TABLE `t_user` (
  `uuid` char(36) NOT NULL,
  `gender` tinyint NOT NULL DEFAULT '0',
  `phone_number` char(11) NOT NULL,
  `head_url` varchar(128) NOT NULL,
  `account` varchar(64) NOT NULL,
  `password` varchar(64) NOT NULL,
  `create_at` timestamp NOT NULL,
  `update_at` timestamp NOT NULL,
  PRIMARY KEY (`uuid`),
  KEY `IDX_ACCOUNT_PASSWORD` (`account`,`password`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8;