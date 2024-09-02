use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
struct UserProfile {
    user_id: String,
    name: String,
    email: String,
    age: u32,
    address: String,
}
fn get_user_profiles() -> Vec<UserProfile> {
   vec![
       UserProfile {
           user_id: "0027".to_string(),
           name: "Priyanshu Singh".to_string(),
           email: "sprsingh624@gmail.comm".to_string(),
           age: 20,
           address: "Anugrah nagar bermo seam near durga mandgap 829104, jharkhand".to_string(),
       },
       UserProfile {
           user_id: "000002".to_string(),
           name: "Priyanshu Kumar singh".to_string(),
           email: "priyanshu@byteping.co.in".to_string(),
           age: 20,
           address: "Bokaro steel city".to_string(),
       },
           UserProfile {
           user_id: "2700".to_string(),
           name: "Suraj singh".to_string(),
           email: "suraj@byteping.co.in".to_string(),
           age: 20,
           address: "Bokaro steel city".to_string(),
           }
       // Add more users as needed
   ]
}
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

async fn get_user_profile(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Create a new owned string with a longer lifetime
    let user_id = query.get("userId").cloned().unwrap_or_else(|| "".to_string());
    let profiles = get_user_profiles();

    match profiles.iter().find(|&p| p.user_id == user_id) {
        Some(profile) => HttpResponse::Ok().json(profile),
        None => HttpResponse::NotFound().json(json!({"error": "User not found"})),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/profile", web::get().to(get_user_profile))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
