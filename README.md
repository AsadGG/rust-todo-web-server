# Rust Todo Web Server

Welcome to the **Rust Todo Web Server** project! This project aims to create a simple web server for managing todos, built using the Rust programming language. As a newcomer to Rust, I'm excited to share my progress and developments. If you're experienced with Rust and have suggestions for improvements or better approaches, please feel free to contribute or provide feedback.

## Features

- **CORS Implementation (Todo)**: Enabling Cross-Origin Resource Sharing (CORS) is on the to-do list. This will allow the web server to handle requests from different origins securely.

- **JWT Implementation (Done)**: JSON Web Tokens (JWT) have been successfully implemented. This ensures secure authentication and authorization for the todo web server.

- **OpenAPI Swagger (Done)**: The project has integrated OpenAPI Swagger, making it easier to document and visualize the API endpoints.

- **Password Hashing (Done)**: User passwords are securely hashed before being stored in the database.

- **Request Payload Validation (Done)**: Input data is properly validated to ensure the integrity and validity of user-submitted information.

- **PostgreSQL Integration (Done)**: The web server uses PostgreSQL as its database to persistently store todo data.

- **Daily Changing File-based Logging (Done)**: Logs are generated and stored in a file that changes every day, providing organized and time-stamped logs for debugging and monitoring purposes.

### Upcoming Features for Learning Purposes

- **Firebase Cloud Messaging (Todo)**: I am planning to implement Firebase Cloud Messaging for sending push notifications. This feature will help me understand integrating external services into a Rust application.

- **Sendgrid Mailer (Todo)**: I intend to integrate Sendgrid for sending emails. This will provide practical experience in working with email APIs and managing communication channels.

- **Stripe Integration (Todo)**: Incorporating Stripe for payment processing is planned. This will give me insights into handling financial transactions securely within the context of a Rust project.

## Getting Started

1. **Prerequisites**: Make sure you have Rust and PostgreSQL installed on your system.

2. **Clone the Repository**: Use the following command to clone this repository to your local machine:

   ```bash
   git clone https://github.com/AsadGG/rust-todo-web-server.git
   ```

3. **Install Dependencies**: Navigate to the project directory and install the required dependencies:

   ```bash
   cd rust-todo-web-server
   cargo build
   ```

4. **Environment Setup**: Configure your environment variables settings in the `.env` file. `.env_sample` for refrence

5. **Run the Server**: Start the web server using the following command:

   ```bash
   cargo run
   ```

6. **API Documentation**: Access the OpenAPI Swagger documentation by visiting `http://localhost:8080/swagger-ui/` in your browser.

## Contribution

As a newcomer to Rust, I'm open to learning and improving the project. If you have any suggestions, improvements, or better ways to achieve the implemented features, please don't hesitate to create an issue or submit a pull request. Your contributions are highly appreciated!
