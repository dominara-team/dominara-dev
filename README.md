# Dominara - Development Environment

This is the development environment for Dominara, a desktop application built with Tauri.

    Node.js (v22 or higher)
    Rust (stable)
    npm

## Getting Started

Clone the repository: `git clone https://github.com/dominara-team/dominara-dev.git`

Install dependencies: `npm install`

Run the app in development mode: `npm run tauri dev`

Build the app for production: `npm run tauri build`

## Environment Variables

Configuration is managed via a .env file. Copy the example file and adjust as needed:

```
cp .env.example .env
```

## API endpoint for backend services

API_URL=http://localhost:3000

## Scripts

```
npm run tauri dev: Starts the app in development mode with hot reloading.
npm run tauri build: Builds the app for distribution.
npm start: Runs the frontend (if applicable).
```
