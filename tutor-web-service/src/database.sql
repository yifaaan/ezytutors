DROP TABLE IF EXISTS ezy_course_c4;
CREATE TABLE ezy_course_c4
(
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    posted_time TIMESTAMP DEFAULT now()
);

/* for test */
INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
VALUES(1, 1, 'Frist course', '2024-9-21 05:40:00');

INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name, posted_time)
VALUES(2, 1, 'Second course', '2024-9-21 05:45:00');




/* add errors */
DROP TABLE IF EXISTS ezy_course_c5;
CREATE TABLE ezy_course_c5
(
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    posted_time TIMESTAMP DEFAULT now()
);


INSERT INTO ezy_course_c5 (course_id, tutor_id, course_name, posted_time)
VALUES(1, 1, 'Frist course', '2024-9-21 05:40:00');

INSERT INTO ezy_course_c5 (course_id, tutor_id, course_name, posted_time)
VALUES(2, 1, 'Second course', '2024-9-21 05:45:00');