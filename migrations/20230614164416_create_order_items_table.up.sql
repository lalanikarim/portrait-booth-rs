-- Add up migration script here
CREATE TABLE `order_items` (
  id bigint unsigned auto_increment not null,
  order_id bigint unsigned not null,
  original_url varchar(255) not null,
  thumbnail_url varchar(255) null,
  processed_url varchar(255) null,
  created_at datetime not null,
  processed_at datetime null,
  primary key (id),
  foreign key (order_id) references `orders` (id)
)
