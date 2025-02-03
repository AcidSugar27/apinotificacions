use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_rt::spawn;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use reqwest::Client;
use gcp_auth::AuthenticationManager;
use dotenv::dotenv;
use std::env;

const FCM_URL: &str = "https://fcm.googleapis.com/v1/projects/frontandroid-ab001/messages:send";


#[derive(Debug, Serialize)]
struct FcmMessage {
    message: MessageData,
}

#[derive(Debug, Serialize)]
struct MessageData {
    token: String,
    notification: Notification,
}

#[derive(Debug, Serialize)]
struct Notification {
    title: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskRequest {
    task_id: String,
    user_token: String,
}

#[derive(Debug, Serialize)]
struct TaskStatus {
    task_id: String,
    status: String,
}

type TaskState = Arc<Mutex<HashMap<String, String>>>;


async fn send_fcm_notification(user_token: String, task_id: String, action: &str) {
    dotenv().ok();

    let auth_manager = AuthenticationManager::new().await.unwrap();
    let token = auth_manager
        .get_token(&["https://www.googleapis.com/auth/firebase.messaging"])
        .await
        .unwrap();

    let (title, body) = match action {
        "created" => (
            "Cliente Creado".to_string(),
            format!("El cliente con ID {} ha sido creado exitosamente.", task_id),
        ),
        "deleted" => (
            "Cliente Eliminado".to_string(),
            format!("El cliente con ID {} ha sido eliminado.", task_id),
        ),
        _ => (
            "Notificación".to_string(),
            format!("La tarea {} ha terminado de forma exitosa.", task_id),
        ),
    };

    let client = Client::new();
    let payload = FcmMessage {
        message: MessageData {
            token: user_token,
            notification: Notification { title, body },
        },
    };

    let response = client
        .post(FCM_URL)
        .bearer_auth(token.as_str())
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(resp) => println!("Notificación enviada: {:?}", resp.text().await.unwrap()),
        Err(err) => println!("Error en la notificación: {}", err),
    }
}

// Función para iniciar una tarea
async fn start_task(task_data: web::Json<TaskRequest>, state: web::Data<TaskState>) -> impl Responder {
    let task_id = task_data.task_id.clone();
    let user_token = task_data.user_token.clone();
    let state_clone = state.clone();

    state.lock().unwrap().insert(task_id.clone(), "En Progreso".to_string());

    spawn({
        let task_id = task_id.clone();
        async move {
            sleep(Duration::from_secs(5)).await;
            let mut tasks = state_clone.lock().unwrap();
            tasks.insert(task_id.clone(), "Completado".to_string());

            send_fcm_notification(user_token, task_id, "created").await; 
        }
    });

    HttpResponse::Accepted().json(TaskStatus {
        task_id,
        status: "En progreso".to_string(),
    })
}



async fn delete_task(task_data: web::Json<TaskRequest>, state: web::Data<TaskState>) -> impl Responder {
    let task_id = task_data.task_id.clone();
    let user_token = task_data.user_token.clone();
    let state_clone = state.clone();

    state.lock().unwrap().insert(task_id.clone(), "Eliminadose".to_string());

    spawn({
        let task_id = task_id.clone();
        async move {
            sleep(Duration::from_secs(5)).await;
            let mut tasks = state_clone.lock().unwrap();
            tasks.insert(task_id.clone(), "Eliminado".to_string());

            send_fcm_notification(user_token, task_id, "deleted").await; 
        }
    });

    HttpResponse::Accepted().json(TaskStatus {
        task_id,
        status: "Eliminado".to_string(),
    })
}



async fn get_task_status(task_id: web::Path<String>, state: web::Data<TaskState>) -> impl Responder {
    let task_id_value = task_id.into_inner();
    let tasks = state.lock().unwrap();
    if let Some(status) = tasks.get(&task_id_value) {
        HttpResponse::Ok().json(TaskStatus {
            task_id: task_id_value.clone(),
            status: status.clone(),
        })
    } else {
        HttpResponse::NotFound().body("tarea no encontrada")
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let task_state: TaskState = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_state.clone()))
            .route("/start_task", web::post().to(start_task))
            .route("/delete_task", web::post().to(delete_task)) 
            .route("/task_status/{task_id}", web::get().to(get_task_status))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}


