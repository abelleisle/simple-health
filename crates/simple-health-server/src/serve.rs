use axum::{
    Router,
    extract::{Extension, Query, State},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use std::collections::HashMap;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

use crate::auth::{authenticate::signout, required_auth};
use crate::core::types::Meal;
use crate::{ServerState, UserContext};

#[derive(Deserialize)]
struct LoginQuery {
    error: Option<String>,
}

pub fn get_routes(state: ServerState) -> Router<ServerState> {
    let mut tera = Tera::new("frontend/web/templates/**/*").expect("Failed to initialize Tera");

    tera.autoescape_on(vec!["html", "htm"]);

    Router::new()
        .route("/", get(dashboard))
        .layer(middleware::from_fn_with_state(state, required_auth))
        .route("/login", get(login))
        .route("/signout", post(signout))
        .nest_service("/static/css", ServeDir::new("frontend/web/static/css"))
        .nest_service("/static/js", ServeDir::new("frontend/web/static/js"))
        .nest_service(
            "/static/assets",
            ServeDir::new("frontend/web/static/assets"),
        )
        .layer(Extension(tera))
}

async fn dashboard(
    State(state): State<ServerState>,
    Extension(ctx): Extension<UserContext>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();

    let goal: i32 = 2000;

    // Add dummy user data
    let mut user = HashMap::new();
    user.insert("name", ctx.user.as_ref().unwrap().name.to_string());
    user.insert("calorie_goal", goal.to_string());
    context.insert("user", &user);

    // let entries: Vec<HashMap<String, String>> = Vec::new();
    let five_days_ago_midnight = (Utc::now() - Duration::days(5))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    let meals = Meal::fetch_between_dates(
        ctx.user.unwrap().id,
        five_days_ago_midnight,
        None,
        state.db.get_pool(),
    )
    .await
    .map_err(|e| {
        log::error!(
            "Unable to fetch meals between {} and now: {}",
            five_days_ago_midnight,
            e
        );
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;

    let entries: Vec<_> = meals
        .iter()
        .map(|m| {
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("name", m.description.clone());
            map.insert("type", m.name.clone());
            map.insert("time", m.created_at.to_string());
            map.insert("calories", m.calories.to_string());

            map
        })
        .collect();

    log::debug!("Entries: {:?}", entries);

    let total_cal: i32 = meals.iter().map(|m| m.calories).sum();
    let reman_cal: i32 = goal - total_cal;
    let percent: i32 = (100 * total_cal) / goal;

    // Add dummy stats using serde_json for proper serialization
    context.insert(
        "stats",
        &serde_json::json!({
            "total_calories": total_cal,
            "remaining_calories": reman_cal,
            "progress_percentage": percent,
            "meal_breakdown": {
                "breakfast": meals .iter() .filter(|m| m.name == "Breakfast") .map(|m| m.calories) .sum::<i32>(),
                "lunch": meals .iter() .filter(|m| m.name == "Lunch") .map(|m| m.calories) .sum::<i32>(),
                "dinner": meals .iter() .filter(|m| m.name == "Dinner") .map(|m| m.calories) .sum::<i32>(),
                "snack": meals .iter() .filter(|m| m.name == "Snack") .map(|m| m.calories) .sum::<i32>(),
                "coffee": meals .iter() .filter(|m| m.name == "Coffee") .map(|m| m.calories) .sum::<i32>(),
            },
            // "entries": []
            "entries": entries,//[
                // {
                //     "name": "Oatmeal with banana",
                //     "type": "breakfast",
                //     "time": "8:30 AM",
                //     "calories": "350"
                // },
                // {
                //     "name": "Chicken salad",
                //     "type": "lunch",
                //     "time": "12:45 PM",
                //     "calories": "480"
                // }
            // ]
        }),
    );

    // Add current date
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    context.insert("selected_date", &current_date);

    // Add health status with database connection check
    let db_connected = state.db.is_connected();
    context.insert(
        "health",
        &serde_json::json!({
            "backend_healthy": true,
            "database_connected": db_connected,
            "message": if db_connected { "Backend Healthy" } else { "Backend Issues" }
        }),
    );

    let rendered = tera.render("dashboard.html.tera", &context).map_err(|e| {
        log::error!("Error rendering dashboard: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}

async fn login(
    State(_state): State<ServerState>,
    Query(query): Query<LoginQuery>,
    Extension(tera): Extension<Tera>,
    Extension(ctx): Extension<UserContext>,
) -> impl IntoResponse {
    let mut context = Context::new();

    if ctx.user.is_some() {
        return Redirect::to("/").into_response();
    }

    if let Some(error) = query.error {
        context.insert("error", &error);
    }

    // You can add error handling and username persistence here
    // context.insert("error", "Invalid credentials");
    // context.insert("username", "user@example.com");

    let rendered = tera.render("login.html.tera", &context).map_err(|e| {
        log::error!("Error rendering login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    Html(rendered).into_response()
}
