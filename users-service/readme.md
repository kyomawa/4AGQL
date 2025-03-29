# Users Service

**Status:** In Progress

## Project Overview

The Users Service is responsible for managing user profile data such as name, email, and pseudo, separate from authentication credentials. This service exposes a GraphQL API to perform CRUD operations on user data, allowing administrators and users to manage profiles securely.

## Key Features

- **User Management:**  
  Create, read, update, and delete user profiles.  
  Users can only update or delete their own profile, while administrators have broader access.

- **GraphQL API:**  
  All operations are exposed through a GraphQL endpoint, ensuring flexible queries and mutations.

- **Secure Access:**  
  Only authenticated users can access personal profile data, and sensitive operations are restricted to administrators or authorized roles.

## Technology Stack

- **Language:** Rust
- **Framework:** Actix-web
- **API Type:** GraphQL
- **Database:** MongoDB (each service has its own dedicated database/collection)
- **Documentation:** Swagger/OpenAPI and individual service README

## Communication with Backend

This service is designed to interact with the API Gateway (Traefik) and other backend microservices. It communicates with the Auth Service for user verification and integrates seamlessly with the overall system architecture.

- **GraphQL Endpoint:** `/api/users/graphql`

GraphQL Operations include:
- `get_users` – List all users 
- `get_users_by_class_id` – Retrieve a list of users belonging to the specified class
- `get_user_by_id` – Retrieve a specific user's profile
- `get_current_user` – Retrieve the profile of the authenticated user
- `create_user` – Create a new user profile
- `update_current_user` – Update the authenticated user's profile
- `update_user_by_id` – Update a specific user’s profile
- `delete_current_user` – Delete the authenticated user's profile
- `delete_user_by_id` – Delete a user profile

## Project Status & Future Enhancements

This project is currently in progress. Future plans include:
- Expanding the user management features (e.g., profile picture uploads, social links).
- Enhancing security measures and error handling.
- Adding detailed logging and monitoring.
- Further refining the GraphQL schema based on user feedback.