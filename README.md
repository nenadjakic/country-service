# country-service

This project is a **country-service** implemented in Rust programming language, utilizing the Diesel ORM for database interactions. The service provides operations for reading country data based on their  code (alpha-2 code; ISO 3166), name, and by database identifier.
**country-service** has three project:
1. **_rest-service_**: REST server.
2. **_grpc-service_**: gRpc server.
3. **_shared_**. It is lib which contains shared logic for **_rest-service_** and **_grpc-service_**

## Features
### Read Operations:
- Find country by alpha-2 code
- Find countries by name
- Find country by database identifier
- Get all countries
  
## Technologies Used
- Diesel ORM for database operations
- Axum for REST operations

## Build and Run
1. **Clone this repository:**

   `git clone https://github.com/nenadjakic/country-service.git`

   `cd country-service`
2. **Create a new database for the project.**
3. **Configure env variables:**

   Configure port and database connection url inside `.env` file

4. **Build and run the project:**

   `cargo run -p country-rest-service`
   or 
   `cargo run -p country-grpc-service`

_Note: The program will automatically apply migrations on startup._

## Dockerize Application
Follow these steps to dockerize and run your application using Docker and Docker Compose:

1. **Installing Docker and Docker Compose:**

   First, make sure Docker and Docker Compose are installed on your machine. If not, you can follow these guides:
    - [Docker Install Guide](https://docs.docker.com/get-docker/)
    - [Docker Compose Install Guide](https://docs.docker.com/compose/install/)
2. **Running Application with Docker Compose:**

   Use Docker Compose to build and run your application in a container:

   `docker compose up`


## License
This project is licensed under the MIT License - see the LICENSE file for details.
