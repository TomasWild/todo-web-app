package com.wild.todo_web_app.controller;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.wild.todo_web_app.TodoWebAppApplication;
import com.wild.todo_web_app.model.Todo;
import com.wild.todo_web_app.service.impl.TodoServiceImpl;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.junit.jupiter.MockitoExtension;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest;
import org.springframework.boot.test.mock.mockito.MockBean;
import org.springframework.http.MediaType;
import org.springframework.test.context.ContextConfiguration;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.ResultActions;
import org.springframework.test.web.servlet.result.MockMvcResultHandlers;
import org.springframework.test.web.servlet.result.MockMvcResultMatchers;

import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.List;

import static org.hamcrest.Matchers.hasSize;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.BDDMockito.given;
import static org.mockito.Mockito.doNothing;
import static org.mockito.Mockito.when;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.delete;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.put;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

@ContextConfiguration(classes = TodoWebAppApplication.class)
@WebMvcTest(controllers = TodoController.class)
@AutoConfigureMockMvc(addFilters = false)
@ExtendWith(MockitoExtension.class)
public class TodoControllerTest {
    @Autowired
    private MockMvc mockMvc;
    @Autowired
    private ObjectMapper objectMapper;
    @MockBean
    private TodoServiceImpl todoService;

    @Test
    public void testCreateTodoApi() throws Exception {
        Todo todo = new Todo(
            "Todo Test",
            "Description test",
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        given(todoService.createTodo(any()))
            .willAnswer((invocation -> {
                Todo createdTodo = invocation.getArgument(0);
                return createdTodo.getId();
            }));

        ResultActions response = mockMvc.perform(post("/api/v1/todos")
            .contentType(MediaType.APPLICATION_JSON)
            .content(objectMapper.writeValueAsString(todo)));

        response.andExpect(status().isCreated())
            .andDo(MockMvcResultHandlers.print());
    }

    @Test
    public void testGetAllTodosApi() throws Exception {
        List<Todo> todos = Arrays.asList(
            new Todo(
                "Todo Test 1",
                "Description test 1",
                LocalDateTime.now(),
                LocalDateTime.now(),
                false
            ),
            new Todo(
                "Todo Test 2",
                "Description test 2",
                LocalDateTime.now(),
                LocalDateTime.now(),
                false
            )
        );

        when(todoService.getAllTodos()).thenReturn(todos);

        ResultActions response = mockMvc.perform(get("/api/v1/todos")
            .contentType(MediaType.APPLICATION_JSON));

        response.andExpect(status().isOk())
            .andExpect(jsonPath("$", hasSize(todos.size())))
            .andDo(MockMvcResultHandlers.print());
    }

    @Test
    public void testGetTodoByIdApi() throws Exception {
        Long todoId = 1L;

        Todo todo = new Todo(
            "Todo Test",
            "Description test",
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        when(todoService.getTodoById(todoId)).thenReturn(todo);

        ResultActions response = mockMvc.perform(
            get("/api/v1/todos/{id}", todoId)
                .contentType(MediaType.APPLICATION_JSON)
        );

        response.andExpect(status().isOk())
            .andExpect(MockMvcResultMatchers.content()
                .json(objectMapper.writeValueAsString(todo))
            )
            .andDo(MockMvcResultHandlers.print());
    }

    @Test
    public void testUpdateTodoApi() throws Exception {
        Long todoId = 1L;

        Todo updatedTodo = new Todo(
            "Todo Test",
            "Description test",
            LocalDateTime.now(),
            LocalDateTime.now(),
            false
        );

        given(todoService.updateTodo(todoId, updatedTodo)).willReturn(updatedTodo);

        ResultActions response = mockMvc.perform(
            put("/api/v1/todos/{id}", todoId)
                .contentType(MediaType.APPLICATION_JSON)
                .content(objectMapper.writeValueAsString(updatedTodo))
        );

        response.andExpect(status().isOk())
            .andExpect(MockMvcResultMatchers.content()
                .json(objectMapper.writeValueAsString(updatedTodo))
            )
            .andDo(MockMvcResultHandlers.print());
    }

    @Test
    public void testDeleteTodoApi() throws Exception {
        Long todoId = 1L;

        doNothing().when(todoService).deleteTodo(todoId);

        ResultActions response = mockMvc.perform(delete(
            "/api/v1/todos/{id}", todoId)
            .contentType(MediaType.APPLICATION_JSON)
        );

        response.andExpect(status().isOk())
            .andDo(MockMvcResultHandlers.print());
    }
}
