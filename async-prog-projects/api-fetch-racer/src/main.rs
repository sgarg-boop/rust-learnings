use tokio::time::{Instant};

async fn fetch_api(api_name: &str, url:&str) -> Result<String, Box<dyn std::error::Error>>{
    println!("Starting request to: {}", api_name);

    let start = Instant::now();


    // make http request
    let response = reqwest::get(url).await?;

    // check if successfull
    if !response.status().is_success(){
        return Err(format!("Failed to fetch API:{} with error code: {}", api_name, response.status()).into());
    }

    // get response body
    let body = response.text().await?;
    let time_taken = start.elapsed();

    println!("Finished fetching: {} in {:?}", api_name, time_taken);

    Ok(body)
}

// using select - first to finish wins
#[tokio::main]
async fn main(){

    // create futures
    let api1 = fetch_api("JSONPlaceholder", "https://jsonplaceholder.typicode.com/posts/1");
    let api2 = fetch_api("GitHub", "https://api.github.com/users/github");
    let api3 = fetch_api("HTTPBin", "https://httpbin.org/delay/2");

    // pin them so select can poll them
    tokio::pin!(api1, api2, api3);

    // Race them!
    // it needs to poll many times, so we need to pin them
    tokio::select!{

        // check result1
        // we need to use mut ref because select! takes ownership of the future
        // future has many states and it is chnaged by select!
        // each time select! polls the future, it checks the state and based on that it decides which branch to take

        // enum for state is
        // enum State {
        // a virtual concept : not written by us, macro + compiler knows these things
        //}


        result1 = &mut api1 =>{
            match result1{
                Ok(data) => println!("🏆 JSONPlaceholder WON! Data: {} bytes", data.len()),
                Err(e) => eprintln!("❌ JSONPlaceholder failed: {}", e),
            }
        }

        // check result2
        result2 = &mut api2 =>{
            match result2{
                Ok(data) => println!("🏆 GitHub WON! Data: {} bytes", data.len()),
                Err(e) => eprintln!("❌ GitHub failed: {}", e),
            }
        }

        // check result3
        result3 = &mut api3 =>{
            match result3{
                Ok(data) => println!("🏆 HTTPBin WON! Data: {} bytes", data.len()),
                Err(e) => eprintln!("❌ HTTPBin failed: {}", e),
            }
        }
    }

    println!("All tasks completed");

}