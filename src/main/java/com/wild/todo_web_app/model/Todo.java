package com.wild.todo_web_app.model;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import jakarta.validation.constraints.NotEmpty;
import jakarta.validation.constraints.Size;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.time.LocalDateTime;

@Entity
@Table(name = "tasks")
@NoArgsConstructor
@AllArgsConstructor
@Data
public class Todo {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private Long id;
    @NotEmpty(message = "You must provide a title.")
    private String title;
    @Size(min = 3, max = 255, message = "Description must be between 3 and 255 characters.")
    private String description;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;
    private boolean isCompleted;

    public Todo(String title,
                String description,
                LocalDateTime createdAt,
                LocalDateTime updatedAt,
                boolean isCompleted) {
        this.title = title;
        this.description = description;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
        this.isCompleted = isCompleted;
    }
}
