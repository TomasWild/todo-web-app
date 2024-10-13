# Simple Backend for a Todo Web App

---

## Introduction

This project is a simple backend service for managing todos.
It allows you to create, read, update,
and delete todo items while using a PostgreSQL database for data persistence.

## Features

- **Todo Management:**\
  Create, read, update, and delete todos.
- **PostgreSQL Database:**\
  Store todo data.

## API Endpoints

### Todo Management

- **POST** `/api/v1/todos` – Create a new todo.
- **GET** `/api/v1/todos` – Retrieve all todos.
- **GET** `/api/v1/todos/:id` – Retrieve a todo by id.
- **PUT** `/api/v1/todos/:id` – Update an existing todo.
- **DELETE** `/api/v1/todos/:id` – Delete a todo.

## Setup

**STEP 1:** Configure Environment Variables.\
Create a `.env` file in the project root and configure it with the following variables:

```dotenv
# Server configuration 
# (If not specified in the environment variables, these values will be used)
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Database configuration
DATABASE_URL=postgres://<DB_USER>:<DB_PASSWORD>@<HOST>:5432/<DB_NAME>
DATABASE_USER=DB_USER
DATABASE_PASSWORD=DB_PASSWORD

# Logging configuration
RUST_LOG=todo_web_app=<LOG-LEVEL>
```

**STEP 2:** Start the Database.\
Run the following command in your terminal from the project root

```bash
docker-compose up -d
```

## Troubleshooting

- **Database connection error:** \
  Ensure that PostgreSQL is running and the credentials in the `.env` file are correct.
- **Port in use error:** \
  If port `3000` is already in use,
  modify the `SERVER_PORT` variable in the `.env` file or stop the process running on that port.