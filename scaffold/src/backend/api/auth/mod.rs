use actix_web::{Scope, web};


// mod register;
// mod login;
// mod logout;
// mod get_user_info;
// mod update_user_info;
//
// use crate::backend::api::auth::{register::register, login::login, logout::logout, get_user_info::get_user_info, update_user_info::update_user_info};

pub fn auth_scope() -> Scope {
    web::scope("/auth")
    //     .route("/register", web::post().to(register))  // 用户注册
    //     .route("/login", web::post().to(login))        // 用户登录
    //     .route("/logout", web::post().to(logout))      // 用户登出
    //     .route("/me", web::get().to(get_user_info))    // 获取用户信息
    //     .route("/update", web::put().to(update_user_info)) // 修改用户信息
}