-- Add up migration script here
CREATE TABLE `users` (
  id int auto_increment not null,
  name varchar(255) not null,
  email varchar(255) null,
  phone varchar(255) null,
  password_hash varchar(255) null,
  role int not null,
  status int not null default 0,
  primary key (id),
  unique (email),
  unique (phone)
)
