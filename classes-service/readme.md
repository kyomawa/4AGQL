# Classes Service

**Status:** In Progress

## Project Overview

The Classes Service is responsible for managing class-related information for the Schoolnc ltd project. This service exposes a GraphQL API to perform CRUD operations on class data and to manage student enrollment within classes. It allows anyone to view class details, while only professors can create, update, delete, or enroll students in classes.

## Key Features

- **Class Management:**  
  Create, read, update, and delete classes.  
  Deletion is only allowed if no students are enrolled in the class.

- **Student Enrollment:**  
  Enroll students into classes, ensuring that only authorized users (professors) can perform this operation.

- **GraphQL API:**  
  All operations are exposed via a GraphQL endpoint for flexible queries and mutations.

- **Health Check:**  
  A dedicated `healthCheck` query is available to verify that the service is operational.

## Technology Stack

- **Language:** Rust  
- **Framework:** Actix-web  
- **API Type:** GraphQL  
- **Database:** MongoDB (dedicated collection within the classes-service database)  
- **Documentation:** Swagger/OpenAPI and this README

## Communication with Backend

The Classes Service integrates with the API Gateway (Traefik) and communicates with other backend microservices as needed. It plays a vital role in providing access to class data and managing student enrollment within the overall system architecture.

- **GraphQL Endpoint:** `/api/classes/graphql`

## GraphQL Operations

- **Query:** `healthCheck → String!`  
  _Description:_ Returns "Service is Alive" to indicate that the Classes Service is operational.  
  _Access:_ Public.

- **Query:** `get_classes(sortBy: Option<String>) → Vec<Class>`  
  _Description:_ List all classes, optionally sorted by a specified field (e.g., name).  
  _Access:_ Public.

- **Query:** `get_class_by_id(id: String) → Class`  
  _Description:_ Retrieve detailed information for a specific class, including the number of enrolled students and any associated courses.  
  _Access:_ Public.

- **Mutation:** `create_class(class: CreateClassRequest) → Class`  
  _Description:_ Create a new class.  
  _Access:_ Restricted to **PROFESSOR OR ADMIN**.

- **Mutation:** `update_class_by_id(id: String, class: UpdateClassRequest) → Class`  
  _Description:_ Update an existing class.  
  _Access:_ Restricted to **PROFESSOR OR ADMIN**.

- **Mutation:** `delete_class_by_id(id: String) → Class`  
  _Description:_ Delete a class.  
  _Constraints:_ Deletion is allowed only if no students are enrolled in the class.  
  _Access:_ Restricted to **PROFESSOR OR ADMIN**.

- **Mutation:** `enroll_student(classId: String, studentId: String) → Class`  
  _Description:_ Enroll a student into a specific class.  
  _Access:_ Restricted to **PROFESSOR OR ADMIN**.
