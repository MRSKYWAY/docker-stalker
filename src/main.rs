use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_inspect = format!("http://localhost:2375/containers/json");
    let response: Value = reqwest::get(&request_inspect).await?.json().await?;
    
    println!("\n\nDOCKER CONTAIER INFO::\n");
    print_container_info(&response);
    
    println!("\n\nDOCKER INSPECT::\n");
    let request_logs = format!("http://localhost:2375/images/json");
    let response2: Value = reqwest::get(&request_logs).await?.json().await?;

    print_images_info(&response2);

    Ok(())
}
fn print_images_info(response: &Value) {
    println!("{:<22} {:<13} {:<15}", "REPOSITORY", "TAG", "SIZE");

    if let Some(images) = response.as_array() {
        for image in images {
            let mut repository = image["RepoTags"]
                .as_array()
                .and_then(|tags| tags.get(0).and_then(|tag| tag.as_str()))
                .unwrap_or("");
            let cont = check_len(repository, 20);
            repository = &cont;
            let mut tag = repository.split(':').last().unwrap_or("");
            let cont = check_len(repository, 10);
            tag = &cont;
            let size_bytes = image["Size"].as_u64().unwrap_or(0);
            let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
            println!("{:<22} {:<13} {}MB", repository, tag, size_mb.round());
        }
    }
}
fn print_container_info(response: &Value) {
    println!(
        "{:<22} {:<10} {:<22} {:<20} {:<15}",
        "CONTAINER ID", "IMAGE", "COMMAND", "STATUS", "NAMES"
    );

    if let Some(containers) = response.as_array() {
        for container in containers {
            let mut container_id = container["Id"].as_str().unwrap_or("");
            let cont = check_len(container_id, 20);
            container_id = &cont;
            let mut image = container["Image"].as_str().unwrap_or("");
            let cont = check_len(image, 15);
            image = &cont;
            let mut command = container["Command"].as_str().unwrap_or("");
            let cont = check_len(command, 20);
            command = &cont;
            let mut created = container["Created"].as_str().unwrap_or("");
            let cont = check_len(created, 20);
            created = &cont;
            let mut status = container["Status"].as_str().unwrap_or("");
            let cont = check_len(status, 20);
            status = &cont;
            let mut names = container["Names"]
                .as_array()
                .map(|names| {
                    names
                        .iter()
                        .map(|name| name.as_str().unwrap_or(""))
                        .collect::<Vec<&str>>()
                        .join(",")
                })
                .unwrap_or_else(|| "".to_string());

            println!(
                "{:<22} {:<10} {:<22} {:<20} {:<15}",
                container_id, image, command, status, names
            );
        }
    }
}

fn check_len(mut name: &str, l: usize) -> String {
    let mut stri = String::new();
    if name.len() > l {
        name = &name[..l];
    }
    stri = name.to_string();
    return stri;
}
