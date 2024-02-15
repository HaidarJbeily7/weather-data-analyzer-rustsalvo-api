# Weather Analyzer Readme

This project is a simple weather analysis server that utilizes a third-party weather API to fetch real-time weather information for a specified city. It is built in Rust using the Salvo web framework and relies on the reqwest and dotenv crates for handling HTTP requests and managing environment variables, respectively.

## Features

### 1. Weather Analysis Endpoint

- **Endpoint**: `/analyze/<city>`
- **Method**: GET
- **Description**: Fetches and analyzes real-time weather information for the specified city.
- **Response**: Returns a JSON object containing relevant weather data, such as location details and current weather conditions.

### 2. Health Check Endpoint

- **Endpoint**: `/`
- **Method**: GET
- **Description**: Simple health check endpoint to verify the server's status.
- **Response**: Returns the message "Server Health!".

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [dotenv](https://crates.io/crates/dotenv)

### Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/your_username/weather-analyzer.git
   cd weather-analyzer
   ```

2. Create a .env file in the project root directory and set the environment variable for the weather API key:

   ```bash
       WEATHER_API_KEY=your_weather_api_key
   ```

3. Build & Run the project:
   ```bash
   cargo build
   cargo run
   ```
