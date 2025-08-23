use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use futures::future::join_all;
use reqwest::header::CONTENT_TYPE;
use chrono::{Local};

#[tokio::main]
async fn main() {

    let urls:Vec<&str> = vec![
    "https://images.unsplash.com/photo-1755331039789-7e5680e26e8f?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxmZWF0dXJlZC1waG90b3MtZmVlZHwxfHx8ZW58MHx8fHx8",
    "https://images.unsplash.com/photo-1755398103904-1c1f4466b535?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxmZWF0dXJlZC1waG90b3MtZmVlZHwyfHx8ZW58MHx8fHx8",
    ];

    create_dir_all("./downloaded_files").await.expect("Unable to create the folder");

    let client = reqwest::Client::new();

     let tasks:Vec<_> = urls.iter().map(|&url| {
        let client = client.clone();
            async move {
                 match client.get(url).send().await {

                    Ok(resp)=>{
                        if let Some(content_type) = resp.headers().get(CONTENT_TYPE) {
                            let image_type = content_type.to_str().ok().and_then(|st| st.split("/").nth(1)).unwrap_or("bin");
                            let current_datetime = Local::now();
                            let mut file = File::create(format!("./downloaded_files/{current_datetime}.{image_type}")).await.expect("Unable to create the file");
                            let bytes = resp.bytes().await.unwrap();
                            file.write_all(&bytes).await.expect("Failed to write the file at this mooment");
                        }
                    }
                    Err(err)=>{
                        println!("Error occurred in this file path {} , Err: {}",url,err);
                    }
                };
            }
     } ).collect();

    join_all(tasks).await;
  
}
