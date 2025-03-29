# Schoolnc ltd Project

**Status:** _In Progress_

### Development Environment

To run the project in a development environment, execute:
```bash
docker compose up --build
```

### Production Environment

To run the project in a production environment, execute:
```bash
docker-compose -f docker-compose.prod.yml up --build
```

## Project Overview

Schoolnc ltd aims to deliver the best web-native experience for students and professors. This project provides an end-to-end solution enabling users to access relevant information about their student/school life. Functionalities include managing user profiles, authentication, class management, and handling student grades—all accessible via a GraphQL API.

## Architecture

The application is built as a microservices architecture composed of the following services:

1. **Front Service:**  
   - Provides the UI website for both students and professors.
   - Built with Vue.js as a Single Page Application (SPA).
   - Consumes the GraphQL APIs exposed via the API Gateway.

2. **Users Service:**  
   - Manages user profile data (name, email, pseudo, etc.).
   - Provides GraphQL endpoints for user management operations (CRUD).
   - Built with Rust and Actix.

3. **Auth Service:**  
   - Handles user authentication (login, logout, token management) and authorization.
   - Uses JWT tokens and returns authentication cookies.
   - Exposes GraphQL API endpoints.
   - Built with Rust and Actix.

4. **Classes Service:**  
   - Manages class-related information including class creation, updates, deletion, and student enrollment.
   - Provides GraphQL endpoints for class operations.
   - Built with Rust and Actix.

5. **Grades Service:**  
   - Manages student grades and enables professors to create, update, and delete grade records.
   - Provides GraphQL endpoints for grade management and filtering (by course).
   - Built with Rust and Actix.

In addition to these services, the project includes:
- **MongoDB Cluster Databases:**  
  - Each service has its own dedicated database (or collection) to store persistent data.
- **Traefik:**  
  - Acts as both the load balancer and API Gateway, routing incoming HTTP/GraphQL requests to the appropriate services.
- **Docker Compose:**  
  - Orchestrates all the services, Traefik, and MongoDB containers for a streamlined development and deployment process.

## API Documentation

- All backend APIs are GraphQL-based and include a dedicated Swagger/OpenAPI specification for documentation.
- Each service has its own README and Swagger file for detailed API documentation.

## Technology Stack

- **Backend:**  
  - **Language:** Rust  
  - **Framework:** Actix-web  
  - **API Type:** GraphQL
- **Frontend:**  
  - **Framework:** Vue.js
- **Database:**  
  - **MongoDB** (each microservice manages its own collections)
- **Infrastructure:**  
  - **API Gateway & Load Balancer:** Traefik (configured via Docker Compose)
  - **Container Orchestration:** Docker Compose

## Getting Started

### Prerequisites

- Docker & Docker Compose installed on your machine.
- Basic knowledge of Rust and Vue.js.

### Running the Project

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/schoolnc-project.git
   cd schoolnc-project
    ```
2. Build and start all services using Docker Compose:
    
    ```bash
    docker-compose up --build
    ```
    
3. Access the front-end application via the URL provided by Traefik (e.g., `http://localhost`).
4. Each microservice’s GraphQL API is accessible under its respective endpoint (e.g., `/api/auth/graphql`, `/api/users/graphql`, etc.).
5. Swagger documentation for each service will be found in the individual service directories.

## Project Status

This project is currently in progress. We are actively developing and testing each service to ensure robust, secure, and scalable functionality. Contributions and suggestions are welcome!

## Additional Information

- **Testing:**
    
    Basic tests are implemented for core functionalities. Additional tests will be added to cover more use cases and ensure code quality.
    
- **Documentation:**
    
    A detailed README and Swagger documentation will be provided for each service.