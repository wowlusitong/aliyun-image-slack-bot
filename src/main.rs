#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate reqwest;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::{Json, Value};

#[derive(Serialize, Deserialize)]
struct PushData {
    digest: String,
    pushed_at: String,
    tag: String,
}
#[derive(Serialize, Deserialize)]
struct Repository {
    date_created: String,
    name: String,
    namespace: String,
    region: String,
    repo_authentication_type: String,
    repo_full_name: String,
    repo_origin_type: String,
    repo_type: String,
}
#[derive(Serialize, Deserialize)]
struct Image {
    push_data: PushData,
    repository: Repository,
}

#[post("/", format = "application/json", data = "<image>")]
fn index(image: Json<Image>) -> Json<Value> {
    let tag = image.push_data.tag.to_string();
    let client = reqwest::Client::new();
    let payload = json! ({
        "attachments": [
            {
                "color": "good",
                "pretext": "Docker image build success",
                "title": image.repository.repo_full_name.to_string() + ":" + &tag
            }
        ]
    });
    let res = client
        .post("SLACK-WEBHOOK-URL")
        .json(&payload)
        .send();
    Json(json! ({ "status": "ok" }))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
