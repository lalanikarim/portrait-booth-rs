-- Add up migration script here
CREATE TABLE `users` (
  id bigint unsigned auto_increment not null,
  name varchar(255) not null,
  email varchar(255) null,
  phone varchar(255) null,
  password_hash varchar(255) null,
  otp_secret varchar(255) null,
  role tinyint not null,
  status tinyint not null default 0,
  primary key (id),
  unique (email),
  unique (phone)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
