# Grades Service

**Status:** Partially Completed

## Project Overview

The Grades Service manages all operations related to student grades for the Schoolnc ltd project. This service provides a GraphQL API that allows users to retrieve their grades and enables professors to create, update, and delete grade records. It ensures that students only see their own grades while granting professors broader access to manage and analyze grades.

## Key Features

- **Grade Management:**  
  Create, read, update, and delete grade records.  
  Only professors can perform create, update, or delete operations, while students can only view their own grades.

- **Filtering Capabilities:**  
  Retrieve grades filtered by a specific course if provided.

- **GraphQL API:**  
  All operations are exposed via a GraphQL endpoint, allowing for flexible queries and mutations.

- **Health Check:**  
  A dedicated `healthCheck` query is available to confirm that the service is operational.

## Technology Stack

- **Language:** Rust  
- **Framework:** Actix-web  
- **API Type:** GraphQL  
- **Database:** MongoDB (dedicated collection within the grades-service database)  
- **Documentation:** Swagger/OpenAPI and this README

## Communication with Backend

The Grades Service integrates with the API Gateway (Traefik) and interacts with other backend services, such as the Users Service for user verification and the Courses Service if course details are needed. It plays a critical role in providing secure and role-based access to grade data across the system.

- **GraphQL Endpoint:** `/api/grades/graphql`

## GraphQL Operations

- **Query:** `healthCheck → String!`  
  _Description:_ Returns "Service is Alive" to indicate that the Grades Service is operational.  
  _Access:_ Public.

- **Query:** `get_grades(course: Option<String>) → Vec<Grade>`  
  _Description:_ Retrieve a list of grades for the authenticated user.  
  **Note:** If `course` is provided, only grades for the specified course(s) are returned.  
  _Access:_  
  - **USER (student):** Returns only their own grades.  
  - **PROFESSOR:** Can view grades across multiple students.

- **Query:** `get_grade_by_id(id: String) → Grade`  
  _Description:_ Retrieve details of a specific grade record.  
  _Access:_ Accessible by **PROFESSOR** and the owner student (role-based).

- **Mutation:** `create_grade(grade: CreateGradeRequest) → Grade`  
  _Description:_ Create a new grade record for a course.  
  _Access:_ Restricted to **PROFESSOR**.

- **Mutation:** `update_grade_by_id(id: String, input: UpdateGradeRequest) → Grade`  
  _Description:_ Update an existing grade record.  
  _Access:_ Restricted to **PROFESSOR**.

- **Mutation:** `delete_grade_by_id(id: String) → Grade`  
  _Description:_ Delete a grade record.  
  _Access:_ Restricted to **PROFESSOR**.