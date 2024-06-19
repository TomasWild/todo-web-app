package com.wild.todo_web_app.repository;

import com.wild.todo_web_app.model.Priority;
import com.wild.todo_web_app.model.Todo;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface TodoRepository extends JpaRepository<Todo, Long> {
    @Query("SELECT t FROM Todo WHERE t.priority = :priority")
    List<Todo> findTodosByPriority(Priority priority);
}
