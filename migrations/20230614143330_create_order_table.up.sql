-- Add up migration script here
CREATE TABLE `orders` (
  id bigint unsigned auto_increment not null,
  customer_id bigint unsigned not null,
  cashier_id bigint unsigned null,
  operator_id bigint unsigned null,
  processor_id bigint unsigned null,
  no_of_photos bigint unsigned not null,
  order_total bigint unsigned not null,
  mode_of_payment tinyint not null,
  order_ref varchar(255) null,
  payment_ref varchar(255) null,
  status tinyint not null,
  created_at datetime not null,
  payment_at datetime null,
  primary key (id),
  foreign key (customer_id) references users (id) on delete cascade,
  
  foreign key (cashier_id) references users (id) on delete cascade,
  foreign key (operator_id) references users (id) on delete cascade,
  foreign key (processor_id) references users (id) on delete cascade
)
