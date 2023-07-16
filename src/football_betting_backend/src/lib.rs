use ic_cdk::api::management_canister::http_request::{
    http_request, 
    CanisterHttpRequestArgument, 
    HttpHeader, 
    HttpMethod, 
    // HttpResponse, 
    // TransformArgs,
    // TransformContext,
};

#[ic_cdk::query]
fn greet_me(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
async fn get_available_games() -> String {
    let host = String::from("bettingraja-go.onrender.com");

    const PORT: u32 = 443;
    const ROUTE: &str = "/games";

    let request_headers = vec![
        HttpHeader {
            name: String::from("Host"),
            value: format!("{}:{}", host, PORT, ),
        },
        HttpHeader {
            name: String::from("Content-Type"),
            value: String::from("application/json"),
        },
        HttpHeader {
            name: String::from("Accept"),
            value: String::from("application/json"),
        },
        HttpHeader {
            name: String::from("User-Agent"),
            value: String::from("betting_raja"),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: format!("https://{}{}", host, ROUTE),
        max_response_bytes: None,
        method: HttpMethod::GET,
        headers: request_headers,
        body: None,
        transform: None,
    };

    match http_request(request).await {
        Ok((response,)) => {
            let response_body =
                String::from_utf8(response.body).expect("response was not valid utf-8");
            response_body
        }
        Err((r, m)) => {
            let message = format!(
                "The http_request resulted into error. RejectionCode: {:?}, Message: {:?}",
                r, m
            );
            message
        }
    }
}

#[ic_cdk::update]
async fn send_bet() -> String {
    let host = String::from("bettingraja-go.onrender.com");

    const PORT: u32 = 443;
    const ROUTE: &str = "/games";

    let request_headers = vec![
        HttpHeader {
            name: String::from("Host"),
            value: format!("{}:{}", host, PORT),
        },
        HttpHeader {
            name: String::from("Content-Type"),
            value: String::from("application/json"),
        },
        HttpHeader {
            name: String::from("User-Agent"),
            value: String::from("betting_raja"),
        },
        HttpHeader {
            name: String::from("Idempotency-Key"),
            value: String::from("UUID-1234567890"),
        }
    ];
    let json_string: String = r#"{
        "name": "Raja",
        "force_sensitive": "true",
        "language": "rust"
    }"#.to_owned();
    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: format!("https://{}{}", host, ROUTE, ),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: None,
    };

    match http_request(request).await {
        Ok((response, )) => {
            let response_body =
                String::from_utf8(response.body).expect("response was not valid utf-8");
            response_body
        }
        Err((r, m, )) => {
            let message = format!(
                "The http_request resulted into error. RejectionCode: {:?}, Message: {:?}",
                r, m
            );
            message
        }
    }
}
