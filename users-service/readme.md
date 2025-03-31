# Grades Service

**Status:** Partially Completed

## Project Overview

The Grades Service manages all operations related to student grades for the Schoolnc ltd project. This service provides a GraphQL API that allows users to retrieve their grades while enabling professors and administrators to create, update, and delete grade records. It ensures that students only see their own grades, whereas professors have controlled access based on class assignments and ownership of the grade record.

## Key Features

- **Grade Management:**  
  Create, read, update, and delete grade records.  
  - **Students:** Can view only their own grades.
  - **Professors:** Can create, update, and delete grade records, but only if the grade belongs to a student enrolled in one of the classes they teach, and only if they originally created the record.
  - **Administrators:** Have full access to view, create, update, and delete any grade record.

- **Filtering Capabilities:**  
  Retrieve grades filtered by a specific course if provided.

- **GraphQL API:**  
  All operations are exposed via a GraphQL endpoint for flexible queries and mutations.

- **Health Check:**  
  A dedicated `healthCheck` query is available to confirm that the service is operational.

## Technology Stack

- **Language:** Rust  
- **Framework:** Actix-web  
- **API Type:** GraphQL  
- **Database:** MongoDB (dedicated collection within the grades-service database)  
- **Documentation:** Swagger/OpenAPI and this README

## Communication with Backend

The Grades Service integrates with the API Gateway (Traefik) and interacts with other backend services, such as the Users Service for user verification and the Courses Service (if detailed course information is required). It plays a critical role in providing secure and role-based access to grade data across the system.

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
  - **PROFESSOR or ADMIN:**  
    - **Professors:** Can view grades across multiple students only if the student is enrolled in at least one class that the professor teaches (i.e., the student's `class_ids` intersect with the professor's assigned `class_ids`).  
    - **ADMINs:** Can view all grades without restriction.

- **Query:** `get_grade_by_id(id: String) → Grade`  
  _Description:_ Retrieve details of a specific grade record.  
  _Access:_  
  - **USER (student):** Can access only if the grade belongs to them.  
  - **PROFESSOR or ADMIN:**  
    - **Professors:** Can view grade details only if the grade belongs to a student enrolled in one of the classes they teach.  
    - **ADMINs:** Can view all grade details.

- **Mutation:** `create_grade(grade: CreateGradeRequest) → Grade`  
  _Description:_ Create a new grade record for a course.  
  **Note:** Ensure that the user with the provided `user_id` exists.  
  _Access:_ Restricted to **PROFESSOR or ADMIN**.
  - **Additional Note for Professors:** A professor can only create a grade if he belong to the class of the grade. A professor can only create a grade with his own id (not another professor_id). The creating professor's ID is stored with the grade record to enforce that only the creator can later modify it.

- **Mutation:** `update_grade_by_id(id: String, input: UpdateGradeRequest) → Grade`  
  _Description:_ Update an existing grade record.  
  **Note:**  
  - A professor can only update a grade record if they originally created the grade.  
  - If an ADMIN performs the update, they bypass this restriction.
  _Access:_ Restricted to **PROFESSOR or ADMIN**.

- **Mutation:** `delete_grade_by_id(id: String) → Grade`  
  _Description:_ Delete a grade record.  
  **Note:**  
  - A professor can only delete a grade record if they originally created the grade.  
  - ADMINs can delete any grade record.
  _Access:_ Restricted to **PROFESSOR or ADMIN**.
