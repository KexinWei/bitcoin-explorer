
# Bitcoin Explorer

This is a Bitcoin Explorer project that includes a backend service for data ingestion and a frontend application for data visualization. The backend is built with Rust, and the frontend is built with React and TypeScript. The application provides real-time Bitcoin data, including the latest block information, market data, network stats, and visualizes them in a cyberpunk-themed interface.

## Project Structure

```
bitcoin-explorer/
├── bitcoin-explorer-ingestion/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── Dockerfile
│   ├── src/
│   │   ├── main.rs
│   │   ├── ingestion.rs
│   │   └── server.rs
│   └── .env
├── bitcoin-explorer-frontend/
│   ├── package.json
│   ├── package-lock.json
│   ├── tsconfig.json
│   ├── Dockerfile
│   ├── public/
│   │   └── index.html
│   └── src/
│       ├── App.tsx
│       ├── App.css
│       ├── index.tsx
│       └── index.css
├── docker-compose.yml
├── init.sql
└── README.md
```

## Features

- **Backend Service**:
  - Fetches Bitcoin market data from CoinGecko API.
  - Fetches network statistics from Blockchain.info API.
  - Fetches the latest block information from Blockstream API.
  - Stores data in a PostgreSQL database.
  - Provides RESTful APIs to serve data to the frontend.

- **Frontend Application**:
  - Displays the latest block information.
  - Visualizes hash rate and difficulty trends.
  - Shows current Bitcoin price and price/volume trends.
  - Cyberpunk-themed UI with responsive design.

## Prerequisites

- **Docker** and **Docker Compose** installed on your system.

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/KexinWei/bitcoin-explorer.git
cd bitcoin-explorer
```

### Build and Run with Docker Compose

Build and start all the services (backend, frontend, and PostgreSQL database) using Docker Compose:

```bash
docker-compose up --build
```

This command will:

- Build the backend Docker image and start the backend service.
- Build the frontend Docker image and start the frontend service.
- Pull the PostgreSQL image, initialize the database with `init.sql`, and start the database service.

### Access the Application

Once all the services are up and running, you can access the application in your web browser at:

```
http://localhost:8080
```

The backend API is accessible at:

```
http://localhost:3001
```

### Stopping the Services

To stop the running services, press `Ctrl+C` in the terminal where `docker-compose` is running, or run:

```bash
docker-compose down
```

## Project Details

### Backend

- **Language**: Rust
- **Frameworks/Crates**:
  - `tokio` for asynchronous runtime.
  - `reqwest` for HTTP requests.
  - `tokio-postgres` for database connections.
  - `warp` for building the web server.
  - `dotenv` for environment variable management.

- **Services**:
  - Ingestion service that periodically fetches data from APIs and stores in the database.
  - RESTful API server that serves data to the frontend.

- **APIs Provided**:
  - `/market-data`: Returns historical market data.
  - `/network-data`: Returns historical network statistics.
  - `/latest-block`: Returns the latest block information.

### Frontend

- **Language**: TypeScript
- **Framework**: React
- **Libraries**:
  - `axios` for HTTP requests.
  - `react-chartjs-2` and `chart.js` for data visualization.

- **Features**:
  - Responsive design with a cyberpunk theme.
  - Real-time updates using periodic polling.

### Database

- **Database**: PostgreSQL
- **Initialization**:
  - The `init.sql` script is used to initialize the database schema when the PostgreSQL container starts.

## Environment Variables

The backend service uses the following environment variables:

- `DATABASE_URL`: The connection string for the PostgreSQL database.

In the provided `docker-compose.yml`, the environment variables are set appropriately, and you do not need to manually set them unless you are running the services outside of Docker.

## Docker Configuration

### Backend Dockerfile

Located at `backend/Dockerfile`, it builds the Rust application and creates a minimal Docker image for running the backend service.

### Frontend Dockerfile

Located at `frontend/Dockerfile`, it builds the React application and serves it using Nginx.

### Docker Compose

The `docker-compose.yml` file orchestrates the services:

- **postgres**: The PostgreSQL database service.
- **backend**: The Rust backend service.
- **frontend**: The React frontend service.

## Notes

- The first time you run the application, the backend service will fetch historical data, which may take some time. Please be patient while the data is being populated.
- Ensure that the ports `80`, `3001`, and `5432` are not occupied by other services on your machine.

## Troubleshooting

- **Port Conflicts**: If you encounter port conflicts, you can change the ports in `docker-compose.yml`.
- **Data Not Displaying**: If the frontend does not display data, ensure that the backend service is running and accessible at `http://localhost:3001`.
- **Database Issues**: If the backend cannot connect to the database, check that the `DATABASE_URL` environment variable is correctly set, and the database service is running.

## License

This project is licensed under the MIT License.

## Acknowledgments

- [CoinGecko API](https://www.coingecko.com/en/api) for market data.
- [Blockchain.info API](https://www.blockchain.com/api) for network statistics.
- [Blockstream API](https://github.com/Blockstream/esplora/blob/master/API.md) for blockchain data.
- Inspired by cyberpunk aesthetics for the UI design.

---

**Enjoy exploring Bitcoin data with a cyberpunk twist!**
