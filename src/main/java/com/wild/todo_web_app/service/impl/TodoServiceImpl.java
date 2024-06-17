package com.wild.todo_web_app.service.impl;

import com.wild.todo_web_app.exception.TodoNotFoundException;
import com.wild.todo_web_app.model.Todo;
import com.wild.todo_web_app.repository.TodoRepository;
import com.wild.todo_web_app.service.TodoService;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class TodoServiceImpl implements TodoService {
    private final TodoRepository todoRepository;

    @Override
    public Long createTodo(Todo todo) {
        Todo newTodo = todoRepository.save(todo);

        return newTodo.getId();
    }

    @Override
    public List<Todo> getAllTodos() {
        return todoRepository.findAll();
    }

    @Override
    public Todo getTodoById(Long id) {
        return todoRepository.findById(id)
            .orElseThrow(() -> new TodoNotFoundException("Todo with ID " + id + " not found."));
    }

    @Override
    public Todo updateTodo(Long id, Todo todo) {
        Todo existingTodo = todoRepository.findById(id)
            .orElseThrow(() -> new TodoNotFoundException("Todo with ID " + id + " not found."));

        existingTodo.setTitle(todo.getTitle());
        existingTodo.setDescription(todo.getDescription());
        existingTodo.setUpdatedAt(todo.getUpdatedAt());
        existingTodo.setCompleted(todo.isCompleted());

        return todoRepository.save(existingTodo);
    }

    @Override
    public void deleteTodo(Long id) {
        Todo todo = todoRepository.findById(id)
            .orElseThrow(() -> new TodoNotFoundException("Todo with ID " + id + " not found."));

        todoRepository.deleteById(todo.getId());
    }
}
