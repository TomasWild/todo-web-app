package com.wild.todo_web_app.service;

import com.wild.todo_web_app.model.Todo;

import java.util.List;

public interface TodoService {
    Long createTodo(Todo todo);
    List<Todo> getAllTodos();
    Todo getTodoById(Long id);
    Todo updateTodo(Long id, Todo todo);
    void deleteTodo(Long id);
}
