use axum::{routing::get, Router};
use mongodb::{bson::{doc, Document}, Cursor};


#[tokio::main]
async fn main() {
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    let db = mongodb::Client::with_uri_str("mongodb://localhost:27017").await.unwrap().database("aws-sample-app");
    
    let app = Router::new().route("/", get(|| async move {
	
	println!("got /");

        db.collection("mycol").insert_one(doc! {
     	    "abc": "def"
        }).await.unwrap();
        
        println!("inserted");
	let mut cursor: Cursor<Document> = db.collection("mycol").find(doc!{}).await.unwrap();
        let mut resp: Vec<String> = vec!["start".to_owned()];
        println!("started query");
        while cursor.advance().await.unwrap() {
            let val = cursor.current().get_str("abc").unwrap().to_owned();
            println!("{:?}", resp);
            resp.push(val);
        }


	println!("sending resp");        
        resp.join(",")
    
    }));
    
    axum::serve(listener, app).await.unwrap();
}
