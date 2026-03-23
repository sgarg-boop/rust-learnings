use tokio::fs::File;
use tokio::io::AsyncWriteExt;  // write entire buffer
use std::path::Path;



async fn download_image(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>>{

    println!("Downloading file: {}", filename);

    // check if file already exists
    let path = format!("downloads/{}", filename);
    if Path::new(&path).exists(){
        println!("File already exists: skipping.... {}", path);
        return Ok(());
    }

    // fetch the file from internet
    let response = reqwest::get(url).await?;

    // check status
    if!response.status().is_success(){
        return Err(format!("Failed to download image: {}", response.status()).into());
    }


    let bytes = response.bytes().await?;


    // save to download folder
    // create a space with this file name under downloads
    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    println!("File saved Success {}", path);
    Ok(())

}

#[tokio::main]
async fn main(){

    // create download folder if not exists
    if let Err(e) = tokio::fs::create_dir_all("downloads").await {
        eprintln!("Failed to create downloads folder: {}", e);
        return;
    }

    // download some stuff
    let (res1,res2) = tokio::join!(
        download_image("https://images.pexels.com/photos/1704488/pexels-photo-1704488.jpeg", "image1.jpg"),
        download_image("https://images.pexels.com/photos/1111597/pexels-photo-1111597.jpeg", "image2.jpg")
    );

    if let Err(e) = res1{
        eprintln!("Error downloading image1: {}", e);
    }

    if let Err(e) = res2{
        eprintln!("Error downloading image2: {}", e);
    }

    println!("All tasks completed");
}
