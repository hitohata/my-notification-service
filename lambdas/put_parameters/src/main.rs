use tokio::sync::OnceCell;
use std::collections::VecDeque;
use std::env::var;
use std::sync::Arc;
use tokio::sync::RwLock;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_ssm::types::{Parameter, ParameterType};
use serde_json::Value;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use futures::future::join_all;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(_event: LambdaEvent<Value>) -> Result<(), Error> {
    // Extract some useful information from the request

    let target_region = var("TARGET_REGIONS").unwrap();
    let origin_region = var("ORIGIN").unwrap();
    let parameter_name = var("PARAMETER_NAME").unwrap();

    let target_region_vector = env_str_to_vector_string(&target_region);

    let put_parameter_region = target_region_vector.iter().filter(|r| *r != &origin_region).to_owned().collect::<Vec<&String>>();

    let parameter_output = ssm_client()
        .await
        .get_parameter()
        .name(parameter_name)
        .send()
        .await
        .expect("Failed to get parameter");

    let parameter = parameter_output.parameter.expect("Parameter not found");
    let arc_parameter = Arc::new(RwLock::new(parameter));

    let put_parameters = put_parameter_region.iter().map(move |region| {
        put_parameter(arc_parameter.clone(), region)
    });

    join_all(put_parameters).await;

    Ok(())
}


async fn put_parameter(parameter: Arc<RwLock<Parameter>>, region: &str) {

    println!("put_parameter: {:?}", parameter);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region.to_owned()))
        .load()
        .await;

    let client = aws_sdk_ssm::Client::new(&config);

    let param = parameter.read().await;

    println!("put_parameter: {:?}", param);

    println!("name: {}", &param.to_owned().name.expect("name is not found").to_string());
    println!("value: {}", &param.to_owned().value.expect("value is not found").to_string());

    client
        .put_parameter()
        .name(&param.to_owned().name.expect("name is not found").to_string())
        .value(&param.to_owned().value.expect("value is not found").to_string())
        .r#type(ParameterType::String)
        .overwrite(true)
        .send()
        .await
        .expect("Failed to put parameter");
}


/// Change str from the environment value to a string vector.
///
/// ```rust
/// let expected_string = "[\"ar-north-1\", \"hv-south-2\"]";
/// let result = env_str_to_vector_string(expected_string);
/// assert_eq!(result, vec!["ar-north-1".to_string(), "hv-south-2".to_string()])
/// ```
fn env_str_to_vector_string(s: &str) -> Vec<String> {
    let mut stack = VecDeque::new();
    let mut result = Vec::new();
    let mut is_inside_str = false;

    for c in s.chars() {
        match c {
            '"' => {
                if is_inside_str && !stack.is_empty() {
                    let mut word = String::new();
                    while let Some(char) = stack.pop_front() {
                        word.push(char);
                    }
                    result.push(word.chars().collect());
                    is_inside_str = false;
                } else {
                    is_inside_str = true;
                }
            }
            _ => {
                if is_inside_str {
                    stack.push_back(c);
                }
            }
        }
    }

    result
}


static SSM_CLIENT: OnceCell<aws_sdk_ssm::Client> = OnceCell::const_new();

async fn ssm_client() -> &'static aws_sdk_ssm::Client {
    SSM_CLIENT.get_or_init(|| async {
        let config = aws_config::load_from_env().await;
        aws_sdk_ssm::Client::new(&config)
    }).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_env_str_to_vector_string() {
        // Arrange
        let expected_string = "[\"ar-north-1\", \"hv-south-2\"]";

        // Act
        let result = env_str_to_vector_string(expected_string);

        // Assert
        assert_eq!(result, vec!["ar-north-1".to_string(), "hv-south-2".to_string()])
    }
}
