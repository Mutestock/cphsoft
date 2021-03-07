create table if not exists tea (
	id SERIAL primary key not null,
	name VARCHAR(255),
	price_per_unit int,
	type VARCHAR(255)
);

create table if not exists tea_shop (
	id SERIAL primary key not null,
	name VARCHAR(255),
	location VARCHAR(255),
	phone_number int
);

create table if not exists storages (
	id SERIAL primary key not null,
	location VARCHAR(255),
	capacity int
);

create table if not exists agent (
	id SERIAL primary key not null,
	CPR VARCHAR(255),
	name VARCHAR(255)
);

create table if not exists orders (
	id SERIAL primary key not null,
	date_ordered TIMESTAMP,
	date_package_received TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	total_price int,
	total_units int
);

create table if not exists distributor (
	id SERIAL primary key not null,
	address VARCHAR(255),
	name VARCHAR(255)
);



