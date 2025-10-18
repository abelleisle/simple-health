use axum::{
    Router,
    extract::{Extension, Query, State},
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use chrono::{Local, NaiveDate, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use serde::Deserialize;
use std::collections::HashMap;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

use crate::auth::{authenticate::signout, required_auth};
use crate::core::types::{Activity, Goal, Meal, UserSetting};
use crate::utils;
use crate::{ServerState, UserContext};

#[derive(Deserialize)]
struct LoginQuery {
    error: Option<String>,
}

#[derive(Deserialize)]
struct DashboardQuery {
    date: Option<String>,
}

pub fn get_routes(state: ServerState) -> Router<ServerState> {
    // Get dynamic paths for templates and static files
    let templates_dir = utils::get_templates_dir();
    let static_dir = utils::get_static_files_dir();

    // Construct Tera template pattern
    let template_pattern = templates_dir
        .join("**/*")
        .to_str()
        .expect("Failed to construct template pattern")
        .to_string();

    let mut tera = Tera::new(&template_pattern).expect("Failed to initialize Tera");

    tera.autoescape_on(vec!["html", "htm"]);

    Router::new()
        .route("/", get(dashboard))
        .route("/profile", get(profile))
        .layer(middleware::from_fn_with_state(state, required_auth))
        .route("/login", get(login))
        .route("/signup", get(signup))
        .route("/signout", post(signout))
        .nest_service("/static/css", ServeDir::new(static_dir.join("css")))
        .nest_service("/static/js", ServeDir::new(static_dir.join("js")))
        .nest_service("/static/assets", ServeDir::new(static_dir.join("assets")))
        .layer(Extension(tera))
}

async fn dashboard(
    State(state): State<ServerState>,
    Extension(ctx): Extension<UserContext>,
    Extension(tera): Extension<Tera>,
    Query(query): Query<DashboardQuery>,
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();

    // Get user's timezone, fallback to UTC if invalid
    let user_timezone: Tz = ctx.settings.timezone.parse().unwrap_or_else(|e| {
        log::error!(
            "Unable to parse user's timezone ({}): {}",
            ctx.settings.timezone,
            e
        );
        chrono_tz::UTC
    });

    // Parse the selected date and convert to UTC date range
    let current_date_utc = Utc::now();
    let current_date_tz = current_date_utc.with_timezone(&user_timezone);
    let current_date_tz_str = current_date_tz.format("%Y-%m-%d").to_string();

    // Create selected_date versions
    let selected_date_tz_str = query.date.unwrap_or_else(|| current_date_tz_str.clone());
    let selected_naive_date = NaiveDate::parse_from_str(&selected_date_tz_str, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    // let selected_date_tz = user_timezone
    //     .from_local_datetime(&selected_naive_date.and_hms_opt(23, 59, 59).unwrap())
    //     .single()
    //     .ok_or(StatusCode::BAD_REQUEST)?;
    // let selected_date_utc = selected_date_tz.with_timezone(&Utc);

    // log::info!("Current date: {}", current_date_tz);
    // log::info!("Selected date: {}", selected_date_tz);
    // log::info!("Current date (UTC): {}", current_date_utc);
    // log::info!("Selected date (UTC): {}", selected_date_utc);

    // Create start and end times for the selected date in user's timezone
    let start_of_day_local = user_timezone
        .from_local_datetime(&selected_naive_date.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .ok_or(StatusCode::BAD_REQUEST)?;
    let end_of_day_local = user_timezone
        .from_local_datetime(&selected_naive_date.and_hms_opt(23, 59, 59).unwrap())
        .single()
        .ok_or(StatusCode::BAD_REQUEST)?;

    // log::info!("Start local: {}", start_of_day_local);
    // log::info!("End local: {}", end_of_day_local);

    // Convert to UTC for database queries
    let start_of_day_utc = start_of_day_local.with_timezone(&Utc);
    let end_of_day_utc = end_of_day_local.with_timezone(&Utc);

    // log::info!("Start UTC: {}", start_of_day_utc);
    // log::info!("End UTC: {}", end_of_day_utc);

    // Get the latest goal given the current date
    let goal = Goal::latest(
        state.db.get_pool(),
        &ctx.user.as_ref().unwrap(),
        end_of_day_utc,
    )
    .await
    .map_err(|e| {
        log::error!(
            "Unable to fetch latest goal for user {}: {}",
            ctx.user.as_ref().unwrap().id,
            e
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let meals = Meal::fetch_between_dates(
        &ctx.user.as_ref().unwrap(),
        start_of_day_utc,
        Some(end_of_day_utc),
        state.db.get_pool(),
    )
    .await
    .map_err(|e| {
        log::error!(
            "Unable to fetch meals for date {} (UTC: {} to {}): {}",
            selected_date_tz_str,
            start_of_day_utc,
            end_of_day_utc,
            e
        );
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;

    let activities = Activity::fetch_between_dates(
        &ctx.user.as_ref().unwrap(),
        start_of_day_utc,
        Some(end_of_day_utc),
        state.db.get_pool(),
    )
    .await
    .map_err(|e| {
        log::error!(
            "Unable to fetch activities for date {} (UTC: {} to {}): {}",
            selected_date_tz_str,
            start_of_day_utc,
            end_of_day_utc,
            e
        );
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;

    let entries: Vec<_> = meals
        .iter()
        .map(|m| {
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("id", m.id.to_string());
            map.insert("name", m.description.clone());
            map.insert("type", m.name.clone());
            // Convert UTC timestamp to user's timezone and format as time only
            let local_time = m.created_at.with_timezone(&user_timezone);
            map.insert("time", local_time.format("%H:%M").to_string());
            map.insert("calories", m.calories.to_string());

            map
        })
        .collect();

    let activity_entries: Vec<_> = activities
        .iter()
        .map(|a| {
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("id", a.id.to_string());
            map.insert("name", a.description.clone());
            map.insert("type", a.name.clone());
            // Convert UTC timestamp to user's timezone and format as time only
            let local_time = a.created_at.with_timezone(&user_timezone);
            map.insert("time", local_time.format("%H:%M").to_string());
            map.insert("calories", a.calories.to_string());

            // Format duration if present
            if let Some(duration_s) = a.duration_s {
                let minutes = duration_s / 60;
                let seconds = duration_s % 60;
                let duration_str = if minutes > 0 {
                    if seconds > 0 {
                        format!("{}m {}s", minutes, seconds)
                    } else {
                        format!("{}m", minutes)
                    }
                } else {
                    format!("{}s", seconds)
                };
                map.insert("duration", duration_str);
            }

            map
        })
        .collect();

    let mut user = HashMap::new();
    user.insert("name", ctx.user.as_ref().unwrap().name.to_string());
    user.insert("calorie_goal", goal.get_consumed().to_string());
    context.insert("user", &user);

    log::debug!("Entries: {:?}", entries);
    log::debug!("Activity entries: {:?}", activity_entries);

    let consumed_cal: i32 = meals.iter().map(|m| m.calories).sum();
    let burned_cal: i32 = activities.iter().map(|a| a.calories).sum();
    let net_cal: i32 = consumed_cal - burned_cal;
    let reman_cal: i32 = goal.get_consumed() - net_cal;
    let percent: i32 = (100 * net_cal) / goal.get_consumed();

    // Add dummy stats using serde_json for proper serialization
    context.insert(
        "stats",
        &serde_json::json!({
            "total_calories": net_cal,
            "consumed_calories": consumed_cal,
            "burned_calories": burned_cal,
            "remaining_calories": reman_cal,
            "progress_percentage": percent,
            "progress_bar": percent.clamp(0, 100),
            "meal_breakdown": {
                "breakfast": meals .iter() .filter(|m| m.name == "Breakfast") .map(|m| m.calories) .sum::<i32>(),
                "lunch": meals .iter() .filter(|m| m.name == "Lunch") .map(|m| m.calories) .sum::<i32>(),
                "dinner": meals .iter() .filter(|m| m.name == "Dinner") .map(|m| m.calories) .sum::<i32>(),
                "snack": meals .iter() .filter(|m| m.name == "Snack") .map(|m| m.calories) .sum::<i32>(),
                "coffee": meals .iter() .filter(|m| m.name == "Coffee") .map(|m| m.calories) .sum::<i32>(),
            },
            "activity_breakdown": {
                "walk": activities .iter() .filter(|a| a.name == "Walk") .map(|a| a.calories) .sum::<i32>(),
                "run": activities .iter() .filter(|a| a.name == "Run") .map(|a| a.calories) .sum::<i32>(),
                "hike": activities .iter() .filter(|a| a.name == "Hike") .map(|a| a.calories) .sum::<i32>(),
                "bike": activities .iter() .filter(|a| a.name == "Bike") .map(|a| a.calories) .sum::<i32>(),
                "ski": activities .iter() .filter(|a| a.name == "Ski") .map(|a| a.calories) .sum::<i32>(),
            },
            "entries": entries,
            "activities": activity_entries
        }),
    );

    // Add dates to context
    context.insert("selected_date", &selected_date_tz_str);
    context.insert("current_date", &current_date_tz_str);

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
    State(state): State<ServerState>,
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

    context.insert(
        "settings",
        &serde_json::json!( {
            "signup_allowed": state.is_signup_allowed()
        }),
    );

    let rendered = tera.render("login.html.tera", &context).map_err(|e| {
        log::error!("Error rendering login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    Html(rendered).into_response()
}

async fn signup(
    State(state): State<ServerState>,
    Query(query): Query<LoginQuery>,
    Extension(tera): Extension<Tera>,
    Extension(ctx): Extension<UserContext>,
) -> impl IntoResponse {
    let mut context = Context::new();

    if !state.is_signup_allowed() {
        return Redirect::to("/login").into_response();
    }

    if ctx.user.is_some() {
        return Redirect::to("/").into_response();
    }

    if let Some(error) = query.error {
        context.insert("error", &error);
    }

    // You can add error handling and username persistence here
    // context.insert("error", "Invalid credentials");
    // context.insert("username", "user@example.com");

    let rendered = tera.render("signup.html.tera", &context).map_err(|e| {
        log::error!("Error rendering login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    });

    Html(rendered).into_response()
}

async fn profile(
    State(state): State<ServerState>,
    Extension(ctx): Extension<UserContext>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();

    let user = ctx.user.as_ref().unwrap();

    // Get user's current settings
    let settings = &ctx.settings;
    context.insert("settings", settings);

    // Get user's current goals
    let user_timezone: Tz = settings.timezone.parse().unwrap_or(chrono_tz::UTC);
    let current_time = Utc::now();

    let goals = Goal::latest(state.db.get_pool(), user, current_time)
        .await
        .map_err(|e| {
            log::error!("Unable to fetch latest goal for user {}: {}", user.id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Convert active_time_s to minutes for display
    let active_minutes = goals.active_time_s.map(|s| s / 60);

    context.insert(
        "goals",
        &serde_json::json!({
            "consumed": goals.calories_consumed,
            "burned": goals.calories_burned,
            "active_minutes": active_minutes
        }),
    );

    let rendered = tera.render("profile.html.tera", &context).map_err(|e| {
        log::error!("Error rendering profile: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}
