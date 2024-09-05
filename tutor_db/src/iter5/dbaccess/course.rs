use crate::errors::EzyTutorError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use actix_web::cookie::time::Duration;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // Course 实现了From<sqlx::FromRow>, sqlx自动将Row转化为Course结构
    let course_rows = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 where tutor_id = $1",
        tutor_id
    ) // 定义sql语句
    .fetch_all(pool) // 执行查询语句
    .await?; // EzyTutorError 实现了From<sqlx::error::Error>

    // 如果没有derive(FromRow)，就要自行转换
    // let courses: Vec<_> = course_rows
    //     .iter()
    //     .map(|course_row| Course {
    //         course_id: course_row.course_id,
    //         tutor_id: course_row.tutor_id,
    //         course_name: course_row.course_name.clone(),
    //         posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    //     })
    //     .collect();

    // match courses.len() {
    //     0 => Err(EzyTutorError::NotFound(
    //         "Courses not found for tutor".into(),
    //     )),
    //     _ => Ok(courses),
    // }
    Ok(course_rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_optional(pool) // 没找到就返回None
    .await?;

    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query_as!(Course,
        "insert into ezy_course_c6 
            (tutor_id, course_name, course_description, course_duration, course_level, course_format, course_language, course_structure, course_price) 
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning
        tutor_id, course_id, course_name, course_description, course_duration, course_level, course_format, course_language, course_structure, course_price, posted_time",
        new_course.tutor_id,
        new_course.course_name,
        new_course.course_description,
        new_course.course_duration,
        new_course.course_level,
        new_course.course_format,
        new_course.course_language,
        new_course.course_structure,
        new_course.course_price
    ) // insert语句
    .fetch_one(pool) // 执行insert
    .await?;
    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    let course_row = sqlx::query!(
        "DELETE FROM ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .execute(pool)
    .await?;
    Ok(format!("Deleted {:#?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| EzyTutorError::NotFound("Course id not found".into()))?;

    // 判断更新的字段
    let name = if let Some(name) = update_course.course_name {
        name
    } else {
        current_course_row.course_name
    };
    let description = if let Some(desc) = update_course.course_description {
        desc
    } else {
        current_course_row.course_description.unwrap_or_default()
    };
    let format = if let Some(format) = update_course.course_format {
        format
    } else {
        current_course_row.course_format.unwrap_or_default()
    };
    let structure = if let Some(structure) = update_course.course_structure {
        structure
    } else {
        current_course_row.course_structure.unwrap_or_default()
    };
    let duration = if let Some(duration) = update_course.course_duration {
        duration
    } else {
        current_course_row.course_duration.unwrap_or_default()
    };
    let level = if let Some(level) = update_course.course_level {
        level
    } else {
        current_course_row.course_level.unwrap_or_default()
    };
    let language = if let Some(language) = update_course.course_language {
        language
    } else {
        current_course_row.course_language.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.course_price {
        price
    } else {
        current_course_row.course_price.unwrap_or_default()
    };
    let course_row =
        sqlx::query_as!(
        Course,
        "UPDATE ezy_course_c6 set course_name = $1, course_description = $2, course_format = $3, 
        course_structure = $4, course_duration = $5, course_price = $6, course_language = $7, 
        course_level = $8 where tutor_id = $9 and course_id = $10 returning tutor_id, course_id, 
        course_name, course_description, course_duration, course_level, course_format, 
        course_language, course_structure, course_price, posted_time ", name, description, format, 
        structure, duration, price, language,level, tutor_id, course_id)
        .fetch_one(pool)
        .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}
