use std::collections::HashMap;

use reqwest::Client;
use anyhow::Result;

use crate::kv::KvPair;

pub async fn get(url: &str) -> Result<()> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    Ok(print::response(res).await?)
}

pub async fn post(url: &str, args: &Vec<KvPair>) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args {
        body.insert(&pair.key, &pair.value);
    }
    let client = Client::new();
    let res = client.post(url).json(&body).send().await?;
    Ok(print::response(res).await?)
}

mod print{
    use mime::Mime;
    use colored::*;

    pub async fn response(res: reqwest::Response) -> anyhow::Result<()> {
        status(&res);
        headers(&res);
        let mime = content_type(&res);
        let body = res.text().await?;
        print_body(mime, body.as_str());
        Ok(())
    }

    fn status(res: &reqwest::Response) {
        let status = format!("{:?} {}", res.version(), res.status()).blue();
        println!("{}\n", status);
    }

    fn headers(res: &reqwest::Response) {
        for (key, value) in res.headers() {
            println!("{}: {}", key.to_string().green(), value.to_str().unwrap());
        }
        println!("")
    }

    fn print_body(mime: Option<Mime>, body: &str) {
        match mime {
            Some(mime) if mime == mime::APPLICATION_JSON => {
                println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
            }
            _ => println!("{}", body),
        }
    }
    fn content_type(res: &reqwest::Response) -> Option<Mime>{
        res.headers().get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .and_then(|ct| ct.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    
    #[tokio::test]
    async fn test_get() -> Result<(), Box<dyn Error>> {
        // Arrange
        let url = "http://httpbin.org/get";
    
        // Act
        let result = get(url).await;
    
        // Assert
        assert!(result.is_ok());
        Ok(())
    }
    
    #[tokio::test]
    async fn test_post() -> Result<(), Box<dyn Error>> {
        // Arrange
        let url = "http://httpbin.org/post";
        let args = vec![
            KvPair { key: "foo".to_owned(), value: "bar".to_owned() },
            KvPair { key: "baz".to_owned(), value: "qux".to_owned() },
        ];
    
        // Act
        let result = post(url, &args).await;
    
        // Assert
        assert!(result.is_ok());
        Ok(())
    }
    
}