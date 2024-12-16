use aws_config;
use aws_sdk_s3::{config::Region, types, Client};
use dotenvy;

// IMPORTANT NOTE: An AWS account is required. A user with suitable permissions is also required (set up in IAM)
#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        // Load the .envlocal file (or .env by default) for local development
        // If deployed, use platform environment variables
        dotenvy::from_filename(".envlocal").expect(".env file not found");
    }

    let region = dotenvy::var("AWS_REGION").expect("AWS_REGION must be set in a .env file");
    // IMPORTANT NOTE: Check bucket name. Rust loves underscores, _, AWS not so much.
    // Bucket name must NOT contain underscores: https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-tables-buckets-naming.html
    let bucket_name = "orangekiwi-s3-example";

    // AWS looks for AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY environment variables
    // so no need to explicitly assign them to variables
    let bucket_config = aws_config::from_env()
        .region(Region::new(region.clone()))
        .load()
        .await;
    // New S3 client instance
    let s3_client = Client::new(&bucket_config);
    // Regional contraint for bucket creation
    let constraint = types::BucketLocationConstraint::from(region.to_string().as_str());
    // Configuration
    let cfg = types::CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();

    // Create bucket (quietly falls over if error since no error handling or <Result> work)
    let create = s3_client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await;

    // Print the create result
    println!("create: {:#?}", create);
}
