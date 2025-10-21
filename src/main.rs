use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::Deserialize;
use reqwest::Client;
use std::env;

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
}

#[post("/search")]
async fn search(req: web::Json<SearchRequest>) -> impl Responder {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let baidu_key = env::var("BAIDU_API_KEY").unwrap_or_else(|_| "your_api_key_here".to_string());
    let url = "https://qianfan.baidubce.com/v2/ai_search/web_search";

    let body = serde_json::json!({
        "messages": [
            {"content": format!("你是一个中国境内景点图片搜索引擎，返回景点{}的图片URL。", req.query),
             "role": "user"}
        ],
        "search_source": "baidu_search_v2",
        "resource_type_filter": [{"type": "image", "top_k": 5}],
        "search_recency_filter": "year"
    });

    // ✅ 打印 curl 命令方便调试
    let curl_cmd = format!(
        "curl --location '{}' \\\n  --header 'X-Appbuilder-Authorization: Bearer {}' \\\n  --header 'Content-Type: application/json' \\\n  --data '{}'",
        url,
        baidu_key,
        body.to_string().replace("'", "\\'")
    );
    // println!("\n==== CURL COMMAND ====\n{}\n====================\n", curl_cmd);

    // println!("发送请求中: {}", req.query);

    // ✅ 发起请求
    let resp = client
        .post(url)
        .header("X-Appbuilder-Authorization", format!("Bearer {}", baidu_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    // ✅ 打印响应或错误
    match resp {
        Ok(r) => {
            let status = r.status();
            let text = r.text().await.unwrap_or_default();
            // println!("响应状态: {}", status);
            // println!("响应内容前200字: {}", &text.chars().take(200).collect::<String>());
            HttpResponse::Ok().body(text)
        }
        Err(e) => {
            println!("❌ 请求失败: {:?}", e);
            HttpResponse::InternalServerError().body(format!("error: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    let env_file = if cfg!(debug_assertions) {
        ".env.debug"
    } else {
        ".env.release"
    };
    dotenv::from_filename(env_file).ok();

    println!("Server running at http://0.0.0.0:8181");

    HttpServer::new(|| {
        let cors: Cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();

        App::new().wrap(cors).service(search)
    })
    .bind(("0.0.0.0", 8181))?
    .run()
    .await
}
