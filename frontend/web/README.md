# Simple Health Frontend

Static HTML templates with minimal JavaScript, designed to be served by Axum with Tera templating.

## Structure

```
templates/
├── base.html.tera          # Base layout template
├── dashboard.html.tera     # Main dashboard page
└── login.html.tera         # Login page

static/
├── css/
│   └── styles.css         # Tailwind CSS with custom styles
├── js/
│   ├── dashboard.js       # Dashboard interactions (modal, date picker)
│   └── login.js          # Login form handling
└── assets/
    └── (static assets like icons, images)
```

## Key Changes from TypeScript SPA

1. **No Build Process**: Raw HTML templates served directly by Axum
2. **Minimal JavaScript**: Only essential DOM interactions and form handling
3. **Server-Side Rendering**: All data comes from Tera template context
4. **Form Submissions**: Standard HTML forms posting to server endpoints
5. **No State Management**: State is managed server-side

## Tera Template Context

### Dashboard Template (`dashboard.html.tera`)

Expected context variables:

```rust
{
    "user": {
        "name": "User Name",
        "calorie_goal": 2000
    },
    "selected_date": "2025-01-01",
    "stats": {
        "total_calories": 1245,
        "remaining_calories": 755,
        "progress_percentage": 62.25,
        "meal_breakdown": {
            "breakfast": 320,
            "lunch": 485,
            "dinner": 440,
            "snack": 0
        },
        "entries": [
            {
                "name": "Oatmeal",
                "calories": 320,
                "type": "breakfast",
                "time": "08:00"
            }
        ]
    },
    "health": {
        "backend_healthy": true,
        "database_connected": true,
        "message": "Backend Healthy"
    }
}
```

### Login Template (`login.html.tera`)

Expected context variables:

```rust
{
    "error": "Optional error message",
    "username": "Optional username to pre-fill"
}
```

## Required Server Endpoints

- `GET /` - Dashboard page
- `GET /login` - Login page
- `POST /login` - Handle login form
- `POST /logout` - Handle logout
- `POST /add-food` - Add food entry
- `GET /static/*` - Serve static assets

## CSS Processing

The Tailwind CSS can be processed either:

1. At build time with Tailwind CLI
2. At runtime by Axum (if you set up Tailwind processing)
3. Use a CDN version for development

For production, pre-compile the CSS:

```bash
npx tailwindcss -i static/css/styles.css -o static/css/compiled.css --watch
```
