package com.wild.todo_web_app.service;

import com.wild.todo_web_app.exception.TodoNotFoundException;
import com.wild.todo_web_app.model.Priority;
import com.wild.todo_web_app.model.Todo;
import com.wild.todo_web_app.repository.TodoRepository;
import com.wild.todo_web_app.service.impl.TodoServiceImpl;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

@ExtendWith(MockitoExtension.class)
public class TodoServiceTest {
    @Mock
    private TodoRepository todoRepository;
    @InjectMocks
    private TodoServiceImpl todoService;

    @Test
    public void testCreateTodo() {
        Todo todo = new Todo(
            1L,
            "Todo Test",
            "Description test",
            Priority.LOW,
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        when(todoRepository.save(todo)).thenReturn(todo);

        Long todoId = todoService.createTodo(todo);

        assertNotNull(todoId);
    }

    @Test
    public void testFindAllTodos() {
        List<Todo> expectedTodos = Arrays.asList(
            new Todo(
                1L,
                "Todo Test 1",
                "Description test 1",
                Priority.MEDIUM,
                LocalDateTime.now(),
                LocalDateTime.now(),
                false
            ),
            new Todo(
                2L,
                "Todo Test 2",
                "Description test 2",
                Priority.LOW,
                LocalDateTime.now(),
                LocalDateTime.now(),
                false
            )
        );

        when(todoRepository.findAll()).thenReturn(expectedTodos);

        List<Todo> actualTodos = todoService.getAllTodos();

        assertEquals(expectedTodos.size(), actualTodos.size());
    }

    @Test
    public void testFindTodoById() {
        Long todoId = 1L;

        Todo expectedTodo = new Todo(
            "Todo Test",
            "Description test",
            Priority.VITAL,
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        when(todoRepository.findById(todoId)).thenReturn(Optional.of(expectedTodo));

        Todo actualTodo = todoService.getTodoById(todoId);

        assertEquals(expectedTodo, actualTodo);
    }

    @Test
    public void testFindTodoByIdNotFound() {
        Long todoId = 1L;

        when(todoRepository.findById(todoId)).thenReturn(Optional.empty());

        assertThrows(
            TodoNotFoundException.class,
            () -> todoService.getTodoById(todoId)
        );
    }

    @Test
    public void testFindTodosByPriority() {
        LocalDateTime fixedDateTime = LocalDateTime.of(2024, 6, 19, 13, 0);

        List<Todo> expectedTodos = Arrays.asList(
            new Todo(
                1L,
                "Todo Test 1",
                "Description test 1",
                Priority.MEDIUM,
                fixedDateTime,
                fixedDateTime,
                false
            ),
            new Todo(
                2L,
                "Todo Test 2",
                "Description test 2",
                Priority.LOW,
                fixedDateTime,
                fixedDateTime,
                false
            ),
            new Todo(
                3L,
                "Todo Test 3",
                "Description test 3",
                Priority.LOW,
                fixedDateTime,
                fixedDateTime,
                false
            )
        );

        when(todoRepository.findTodosByPriority(Priority.LOW)).thenReturn(Arrays.asList(
            new Todo(
                2L,
                "Todo Test 2",
                "Description test 2",
                Priority.LOW,
                fixedDateTime,
                fixedDateTime,
                false
            ),
            new Todo(
                3L,
                "Todo Test 3",
                "Description test 3",
                Priority.LOW,
                fixedDateTime,
                fixedDateTime,
                false
            )
        ));

        List<Todo> actualTodos = todoService.getTodosByPriority(Priority.LOW);

        assertEquals(2, actualTodos.size());
        assertEquals(expectedTodos.get(1), actualTodos.getFirst());
        assertEquals(expectedTodos.get(2), actualTodos.get(1));
    }

    @Test
    public void testUpdateTodoSuccess() {
        Long todoId = 1L;

        Todo existingTodo = new Todo(
            "Todo Test",
            "Description test",
            Priority.HIGH,
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        Todo updatedTodo = new Todo(
            "Updated Todo Test",
            "Updated description test",
            Priority.LOW,
            existingTodo.getCreatedAt(),
            LocalDateTime.now(),
            true
        );

        when(todoRepository.findById(todoId)).thenReturn(Optional.of(existingTodo));
        when(todoRepository.save(any())).thenReturn(existingTodo);

        Todo _ = todoService.updateTodo(todoId, updatedTodo);

        assertEquals(updatedTodo.getTitle(), existingTodo.getTitle());
        assertEquals(updatedTodo.getDescription(), existingTodo.getDescription());
        assertEquals(updatedTodo.getUpdatedAt(), existingTodo.getUpdatedAt());
        assertEquals(updatedTodo.isCompleted(), existingTodo.isCompleted());

        verify(todoRepository, times(1)).findById(todoId);
        verify(todoRepository, times(1)).save(existingTodo);
    }

    @Test
    public void testUpdateTodoNotFound() {
        Long todoId = 1L;

        Todo updatedTodo = new Todo(
            "Todo Test",
            "Description test",
            Priority.HIGH,
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        when(todoRepository.findById(todoId)).thenReturn(Optional.empty());

        assertThrows(
            TodoNotFoundException.class,
            () -> todoService.updateTodo(todoId, updatedTodo)
        );

        verify(todoRepository, times(1)).findById(todoId);
        verify(todoRepository, never()).save(any());
    }

    @Test
    public void testDeleteTodoById() {
        Long todoId = 1L;

        Todo todo = new Todo(
            "Todo Test",
            "Description test",
            Priority.LOW,
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        when(todoRepository.findById(todoId)).thenReturn(Optional.of(todo));

        todoService.deleteTodo(todoId);

        verify(todoRepository, times(1)).deleteById(todo.getId());
    }
}
