use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use std::{fs::File, fs::OpenOptions, io::BufReader, io::Write};

#[derive(serde::Deserialize)]
pub struct Course {
    pub course_id: String,
    pub course_name: String,
    pub instructor: String,
    pub semester: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Course Information management system")
}

pub fn write_to_file(course: Course) {
    // open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .open("./courseInfo.csv")
        .expect("Failed to open file");

    // write the user data to a file
    file.write_all(
        format!(
            "{},{},{},{}\n",
            course.course_id, course.course_name, course.instructor, course.semester
        )
        .as_bytes(),
    )
    .unwrap();
}

#[get("/addNew/{course_id}/{course_name}/{instructor}/{semester}")]
async fn submit(path: web::Path<(String, String, String, String)>) -> impl Responder {
    // Extract the tuple from the web::Path struct using into_inner()
    let (courseId, courseName, instructor, semester) = path.into_inner();

    // create a new course
    let course = Course {
        course_id: courseId,
        course_name: courseName,
        instructor: instructor,
        semester: semester,
    };

    // call the write_to_file function to write the data to a file
    write_to_file(course);

    HttpResponse::Ok().body("Successfully added.")
}
