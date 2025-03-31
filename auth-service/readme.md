# Auth Service

**Status:** Partially Completed

## Project Overview

The Auth Service is responsible for handling user authentication and basic authorization for the Schoolnc ltd project. This service manages operations such as user registration, login, logout, token refresh, and cleanup of authentication credentials. All operations are exposed via a GraphQL API and are built with Rust and Actix-web, ensuring a robust and secure authentication mechanism.

## Key Features

- **User Registration:**  
  Create authentication credentials for a new user.

- **Login & Logout:**  
  Authenticate users using email and password, and return a JWT as a cookie for session management. Logout functionality removes the authentication cookie.

- **Token Management:**  
  Refresh tokens nearing expiration to maintain secure and stateless sessions.

- **Authentication Details:**  
  Retrieve details of the currently authenticated user to facilitate secure access to protected resources.

- **Credential Cleanup:**  
  Delete authentication credentials when a user is removed, ensuring data consistency across services.

- **GraphQL API:**  
  All authentication operations are exposed through a GraphQL endpoint, offering flexible and efficient queries and mutations.

## Technology Stack

- **Language:** Rust
- **Framework:** Actix-web
- **API Type:** GraphQL
- **Database:** MongoDB (dedicated collection within the auth-service database)
- **Documentation:** Swagger/OpenAPI and this README

## Communication with Backend

The Auth Service integrates with the API Gateway (Traefik) and other backend microservices to provide secure authentication across the system. It communicates with the Users Service for user verification and is a critical component of the overall microservices architecture.

- **GraphQL Endpoint:** `/api/auth/graphql`

GraphQL Operations include:
- **Query:** `healthCheck → String!`  
  _Description:_ Returns "Service is Alive" to indicate that the Auth Service is operational.
  
- **Mutation:** `register(user: CreateAuthRequest) → User`  
  _Description:_ Register a new user by creating authentication credentials.  
  _Access:_ Public.

- **Mutation:** `login(auth: LoginRequest) → AuthCookieResponse`  
  _Description:_ Authenticate a user using email/password and return a JWT (or similar token) as a cookie directly to the front-end.  
  _Access:_ Public.

- **Mutation:** `logout → Boolean`  
  _Description:_ Remove the authentication cookie from the client, effectively logging out the user.  
  _Access:_ Protected.

- **Query:** `get_current_auth → ExternalClaim`  
  _Description:_ Retrieve authentication details (user ID, roles) for the currently authenticated user.  
  _Access:_ Protected.

- **Mutation:** `delete_auth_by_user_id(user_id: String) → Auth`  
  _Description:_ Delete authentication credentials associated with a user. This operation is triggered internally when a user is deleted to ensure proper cleanup.  
  _Access:_ Restricted to internal requests.

