-- Add up migration script here
CREATE TABLE `settings` (
  name varchar(255) not null,
  value text not null,
  primary key (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
