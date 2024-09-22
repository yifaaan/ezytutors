use actix_web::{web, Error, HttpResponse, Result};
use argon2::Config;
use serde_json::json;

use crate::iter6::{
    dbaccess::{get_user_record, post_new_user},
    errors::EzyTutorError,
    model::{TutorRegisterForm, TutorResponse, User},
    state::AppState,
};

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.clone()).await;
    let user_not_found = user.is_err();
    // 还没注册过的用户
    if user_not_found {
        // 要求两次输入的密码一样
        if params.password != params.confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            // 创建注册用户信息
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_pic_url": &params.imageurl,
                "tutor_profile": &params.profile
            });
            let awc_client = awc::Client::default();
            // 向tutor web service  发送post请求注册新tutor
            let res = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;
            let tutor_response: TutorResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
            s = format!("Congratulations. You have been successfully registered with EzyTutor and your tutor id is: {}. To start using EzyTutor, please login with your credentials.",tutor_response.tutor_id);
            // Hash the password
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash =
                argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                username,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash,
            };
            // 向ezytutor_web_ssr数据库插入tutor用户
            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        // 已注册的用户
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
