use actix_web::{web, Error, HttpResponse};

use crate::{
    iter6::{dbaccess::get_user_record, errors::EzyTutorError, state::AppState},
    model::{TutorRegisterForm, TutorResponse, TutorSigninForm, User},
};
pub async fn show_signin_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_password", "");
    let s = tmpl
        .render("signin.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
pub async fn handle_signin(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorSigninForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.clone()).await;
    if let Ok(user) = user {
        // 验证密码
        let does_password_match =
            argon2::verify_encoded(&user.user_password.trim(), params.password.as_bytes()).unwrap();
        if !does_password_match {
            ctx.insert("error", "Invalid login");
            ctx.insert("current_name", &params.username);
            ctx.insert("current_password", &params.password);
            s = tmpl
                .render("signin.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            ctx.insert("name", &params.username);
            ctx.insert("title", &"Signin confirmation!".to_owned());
            ctx.insert(
                "message",
                &"You have successfully logged in to EzyTutor!".to_owned(),
            );
            s = tmpl
                .render("user.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        }
    } else {
        // 未注册过
        ctx.insert("error", "User id not found");
        ctx.insert("current_name", &params.username);
        ctx.insert("current_password", &params.password);
        s = tmpl
            .render("signin.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
