use std::str::FromStr;

use axum::{extract::State, Json};
use bson::{doc, oid::ObjectId, Document};
use hyper::StatusCode;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
  name: String,
  prompt: String,
  photo: String
}

#[derive(Serialize, Deserialize)]
pub struct PostId {
  id: ObjectId
}

pub async fn returns_views(State(client): State<Client>) ->(StatusCode, Json<Value>) {
    let db = client.database("test");
    let collection: Collection<Document> = db.collection("views");
    let mut return_value: Vec<Document> = vec![];
    let object_id = ObjectId::from_str("64a6b5b1a564cf55ff372e56");

    match object_id {
        Ok(oid) => {
          match collection.find(doc! {"_id": oid})
            .await {
              Ok(res) => {
                  match res.deserialize_current() {
                      Ok(value) => {
                          return_value.push(value);
                      },
                      Err(error) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to deserialize the document"})))
                  }
              },
              Err(e) => {
                  println!("Err getting view: {:?}", e);
                  return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({})))
              }
            }
        },
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))) 
    }

    (StatusCode::FOUND, Json(json!(return_value)))
}

pub async fn update_and_return(State(client): State<Client>) -> (StatusCode, Json<Value>) {
  let db: Collection<Document> = client.database("test").collection("views");
  let mut count = 0;

  match db.find_one_and_update(doc! {"_id": ObjectId::from_str("64a6b5b1a564cf55ff372e56").unwrap()}, doc! { "$inc": doc! {"views": 1}}).await {
        Ok(response) => {
            count = response.unwrap().get("views").unwrap().as_i32().expect("error reading count of views"); 
        },
        Err(error) => println!("error: {:?}", error)
    }

  count += 1;
   
  (StatusCode::ACCEPTED, Json(json!({"count": count})))
}

pub async fn get_posts(State(client): State<Client>) -> (StatusCode, Json<Value>) {
  let db = client.database("test");
  let collection: Collection<Document> = db.collection("posts");
  let mut posts: Vec<Document> = vec![];

  match collection.find(doc! {}).await {
      Ok(mut response) => {
        while response.advance().await.unwrap() {
          posts.push(response.deserialize_current().unwrap());
        }
      },
      Err(error) => println!("Error: {:?}", error)
  }

  (StatusCode::OK, Json(json!(posts)))
}

pub async fn add_post(State(client): State<Client>, Json(payload): Json<Post>) -> (StatusCode, Json<Value>) {
  let db: Collection<Post> = client.database("test").collection("posts");
  println!("payload: {:#?}", payload);
  let post = Post {
    name: payload.name,
    prompt: payload.prompt,
    photo: payload.photo
  };
  let mut message: &str = "";

  match db.insert_one(&post).await {
      Ok(reponse) => {
        println!("Response on GET api/posts {:#?}", reponse);
        message = "Added new post";
      },
      Err(error) => {
        println!("Error on GET api/posts {}", error);
        message = "Failed to add post";
      }
  }

  println!("Message: {}", message);
  (StatusCode::CREATED, Json(json!(post)))
}

pub async fn delete_post(State(client): State<Client>, Json(payload): Json<PostId>) -> (StatusCode, Json<Value>) {
  let db: Collection<Post> = client.database("test").collection("posts");

  match db.delete_one(doc! {"_id": ObjectId::from_str(&payload.id.to_string()).unwrap()}).await {
    Ok(response) => println!("Response: {:?}", response),
    Err(error) => println!("Error: {}", error)
  }

  (StatusCode::OK, Json(json!({"result": format!("{} {}", "Successfully deleted post with id: ", payload.id)})))
}