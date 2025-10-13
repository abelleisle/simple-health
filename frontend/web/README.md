# Simple Health Frontend

Static HTML templates with TypeScript, designed to be served by Axum with Tera templating.

## Structure

```
templates/
├── base.html.tera          # Base layout template
├── dashboard.html.tera     # Main dashboard page
├── login.html.tera         # Login page
├── signup.html.tera        # User registration page
└── profile.html.tera       # User settings and goals

static/
├── css/
│   └── styles.css         # Tailwind CSS v4.1 with custom styles
├── ts/
│   ├── dashboard.ts       # Dashboard interactions (modals, date picker)
│   ├── login.ts          # Login form handling
│   ├── signup.ts         # Signup form handling
│   ├── profile.ts        # Profile settings and goals management
│   ├── utils.ts          # Shared utilities (timezone, cookies, UUID)
│   └── types.ts          # TypeScript type exports from Rust bindings
├── js/                    # Compiled JavaScript from TypeScript
└── assets/
    └── (static assets like icons, images)
```

## Development

Uses **Bun** as the runtime and package manager:

- `bun run dev` - Start development server with hot reload
- `bun run build-ts` - Compile TypeScript to JavaScript
- `bun run build-tw` - Build Tailwind CSS
- `bun run watch-ts` - Watch TypeScript files for changes
- `bun run watch-tw` - Watch CSS files for changes

## Key Features

1. **TypeScript with Type Safety**: Uses Rust-generated type bindings via ts-rs
2. **Timezone Support**: Client-side timezone utilities with server-side timezone handling
3. **Server-Side Rendering**: All data comes from Tera template context
4. **Form Submissions**: AJAX form submissions with proper error handling
5. **Cookie-based Settings**: User preferences stored in HTTP-only cookies

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

### Pages

- `GET /` - Dashboard page
- `GET /login` - Login page
- `GET /signup` - Signup page
- `GET /profile` - Profile settings page
- `POST /signout` - Handle logout

### API

- `POST /api/v1/login` - Handle login form
- `POST /api/v1/signup` - Handle user registration
- `POST /api/v1/meal` - Add meal entry
- `POST /api/v1/activity` - Add activity entry
- `POST /api/v1/settings` - Update user settings
- `POST /api/v1/goals` - Update user goals
- `GET /api/v1/health` - Health check

### Static

- `GET /static/css/*` - Serve CSS files
- `GET /static/js/*` - Serve JavaScript files
- `GET /static/assets/*` - Serve static assets
