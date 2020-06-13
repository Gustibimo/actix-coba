drop table if exists todo_item;
drop table if exists todo_list;

create table todo_list (
    id serial primary key,
    title varchar(150) not null
);

create table todo_item (
    id serial primary key,
    title varchar(150) not null,
    checked boolean not null default false,
    list_id int not null,
    foreign key(list_id) references todo_list(id)
);

insert into todo_list(title) values ('List 1'), ('list 2');

insert into todo_item (title, list_id)
    values ('item 1', 1), ('item 2', 1), ('item 3', 2);
