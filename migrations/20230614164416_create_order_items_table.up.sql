-- Add up migration script here
CREATE TABLE `order_items` (
  id bigint unsigned auto_increment not null,
  order_id bigint unsigned not null,
  file_name varchar(255) not null,
  mode tinyint not null,
  get_url text not null,
  put_url text not null,
  uploaded tinyint(1) not null default 0,
  uploaded_at datetime null,
  created_at datetime not null,
  primary key (id),
  foreign key (order_id) references `orders` (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
