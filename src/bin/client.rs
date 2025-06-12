use futures::future::join_all;
use serde::Deserialize;

// This struct will help us parse the JSON response from the server.
// It only needs the fields we care about.
#[derive(Deserialize, Debug)]
struct GradeResponse {
    grade: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut tasks = vec![];

    for id in 1111..9999 {
        let student_id = format!("2022{}", id);
        let url = format!(
            "http://127.0.0.1:8080/grades?subjectid=1293&studentid={}",
            student_id
        );
        let client = client.clone();

        // Spawn a new asynchronous task for each request. This allows all the requests to run concurrently, not one by one.
        let task = tokio::spawn(async move {
            match client.get(&url).send().await {
                Ok(response) => {
                    // Check if the server responded with a success status (e.g., 200 OK)
                    if response.status().is_success() {
                        // Try to parse the JSON response into our GradeResponse struct
                        match response.json::<GradeResponse>().await {
                            Ok(data) => {
                                println!(
                                    "[SUCCESS] Student: {}, Grade: {}",
                                    student_id, data.grade
                                );
                            }
                            Err(_) => {
                                // This might happen if the response isn't valid JSON
                                eprintln!(
                                    "[ERROR] Student: {}: Failed to parse JSON response.",
                                    student_id
                                );
                            }
                        }
                    }
                }
                Err(_) => {
                    eprintln!(
                        "[ERROR] Student: {}: Request failed (is the server running?)",
                        student_id
                    );
                }
            }
        });

        tasks.push(task);
    }

    // Wait for all the spawned tasks to complete.
    join_all(tasks).await;
    println!("--- Scan finished ---");
}
