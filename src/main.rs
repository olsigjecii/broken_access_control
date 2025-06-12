use actix_web::{App, HttpResponse, HttpServer, Responder, get, patch, web};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// A thread-safe, in-memory "database" using DashMap.
// The key will be "studentid-subjectid".
type Db = Arc<DashMap<String, String>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting at http://127.0.0.1:8080");

    // Initialize our in-memory database with sample data
    let db: Db = Arc::new(DashMap::new());
    db.insert(
        "20223948-1293".to_string(), // Ezra's grade
        "F".to_string(),
    );
    db.insert(
        "20223949-1293".to_string(), // Another student's grade
        "A+".to_string(),
    );

    // we are using Actix-web for this example
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_grade)
            .service(update_grade)
            .service(secure_get_grade)
            .service(secure_update_grade)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Structs for handling request and response data
#[derive(Deserialize)]
struct GradeLookup {
    studentid: String,
    subjectid: String,
}

#[derive(Serialize)]
struct GradeResponse {
    grade: String,
}

#[derive(Deserialize)]
struct GradeUpdate {
    studentid: String,
    subjectid: String,
    grade: String,
}

#[derive(Serialize)]
struct UpdateResponse {
    message: String,
    new_grade: String,
}

#[derive(Debug)]
struct CurrentUser {
    id: String,
    role: String, // "student" or "teacher"
}

#[get("/grades")]
async fn get_grade(db: web::Data<Db>, query: web::Query<GradeLookup>) -> impl Responder {
    let grade_key = format!("{}-{}", query.studentid, query.subjectid);

    // VULNERABILITY: No check to see who is asking!
    match db.get(&grade_key) {
        Some(grade_ref) => HttpResponse::Ok().json(GradeResponse {
            grade: grade_ref.value().clone(),
        }),
        None => HttpResponse::NotFound().finish(),
    }
}

#[patch("/grades")]
async fn update_grade(db: web::Data<Db>, payload: web::Json<GradeUpdate>) -> impl Responder {
    let grade_key = format!("{}-{}", payload.studentid, payload.subjectid);

    // VULNERABILITY: No check to see if the user is a teacher!
    // Anyone can update any grade.
    db.insert(grade_key, payload.grade.clone());

    HttpResponse::Ok().json(UpdateResponse {
        message: "Grade updated successfully".to_string(),
        new_grade: payload.grade.clone(),
    })
}

// SECURED handler for GET /grades
#[get("/secure/grades")]
async fn secure_get_grade(db: web::Data<Db>, query: web::Query<GradeLookup>) -> impl Responder {
    let current_user = get_current_user_from_request();

    // ACCESS CONTROL CHECK
    if current_user.role == "student" && current_user.id != query.studentid {
        return HttpResponse::Forbidden().body("You can only view your own grades.");
    }

    // Teachers are allowed to view any grade.

    let grade_key = format!("{}-{}", query.studentid, query.subjectid);
    match db.get(&grade_key) {
        Some(grade_ref) => HttpResponse::Ok().json(GradeResponse {
            grade: grade_ref.value().clone(),
        }),
        None => HttpResponse::NotFound().finish(),
    }
}

#[patch("/secure/grades")]
async fn secure_update_grade(db: web::Data<Db>, payload: web::Json<GradeUpdate>) -> impl Responder {
    let current_user = get_current_user_from_request();

    // ROLE-BASED ACCESS CONTROL CHECK
    if current_user.role != "teacher" {
        return HttpResponse::Forbidden().body("You are not authorized to update grades.");
    }

    let grade_key = format!("{}-{}", payload.studentid, payload.subjectid);
    db.insert(grade_key, payload.grade.clone());

    HttpResponse::Ok().json(UpdateResponse {
        message: "Grade updated successfully".to_string(),
        new_grade: payload.grade.clone(),
    })
}

// This function simulates getting a user from request headers or a session.
// In a real app, this would involve validating a token from the `HttpRequest`.
fn get_current_user_from_request() -> CurrentUser {
    // For this lesson, we'll hardcode Ezra, a student.
    // To test the teacher role, you can change this.
    CurrentUser {
        id: "20223948".to_string(),
        role: "student".to_string(),
    }
    // Teacher example:
    // CurrentUser { id: "teacher-001".to_string(), role: "teacher".to_string() }
}
