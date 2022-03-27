use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Project {
    id: u32,
    name: String,
    person_id: u32,
    pin: bool,
    organization: String,
}

pub async fn fake_projects() -> Json<Vec<Project>> {
    let mut projects = Vec::new();
    let p1 = Project {
        id: 1,
        name: "小华".into(),
        person_id: 1,
        pin: false,
        organization: "销售部".into(),
    };

    let p2 = Project {
        id: 2,
        name: "小红".into(),
        person_id: 2,
        pin: false,
        organization: "营销部".into(),
    };

    projects.push(p1);
    projects.push(p2);

    Json(projects)
}
