# Front Service

**Status:** In Progress

## Project Overview

The Front Service is the user interface component of the Schoolnc ltd project. This service delivers a responsive single-page application (SPA) built with Vue.js, allowing both students and professors to interact with the system. The application communicates with various backend microservices through GraphQL APIs, which are routed via our API Gateway (Traefik).

## Key Features

- **Responsive UI:**  
  The application is designed to work seamlessly on a variety of devices, providing a smooth user experience.

- **GraphQL Integration:**  
  The front-end interacts securely with backend services (Users, Auth, Grades, Classes, etc.) via GraphQL endpoints.

- **Role-Based Views:**  
  - **Students:** Can view their own grades, classes, and course-related information.
  - **Professors:** Can manage grades, classes, and view aggregated student data.

## Technology Stack

- **Framework:** Nuxt
- **Communication:** GraphQL (via API Gateway)
- **Styling & UI Components:** TailwindCss, ShadcnUI

## Communication with Backend

The front-end is configured to interact with the GraphQL APIs provided by the API Gateway. Ensure that your Docker Compose environment (including Traefik, MongoDB, and backend services) is up and running. The GraphQL endpoints are:

- Auth Service: `/api/auth/graphql`
- Users Service: `/api/users/graphql`
- Grades Service: `/api/grades/graphql`
- Classes Service: `/api/classes/graphql`

## Project Status & Future Enhancements

This project is currently in progress. We are continuously refining the user interface and integrating additional backend functionalities. Planned improvements include:

- Enhanced UI/UX design and additional pages.
- Improved error handling and user notifications.
- Integration with further backend features as they become available.

