-- Add up migration script here
CREATE TABLE `users` (
  id int auto_increment not null,
  username varchar(255) not null,
  password_hash varchar(255) not null,
  role int not null,
  name varchar(255) not null,
  primary key (id),
  unique (username)
)
