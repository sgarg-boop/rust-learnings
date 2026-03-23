use tokio::time::{Duration, sleep, Instant};


async fn fetch_url(url: &str) -> Result<String, reqwest::Error>{
    println!(" Starting to fetch: {}", url);

    // // simulate network delay
    // sleep(Duration::from_secs(delay_secs)).await;

    // println!(" finished fetching: {}", url);
    // format!("Data from {}", url)


    //////// make real http request //////
    let response = reqwest::get(url).await?;

    // get html/text content
    let body = response.text().await?;

    // get first 100 chars
    let preview = &body[..100.min(body.len())];

    println!("Finished fetching: {} ({:?} )", url, preview);
    Ok(body) // Result return
}

#[tokio::main]
async fn main(){

    let start = Instant::now();

    // // Sequential slow -> takes 6 seconds
    // println!("\n=== SEQUENTIAL FETCHING ===");
    // fetch_url("example.com", 2).await;
    // fetch_url("rust-lang.org", 2).await;
    // fetch_url("github.com", 2).await;


    // // start.elapsed means time taken from start to end
    // println!("Sequential fetching took: {:?}", start.elapsed());

    // // concurrent fetching
    // let start = Instant::now();
    // println!("\n=== CONCURRENT FETCHING ===");

    // let task1 = fetch_url("example.com", 2);
    // let task2 = fetch_url("rust-lang.org", 2);
    // let task3 = fetch_url("github.com",2);


    // real url passing
    let (r1,r2,r3) = tokio::join!{
        fetch_url("https://example.com"),
        fetch_url("https://www.rust-lang.org"),
        fetch_url("https://github.com")
    };
    // run all 3 at a time
    // tokio::join!(task1, task2, task3);

    println!("\n Total time: {:?}", start.elapsed());


}