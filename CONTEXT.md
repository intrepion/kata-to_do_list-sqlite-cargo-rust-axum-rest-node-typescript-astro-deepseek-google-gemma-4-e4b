# Application Domain Model: To Do List Service

## 🧭 Purpose
This document serves as the single source of truth (the "CONTEXT.md") for the To Do List application, defining all entities, their attributes, relationships, and the immutable business rules that govern them. This document must remain agnostic of implementation details (e.g., "is this in Rust or TypeScript?").

---

## 👤 Entities

### Project
The `Project` is the top-level container for related tasks. It represents a bounded body of work that may span multiple sessions or lifetimes.

| Attribute | Type | Constraint/Description |
| :--- | :--- | :--- |
| `id` | UUID | Primary Key. Unique identifier for this project instance. |
| `title` | String | A concise, user-facing summary of the project. *Required.* |
| `description` | Text | Detailed scope, goals, or background context. Optional. |
| `created_at` | Timestamp | System-generated timestamp of initial creation. |
| `updated_at` | Timestamp | Last time the project metadata was modified. |

### Task
The `Task` is a single, actionable unit of work within the context of a specific Project. It is the most dynamic and frequently updated entity.

| Attribute | Type | Constraint/Description |
| :--- | :--- | :--- |
| `id` | UUID | Primary Key. Unique identifier for this specific task instance. |
| `project_id` | UUID | Foreign Key linking the Task to its parent Project. *Required.* |
| `title` | String | The short, actionable subject line of the task. *Required.* |
| `description` | Text | Detailed notes, acceptance criteria, or execution steps. Optional. |
| `status` | Enum | The current state of the task within its lifecycle. *Required.* |
| `due_date` | Date/Time | The target completion deadline. Optional. |
| `created_at` | Timestamp | System-generated timestamp of initial creation. |
| `updated_at` | Timestamp | Last time the task metadata was modified. |

---

## 🚦 Domain Lifecycle & Business Rules (The "How Things Move")

### Task Status Workflow
The `status` field is not merely a label; it enforces the linear progression of work. The system must enforce the following transitions:

1.  **`PENDING` (To Do):** Initial state. The task has been created but not yet begun or actively worked on.
2.  **`IN_PROGRESS` (Active):** The task is actively being worked on by a responsible party.
3.  **`COMPLETED` (Done):** The task meets all acceptance criteria and is finished. This is a terminal state for this specific instance of the task.

### Transition Guardrails
*   **Linear Progression:** The only valid, non-error-inducing path is `PENDING` $\rightarrow$ `IN_PROGRESS` $\rightarrow$ `COMPLETED`.
*   **Rejection of Jumps:** Attempting to transition from a terminal state (`COMPLETED`) or skipping a stage is rejected by the system.
*   **Reopening:** If a task in `COMPLETED` status requires further work, it must be explicitly converted into a brand-new task instance in the `PENDING` state.

### Completion Prerequisite (Auditability)
*   **Mandatory Note:** Any transition to the `COMPLETED` status is contingent upon providing a meaningful **Resolution Note** (`resolution_note`). This note captures the final success criteria or findings, preventing silent acceptance of incomplete work.

---
**NOTE:** This document governs the *behavior* of the application, not the language used in any single implementation. The current implementations (Rust/Axum/SQLite) are bound by these rules.