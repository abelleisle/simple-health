---

## Frontend Web Development Guidelines

### Runtime & Package Manager

- Use **Bun** as the runtime and package manager
- Use `bun run dev` instead of `npm run dev`
- Use `bun add` instead of `npm install`
- Use `bun build` instead of other build tools

### Framework & Performance

- **Raw TypeScript** with Vite for optimal performance
- No heavy frameworks - keep things lightweight and fast
- Direct DOM manipulation for UI updates

### Styling

- **Tailwind CSS v4.1** with new syntax
- Use `@import "tailwindcss"` (v4 syntax) instead of `@tailwind base/components/utilities`
- Configured with Vite integration via `@tailwindcss/vite`

### Development Commands

- `bun run dev` - Start development server with hot reload
- `bun run build` - Build for production
- `bun run preview` - Preview production build

### Project Structure

- `src/main.ts` - Main TypeScript entry point
- `src/style.css` - Tailwind CSS imports
- `index.html` - Main HTML template
