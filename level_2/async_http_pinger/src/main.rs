use std::time::Instant;
use futures::future::join_all;
use reqwest::Client;

#[tokio::main]
 async fn main(){
    
    let urls:Vec<&str> = vec![
        "https://www.google.com/",
        "https://www.instagram.com/",
        "https://www.linkedin.com/",
        "https://www.x.com/",
        "https://www.github.com/",
        "https://www.youtube.com/",
        "https://www.devng.tech/"
    ];

    let client = reqwest::Client::new();

    let tasks:Vec<_> = urls.iter().map(|&url| {
        let client = client.clone(); // futures captures ownership
        async move{
            let start = Instant::now();
            match client.get(url).send().await {
                Ok(resp)=>{
                    let elapsedTime = start.elapsed();
                    println!("{} -> {} ({:?})", url, resp.status(), elapsedTime);
                }
                Err(err)=>{
                    println!("Error occurred, url :{} , : {}",err,url);
                }
            };
        }

    }).collect();

        join_all(tasks).await;
    

}




