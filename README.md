# 🎯 Vibe Coding Template

## What is Vibe Coding?

Vibe Coding is a systematic approach to software development designed specifically for working with LLMs (Large Language Models). It addresses the key challenge of LLMs: limited context windows and lack of persistent memory between sessions.

## 🚀 Quick Start

1. **Copy this template folder** to your new project directory
2. **Open the project in Cursor** (or your preferred LLM-powered editor)
3. **Let the LLM read `.cursorrules`** first
4. **Start the discovery process** by having the LLM read `task_on_hand.md`
5. **Answer the questions** posed by the LLM
6. **Watch the documentation evolve** as your project takes shape

## 📁 Template Contents

### `.cursorrules`

The algorithm that tells the LLM how to use all other documents:

- Step-by-step instructions for reading documents
- Rules for updating each document type
- The discovery and development workflow
- Critical rules to prevent context loss

### `task_on_hand.md`

The main working document for discovery and progress:

- Current task and status
- Open questions to ask
- Progress tracking with checkboxes
- Immediate next steps

### `project_context.md`

Empty template for business/product information:

- Project overview and vision
- Users and stakeholders
- Business requirements
- External dependencies

### `technical_details.md`

Empty template for technical decisions:

- Technology stack choices
- Architecture decisions
- Security approach
- Development setup

### `development_log.md`

Empty template for append-only progress log:

- Timestamped entries
- Decisions and rationale
- Changes and their impact
- Never modified, only appended

## 🔄 The Process

### 1. Discovery Phase

The LLM will ask you questions about:

- Project goals and objectives
- Technical requirements
- Constraints and preferences
- Success criteria

### 2. Documentation Evolution

As you answer questions:

- `task_on_hand.md` gets updated with specifics
- `project_context.md` is created with permanent info
- Technical decisions are documented

### 3. Development Phase

Once understanding is complete:

- Clear implementation plan emerges
- Progress is tracked with checkboxes
- Each session builds on previous work

## 💡 Key Benefits

1. **No Context Loss** - Everything is documented
2. **Seamless Handoffs** - Any LLM can continue the work
3. **Clear Progress** - Always know what's been done
4. **Systematic Approach** - No important details missed
5. **Self-Improving** - Documentation evolves with understanding

## 🎯 Best Practices

1. **Always start by reading `.cursorrules`**
2. **Update documentation immediately** after decisions
3. **Use checkboxes** for progress tracking
4. **Ask questions** when uncertain
5. **Document rationale** for decisions

## 🚫 Common Mistakes

1. **Jumping into coding** before understanding requirements
2. **Not updating documentation** as you go
3. **Assuming the LLM remembers** previous sessions
4. **Skipping the question phase**
5. **Not using checkboxes** for progress

## 📝 Example Usage

```
You: "I want to build a task management app"

LLM: *Reads .cursorrules and task_on_hand.md*
"I see you want to build a task management app. Let me ask some clarifying questions:

1. What is the main problem this task management app aims to solve?
2. Who are the primary users (individuals, teams, enterprises)?
3. What makes this different from existing solutions?
..."

You: *Answers questions*

LLM: *Updates task_on_hand.md with your answers and asks follow-up questions*
```

## 🔧 Customization

Feel free to:

- Add more template documents
- Modify the question categories
- Adjust the process to your workflow
- Add project-specific sections

The key is maintaining the core principle: **Document everything for context preservation**.

## 🤝 Contributing

If you develop improvements to the vibe coding system:

1. Document what worked well
2. Note what could be improved
3. Share your enhanced templates
4. Help evolve the methodology

---

**Remember**: The goal is to create a sustainable development flow that works within LLM limitations while maximizing productivity and code quality.
