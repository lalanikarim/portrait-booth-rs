-- Add up migration script here
CREATE TABLE `order_items` (
  id bigint unsigned auto_increment not null,
  order_id bigint unsigned not null,
  original_get_url varchar(255) not null,
  original_put_url varchar(255) not null,
  original_uploaded tinyint(1) default 0,
  original_uploaded_at datetime null,
  thumbnail_get_url varchar(255) not null,
  thumbnail_put_url varchar(255) not null,
  thumbnail_uploaded tinyint(1) default 0,
  thumbnail_uploaded_at datetime null,
  processed_get_url varchar(255) not null,
  processed_put_url varchar(255) not null,
  processed_uploaded tinyint(1) default 0,
  processed_uploaded_at datetime null,
  created_at datetime not null,
  processed_at datetime null,
  primary key (id),
  foreign key (order_id) references `orders` (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
