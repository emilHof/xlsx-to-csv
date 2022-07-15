mod converter;

use aws_config::meta::region::RegionProviderChain;
use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::{Client, Region};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    let s3_event = event.payload.records;

    println!("{:?}", s3_event);

    let records = s3_event.last().cloned().unwrap();
    let region = records.aws_region.unwrap();
    let bucket = records.s3.bucket.name.unwrap();
    let key = records.s3.object.key.unwrap();

    println!("region: {}, bucket: {}, key: {}", region, bucket, key);

    let region_provider
        = RegionProviderChain::first_try(Region::new(region.clone()))
        .or_default_provider();

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp
        = download_object(&client, bucket.clone().as_str(), key.clone().as_str())
        .await?;

    Ok(resp)
}

async fn download_object(client: &Client, bucket_name: &str, key: &str) -> Result<(), Error> {
    let resp = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;
    let data = resp.body.collect().await;
    println!(
        "Data from downloaded object: {:?}",
        data.unwrap().into_bytes().slice(0..20)
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
