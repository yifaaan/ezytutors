/* Drop table if it already exists*/
drop table if exists ezy_course_c4;
/* Create a table. */
create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);
/* Load seed data for testing */
insert into ezy_course_c4
(course_id,tutor_id, course_name,posted_time)
values(1, 1, 'First course', '2024-9-4 15:45:00');

insert into ezy_course_c4
(course_id, tutor_id, course_name,posted_time)
values(2, 1, 'Second course', '2024-9-4 15:46:00');

/* psql -U lyf -d ezytutors < ./src/database.sql */




drop table if exists ezy_course_c5;

create table ezy_course_c5
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

/* Load seed data for testing */
insert into ezy_course_c5
    (course_id,tutor_id, course_name,posted_time)
values(1, 1, 'First course', '2024-9-5 05:40:00');

insert into ezy_course_c5
    (course_id, tutor_id, course_name,posted_time)
values(2, 1, 'Second course', '2021-9-5 05:45:00');




drop table if exists ezy_course_c6;

create table ezy_course_c6
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now()
);
