# Users Service

**Status:** Partially Completed

## Project Overview

The Users Service is responsible for managing user profile data—including name, email, and pseudo—separately from authentication credentials. This service exposes a GraphQL API that enables CRUD operations on user data, ensuring secure profile management. It allows administrators and professors to manage user profiles while ensuring that individual users can only update or delete their own profiles.

## Key Features

- **User Management:**  
  Create, read, update, and delete user profiles. Users can only modify their own profile, while administrators have full control over all profiles.

- **GraphQL API:**  
  All operations are exposed via a GraphQL endpoint, offering flexible queries and mutations.

- **Role-Based Access:**  
  Sensitive operations are restricted to ADMIN (and in some cases, PROFESSOR), ensuring secure data management.

- **Health Check:**  
  A dedicated `healthCheck` query is available to verify that the service is operational.

## Technology Stack

- **Language:** Rust  
- **Framework:** Actix-web  
- **API Type:** GraphQL  
- **Database:** MongoDB (dedicated collection within the users-service database)  
- **Documentation:** Swagger/OpenAPI and this README

## Communication with Backend

The Users Service integrates with the API Gateway (Traefik) and collaborates with other backend microservices, such as the Auth Service for user verification. It plays a crucial role in managing and providing secure access to user data across the overall microservices architecture.

- **GraphQL Endpoint:** `/api/users/graphql`

## GraphQL Operations

- **Query:** `healthCheck → String!`  
  _Description:_ Returns a message such as "Service is Alive" to indicate that the Users Service is operational.  
  _Access:_ Public.

- **Query:** `get_users → Vec<User>`  
  _Description:_ List all users.  
  _Access:_ Restricted to **ADMIN**.

- **Query:** `get_users_by_class_id(class_id: String) → Vec<User>`  
  _Description:_ Retrieve a list of users belonging to the specified class.  
  _Access:_ Restricted to **ADMIN** and **PROFESSOR**.

- **Query:** `get_user_by_email_or_pseudo → User`  
  _Description:_ Retrieve the profile of a specific user by email or pseudo.  
  _Access:_ Restricted to **ADMIN**.

- **Query:** `get_user_by_id(id: String) → User`  
  _Description:_ Retrieve the profile of a specific user.  
  _Access:_ Restricted to **ADMIN**.

- **Query:** `get_current_user → User`  
  _Description:_ Retrieve the profile of the currently authenticated user.  
  _Access:_ Protected (requires a valid token).

- **Mutation:** `create_user(input: CreateUserRequest) → User`  
  _Description:_ Create a new user profile (used if registration is split into two steps).  
  _Access:_ Internal.

- **Mutation:** `update_current_user(input: UserInput) → User`  
  _Description:_ Update the profile of the currently authenticated user.  
  _Access:_ Protected.

- **Mutation:** `update_user_by_id(id: String, input: UserInput) → User`  
  _Description:_ Update a specific user’s profile.  
  _Access:_ Restricted to **ADMIN**.

- **Mutation:** `delete_current_user → User`  
  _Description:_ Delete the profile of the currently authenticated user.  
  _Access:_ Restricted to the user himself.

- **Mutation:** `delete_user_by_id(id: String) → User`  
  _Description:_ Delete a user profile.  
  _Access:_ Restricted to **ADMIN**.