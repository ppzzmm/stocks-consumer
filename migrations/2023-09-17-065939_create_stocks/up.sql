create table users (
    id serial primary key,
    name varchar not null unique,
    email varchar(50) not null
);

create table stocks (
    id serial primary key,
    symbol varchar not null,
    shares integer,
    price varchar default '0.0',
    percentage_change varchar default '0.0',
    action_type varchar(10),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id integer references users not null
);

insert into users(name, email) values ('Pablo ZM', 'pablo.zuniga.mata@gmail.com');
