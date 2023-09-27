use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
struct Port {
    private_port: u64,
    public_port: u64,
}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
struct Containers {
    id: Option<String>,
    image: String,
    command: String,
    status: String,
    ports: Vec<Port>,
    names: Vec<String>,
}
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
struct Images {
    created: i32,
    repo_tags: Vec<String>,
    size: i64,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let request_inspect = format!("http://localhost:2375/containers/json");
    let res = get_response(&request_inspect).await?;
    let response: Vec<Containers> = serde_json::from_str(&res).unwrap();
    println!(
        "{:<22} {:<10} {:<22} {:<20} {:<20} {:<15}",
        "CONTAINER ID", "IMAGE", "COMMAND", "STATUS", "PORTS", "NAMES"
    );
    for containers in response {
        print_container_info(containers);
    }

 
    let request_logs = format!("http://localhost:2375/images/json");
    let res2 = get_response(&request_logs).await?;
    let response2: Vec<Images> = serde_json::from_str(&res2).unwrap();
    println!("{:<22} {:<13} {:<15}", "REPOSITORY", "TAG", "SIZE");

    for images in response2{
        print_images_info(images);
    }
    Ok(())
}
fn print_images_info(response: Images) {
    let mut names: Vec<String> = Vec::new();
    let mut tags: Vec<String> = Vec::new();
    for name in &response.repo_tags {
        let v: Vec<&str> = name.split(':').collect();
        if v.len() == 2 {
            names.push(v[0].to_string());
            tags.push(v[1].to_string());
        }
    }
    let name = names.join(", ");
    let tag = tags.join(", ");    
    let size_bytes = response.size;
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    println!("{:<22.15} {:<13} {}MB ", name, tag, size_mb.round());
        
    
}
fn print_container_info(res: Containers) {
    let id = res.id.unwrap();
    let image = res.image;
    let command = res.command;
    let status = res.status;
    let names = res.names.join(",");
    let ports =  res.ports.iter().map(|port| format!("{},{}", port.private_port, port.public_port)).collect::<Vec<String>>().join(",");
    
    println!(
        "{:<22.15}{:<10.10}{:<22.10}{:<20.16}{:<20.10}{:<15.20}",
        id,        image,        command,        status,        ports,        names,
    );   
    
}
async fn get_response(end: &String) -> Result<String, reqwest::Error>{

    let response = reqwest::get(end).await?.text().await?; // Explicitly specify T
    Ok(response)

}
#[tokio::test]
async fn check_connection() {
    // Create a test runtime and run the main function asynchronously
    let test = String::from("http://localhost:2375/containers/json");
    let result = get_response(&test).await;
    // Assert that the result is an Ok variant
    assert!(result.is_ok());
}
