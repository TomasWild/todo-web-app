package com.wild.todo_web_app.controller;

import com.wild.todo_web_app.model.Todo;
import com.wild.todo_web_app.service.impl.TodoServiceImpl;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/api/v1/todos")
@RequiredArgsConstructor
public class TodoController {
    private final TodoServiceImpl todoService;

    @PostMapping
    public ResponseEntity<Long> createTodo(@RequestBody Todo todo) {
        Long newTodo = todoService.createTodo(todo);

        return new ResponseEntity<>(newTodo, HttpStatus.CREATED);
    }

    @GetMapping
    public ResponseEntity<List<Todo>> getAllTodos() {
        List<Todo> todos = todoService.getAllTodos();

        return new ResponseEntity<>(todos, HttpStatus.OK);
    }

    @GetMapping("{id}")
    public ResponseEntity<Todo> getTodoById(@PathVariable("id") Long id) {
        Todo todo = todoService.getTodoById(id);

        return new ResponseEntity<>(todo, HttpStatus.OK);
    }

    @PutMapping("{id}")
    public ResponseEntity<Todo> updateTodo(@PathVariable("id") Long id,
                                           @RequestBody Todo todo) {
        Todo updatedTodo = todoService.updateTodo(id, todo);

        return new ResponseEntity<>(updatedTodo, HttpStatus.OK);
    }

    @DeleteMapping
    public ResponseEntity<Todo> deleteTodo(Long id) {
        todoService.deleteTodo(id);

        return new ResponseEntity<>(HttpStatus.OK);
    }
}
