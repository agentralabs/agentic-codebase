---
status: stable
---

# MCP Prompts

AgenticCodebase provides 2 built-in MCP prompts that agents can invoke for structured code analysis reasoning.

## `analyse_unit`

Analyse a code unit including its dependencies, stability, and test coverage.

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `unit_name` | string | Yes | Name of the code unit to analyse |
| `graph` | string | No | Graph name |

### Behavior

The prompt instructs the agent to:

1. Look up the unit by name in the code graph
2. Examine its forward and reverse dependencies
3. Assess stability based on change history and complexity
4. Check for test coverage and identify gaps
5. Evaluate coupling with other units
6. Produce a structured analysis with risk assessment and recommendations

### Example

```json
{
  "name": "analyse_unit",
  "arguments": {
    "unit_name": "UserService",
    "graph": "myproject"
  }
}
```

## `explain_coupling`

Explain coupling between two code units.

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `unit_a` | string | Yes | First unit name |
| `unit_b` | string | Yes | Second unit name |
| `graph` | string | No | Graph name |

### Behavior

The prompt instructs the agent to:

1. Look up both units in the code graph
2. Find all direct and transitive connections between them
3. Classify the coupling type (data, control, structural, temporal)
4. Assess the coupling strength and directionality
5. Identify potential risks if one unit changes
6. Suggest decoupling strategies if coupling is excessive

### Example

```json
{
  "name": "explain_coupling",
  "arguments": {
    "unit_a": "AuthController",
    "unit_b": "UserRepository",
    "graph": "myproject"
  }
}
```
