/*run script

psql -U <user-name> -d ezytutors < <path.to.file>/tutor-course.sql

*/



/* Drop tables if they already exist*/

drop table if exists ezy_course_c6 cascade;
drop table if exists ezy_tutor_c6;

/* Drop database user if it exists and recreate it */
-- drop user if exists lyf;
-- create user lyf with password '1119';

/* Create tables. */
/* Note: Don't put a comma after last field */

create table ezy_tutor_c6 (
    tutor_id serial primary key,
    tutor_name varchar(200) not null,
    tutor_pic_url varchar(200) not null,
    tutor_profile varchar(2000) not null
);

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
    posted_time TIMESTAMP default now(),
    CONSTRAINT fk_tutor  /*外键名称*/
    FOREIGN KEY(tutor_id)
        REFERENCES ezy_tutor_c6(tutor_id)
        ON DELETE cascade
);

/* Grant privileges to specific user */
grant all privileges on table ezy_tutor_c6 to lyf;
grant all privileges on table ezy_course_c6 to lyf;
grant all privileges on all sequences in schema public to lyf;

/* Load seed data for testing */
insert into ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url,tutor_profile)
values(1,'Merlene','http://s3.amazon.aws.com/pic1','Merlene is an experienced finance professional');

insert into ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url,tutor_profile)
values(2,'Frank','http://s3.amazon.aws.com/pic2','Frank is an expert nuclear engineer');

insert into ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url,tutor_profile)
values(3,'Bob','http://s3.amazon.aws.com/pic3','Bob has spent many years teaching ML to students and professionals alike');

-- insert into ezy_course_c6
--     (course_id,tutor_id, course_name,course_level, posted_time)
-- values(1, 1, 'First course', 'Beginner' , '2024-09-5 05:40:00');
-- insert into ezy_course_c6
--     (course_id, tutor_id, course_name, course_format, posted_time)
-- values(2, 2, 'Second course', 'ebook', '2024-09-5 05:45:00');

-- insert into ezy_course_c6
--     (course_id, tutor_id, course_name, course_format, posted_time)
-- values(3, 1, 'Second course from author 1', 'ebook', '2024-09-6 05:45:00');

-- insert into ezy_course_c6
--     (course_id, tutor_id, course_name, course_format, posted_time)
-- values(4, 1, 'Third course from author 1', 'ebook', '2024-09-3 05:45:00');

-- insert into ezy_course_c6
--     (course_id, tutor_id, course_name, course_format, posted_time)
-- values(5, 3, 'First course from author 3', 'ebook', '2021-04-12 05:45:00');
