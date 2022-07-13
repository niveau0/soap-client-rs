# AI Assistant Configuration

> **Note**: This file was renamed from `README.md` to `AI_ASSISTANTS.md` to avoid conflicting with GitHub's repository README display. GitHub prioritizes `.github/README.md` over the root `README.md`, so we use a different name here.

This directory contains configuration files for AI assistants working with this project.

## Files

### `.github/copilot-instructions.md`
**Purpose**: Coding guidelines for GitHub Copilot's code completion

**Used by**: GitHub Copilot IDE extension (while you type)

**Contains**:
- Code style and naming conventions
- Error handling patterns
- Testing guidelines
- Documentation standards
- Project-specific patterns
- Security considerations

**When to update**: When coding conventions, patterns, or standards change

**Example use case**: Copilot uses these instructions to generate code that follows our project's style (e.g., using `snake_case` for functions, preferring `Result<T, E>` over panics).

---

### `.github/copilot-chat-context.md`
**Purpose**: Project context for chat-based AI assistants

**Used by**: 
- GitHub Copilot Chat
- Claude
- ChatGPT
- Other chat-based coding assistants

**Contains**:
- Current project status and architecture
- Recent changes and decisions
- What works and what doesn't
- Common tasks and debugging tips
- Known issues and constraints
- Development workflow

**When to update**: When making significant changes:
- Architecture refactorings
- New features or capabilities
- Changed conventions
- Important decisions
- New constraints

**Example use case**: At the start of a new chat session, read this file to understand the current state of the project, recent changes, and what's currently being worked on.

---

## Usage

### For AI Assistants

**GitHub Copilot (IDE)**:
- Automatically reads `copilot-instructions.md` for code suggestions
- No manual action needed

**Chat Sessions (Copilot Chat, Claude, ChatGPT)**:
```
Please read .github/copilot-chat-context.md to understand the project state.
```

Or to understand this directory:
```
Please read .github/AI_ASSISTANTS.md for an overview of AI assistant files.
```

### For Contributors

**When you make changes**:

1. **Code conventions changed?** → Update `copilot-instructions.md`
   - New patterns introduced
   - Different error handling approach
   - Changed module organization

2. **Architecture changed?** → Update `copilot-chat-context.md`
   - Major refactorings (like parser split)
   - New features added
   - Design decisions made
   - Important constraints discovered

3. **Don't forget to update the "Last Updated" date** in the file

---

## Benefits

✅ **Consistency**: AI assistants follow the same conventions across sessions
✅ **Context Preservation**: Chat sessions can quickly resume where you left off
✅ **Onboarding**: New AI assistant sessions understand the project faster
✅ **Documentation**: Decisions and rationale are preserved
✅ **Quality**: Generated code follows project standards

---

## Best Practices

### copilot-instructions.md
- Keep it focused on **how to write code**
- Include examples of good patterns
- Explain **why** conventions exist, not just **what** they are
- Update when you notice Copilot generating non-idiomatic code

### copilot-chat-context.md
- Keep it focused on **current state**
- Don't duplicate information from README.md or other docs
- Include **context** that helps with decisions, not just facts
- Update "Recent Changes" section with significant events
- Include **why** decisions were made, not just what was done

---

**Last Updated**: 2024-12-17