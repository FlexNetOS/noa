# Interactive Mode

Master the DSPy Code interactive environment for efficient DSPy development.

## What is Interactive Mode?

Interactive Mode is the **primary interface** for DSPy Code. It provides:

- Natural language code generation
- Conversational workflow
- Context-aware responses
- Session persistence
- Real-time validation

**Start interactive mode:**

```bash
dspy-code
```

## The Welcome Screen

When you start DSPy Code, you see:

```
✓ DSPy Version: 3.0.4

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║            ██████╗ ███████╗██████╗ ██╗   ██╗     ██████╗██╗     ██╗          ║
║            ██╔══██╗██╔════╝██╔══██╗╚██╗ ██╔╝    ██╔════╝██║     ██║          ║
║            ██║  ██║███████╗██████╔╝ ╚████╔╝     ██║     ██║     ██║          ║
║            ██║  ██║╚════██║██╔═══╝   ╚██╔╝      ██║     ██║     ██║          ║
║            ██████╔╝███████║██║        ██║       ╚██████╗███████╗██║          ║
║            ╚═════╝ ╚══════╝╚═╝        ╚═╝        ╚═════╝╚══════╝╚═╝          ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
                    ✨ Your AI-Powered DSPy Development Assistant: Claude Code for DSPy ✨

⚡ Get Started:

  /intro    - Complete guide & all features
  /init     - Initialize your project
  /demo     - See it in action
  /help     - View all commands

💬 Or just describe what you want to build in natural language!

💡 New here? Try /demo or /init to get started

🚀 Ready! New here? Try /intro for the complete guide!

💭 Try: "Create a signature for..." or "Build a module using..." or "/help" for
commands

╭────────────────────────────── ✨ Your Message ───────────────────────────────╮
│ Type your request here... (or /help for commands)                            │
╰──────────────────────────────────────────────────────────────────────────────╯
  →
```

**Key information displayed:**

- ✅ Your DSPy version
- ✅ Quick start commands
- ✅ Usage hints
- ✅ Input prompt

## Basic Usage

### Natural Language Requests

Simply describe what you want:

```
Build a module for sentiment analysis
```

```
Create a signature for email classification with subject and body as inputs
```

```
Generate a complete RAG system for document Q&A
```

DSPy Code understands your intent and generates appropriate code!

### Slash Commands

Use `/` prefix for specific commands:

```
/help
/init
/model
/save my_module.py
/validate
/run
/status
```

[See all slash commands →](slash-commands.md)

### Mixed Interaction

Combine natural language and commands:

```
→ Create a sentiment analyzer
  [Code generated]

→ /save sentiment.py
  ✓ Saved!

→ Add error handling to it
  [Code updated]

→ /validate
  ✓ Valid!

→ /run
  [Execution results]
```

## Session Context

### What's in Context?

The session maintains:

**1. Generated Code:**

```python
context['last_generated'] = "..."
context['type'] = 'module'
```

**2. Generated Data:**

```python
context['last_generated_data'] = [...]
context['data_task'] = 'sentiment'
```

**3. Conversation History:**

```python
context['history'] = [
    {'role': 'user', 'content': '...'},
    {'role': 'assistant', 'content': '...'}
]
```

**4. Model Connection:**

```python
context['model'] = {
    'provider': 'ollama',
    'model': 'llama3.1:8b',
    'connected': True
}
```

### Check Context

Use `/status` to see what's in context:

```
/status
```

**Output:**

```
Current Session Status:

✓ Generated Code: Module (1,234 chars)
  Use /save <filename>, /validate, /run

✗ No Generated Data
  Try: "Generate 20 examples for sentiment analysis"

✓ Model Connected: llama3.1:8b (ollama)

✓ Project Initialized: my-dspy-project
  Use /init to re-index or update

✓ Codebase Index Loaded: 3,421 elements
```

### Clear Context

Clear conversation history:

```
/clear
```

**What's cleared:**

- Conversation history
- Generated code
- Generated data

**What's kept:**

- Model connection
- Project configuration
- Codebase index

## Conversation Flow

### Typical Workflow

**1. Initialize project:**

```
→ /init
  [Project setup]
```

**2. Connect model:**

```
→ /model
  [Pick Ollama vs cloud, then choose a model]
  ✓ Connected!
```

**3. Generate code:**

```
→ Create a text classifier
  [Code generated]
```

**4. Save code:**

```
→ /save classifier.py
  ✓ Saved!
```

**5. Validate:**

```
→ /validate
  ✓ Valid!
```

**6. Test:**

```
→ /run
  [Results]
```

**7. Iterate:**

```
→ Add confidence scores to the output
  [Code updated]

→ /save classifier.py
  ✓ Updated!
```

### Iterative Development

Refine code through conversation:

```
→ Create a sentiment analyzer
  [Initial code]

→ Use ChainOfThought instead of Predict
  [Updated with ChainOfThought]

→ Add a confidence score output field
  [Added confidence field]

→ Add error handling for empty input
  [Added validation]

→ /save sentiment_analyzer.py
  ✓ Saved!
```

### Multi-Step Tasks

Break complex tasks into steps:

```
→ I want to build a RAG system

  Great! Let's break this down:
  1. Create retrieval signature
  2. Create generation signature
  3. Build RAG module
  4. Add example usage

  Ready to start?

→ Yes, start with step 1

  [Retrieval signature generated]

→ Looks good, next step

  [Generation signature generated]

→ Continue

  [RAG module generated]

→ /save rag_system.py
  ✓ Complete RAG system saved!
```

## Advanced Features

### Code Refinement

Ask for specific improvements:

```
→ Optimize the last generated code for performance
→ Add type hints to all functions
→ Add docstrings following Google style
→ Refactor to use fewer API calls
→ Make it more modular
```

### Code Explanation

Ask questions about generated code:

```
→ Explain how the ChainOfThought works in this code
→ What does the forward method do?
→ Why did you use this signature structure?
```

### DSPy Concepts

Learn while you code:

```
→ What's the difference between Predict and ChainOfThought?
→ When should I use ReAct?
→ How do I optimize this module?
→ What are best practices for signatures?
```

### Project-Specific Questions

Ask about your codebase:

```
→ What modules do I have in my project?
→ Show me my existing signatures
→ How does my RAG module work?
→ What's the structure of my project?
```

## Session Management

### Save Sessions

Save your work:

```
/session save my-rag-project
```

**What's saved:**

- Generated code
- Training data
- Conversation history
- Model configuration
- Project context

### Load Sessions

Resume previous work:

```
/session load my-rag-project
```

**What's restored:**

- All generated code
- Training data
- Conversation history
- Model connection
- Project state

### List Sessions

See all saved sessions:

```
/sessions list
```

**Output:**

```
Saved Sessions:

1. my-rag-project
   Created: 2025-01-15 14:30
   Last modified: 2025-01-15 16:45
   Size: 45 KB

2. sentiment-analyzer
   Created: 2025-01-14 10:20
   Last modified: 2025-01-14 12:30
   Size: 23 KB

3. email-classifier
   Created: 2025-01-13 09:15
   Last modified: 2025-01-13 11:00
   Size: 31 KB
```

### Delete Sessions

Remove old sessions:

```
/session delete old-project
```

## Tips and Tricks

### 1. Use Tab Completion

Press `Tab` to autocomplete:

- Slash commands
- File names
- Model names
- Server names

### 2. Use History

Press `↑` to cycle through previous commands.

### 3. Multi-Line Input

For long requests, use natural line breaks:

```
→ Create a module that:
  - Takes a question and context
  - Uses ChainOfThought for reasoning
  - Returns an answer with confidence
  - Handles edge cases
```

### 4. Quick Edits

Reference previous code:

```
→ Create a classifier
  [Code generated]

→ Change the predictor to ReAct
  [Updated]

→ Add another output field called "reasoning"
  [Updated]
```

### 5. Batch Operations

Do multiple things at once:

```
→ Create a sentiment analyzer, validate it, and save it as sentiment.py
  [Generated, validated, saved]
```

### 6. Use Examples

Provide examples for better results:

```
→ Create a classifier that categorizes:
  - "I love this!" → positive
  - "This is terrible" → negative
  - "It's okay" → neutral
```

### 7. Ask for Alternatives

Get different approaches:

```
→ Show me 3 different ways to implement this
→ What's a simpler version?
→ What's a more advanced version?
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Cancel current operation |
| `Ctrl+D` | Exit DSPy Code |
| `↑` / `↓` | Navigate history |
| `Tab` | Autocomplete |
| `Ctrl+L` | Clear screen |

## Visual Feedback

### Progress Indicators

Watch for visual feedback:

**Thinking:**

```
⚙️  Generating your DSPy module...
```

**Success:**

```
✓ Module created successfully!
```

**Error:**

```
✗ Validation failed: Missing InputField
```

**Warning:**

```
⚠️  DSPy version is old. Consider upgrading.
```

### Code Display

Generated code is beautifully formatted:

```python
╭────────────────────────── Generated DSPy Module ──────────────────────────╮
│ import dspy                                                                │
│                                                                            │
│ class SentimentSignature(dspy.Signature):                                 │
│     """Analyze sentiment of text."""                                      │
│     text = dspy.InputField(desc="Text to analyze")                        │
│     sentiment = dspy.OutputField(desc="positive, negative, or neutral")   │
│                                                                            │
│ class SentimentAnalyzer(dspy.Module):                                     │
│     def __init__(self):                                                   │
│         super().__init__()                                                │
│         self.predictor = dspy.Predict(SentimentSignature)                 │
│                                                                            │
│     def forward(self, text):                                              │
│         return self.predictor(text=text)                                  │
╰────────────────────────────────────────────────────────────────────────────╯
```

### Next Steps

After generation, see suggested actions:

```
Next Steps:
• Type /save <filename> to save this code
• Type /validate to check for errors
• Type /run to test execution
• Type /status to see what's in context
• Ask me to create a complete program
• Request optimizations or improvements
```

## Common Patterns

### Pattern 1: Quick Prototype

```
→ /init
→ /connect ollama llama3.1:8b
→ Create a [task] module
→ /save module.py
→ /run
```

### Pattern 2: Iterative Development

```
→ Create a basic [task] module
→ Add [feature]
→ Improve [aspect]
→ /validate
→ /save module.py
```

### Pattern 3: Full Workflow

```
→ /init
→ /connect [model]
→ Create [program]
→ /save program.py
→ Generate 50 examples for [task]
→ /save-data examples.jsonl
→ /optimize program.py examples.jsonl
```

### Pattern 4: Learning Mode

```
→ What is [concept]?
→ Show me an example of [feature]
→ Create a simple [task] using [technique]
→ Explain how this works
```

## Troubleshooting

### Command Not Working

**Check spelling:**

```
/hlep  ✗
/help  ✓
```

**Use `/help` to see all commands**

### Code Not Saving

**Check context:**

```
/status
```

**Generate code first if context is empty**

### Model Not Responding

**Check connection:**

```
/status
```

**Reconnect if needed:**

```
/disconnect
/connect ollama llama3.1:8b
```

### Unexpected Behavior

**Clear context and restart:**

```
/clear
```

**Or exit and restart:**

```
/exit
dspy-code
```

## Best Practices

### 1. Start with /init

Always initialize your project:

```
/init
```

This builds the codebase index!

### 2. Check /status Often

Know what's in context:

```
/status
```

### 3. Save Frequently

Don't lose your work:

```
/save module.py
/session save my-project
```

### 4. Validate Before Running

Catch errors early:

```
/validate
/run
```

### 5. Be Specific

Better requests = better code:

```
Good: "Create a ChainOfThought module for sentiment analysis with text input and sentiment output"

Bad: "Make sentiment thing"
```

### 6. Iterate

Refine through conversation:

```
→ Create module
→ Add feature
→ Improve aspect
→ Fix issue
```

### 7. Use Natural Language

Don't overthink it:

```
→ Make it better
→ Add error handling
→ Simplify this
→ Optimize for speed
```

## Summary

Interactive Mode provides:

- ✅ Natural language interface
- ✅ Context-aware responses
- ✅ Session persistence
- ✅ Real-time validation
- ✅ Iterative development
- ✅ Learning support

**Master these concepts:**

- Session context
- Slash commands
- Natural language requests
- Iterative refinement
- Session management

[Learn All Slash Commands →](slash-commands.md){ .md-button .md-button--primary }
[Build Your First Program →](./../getting-started/first-program.md){ .md-button }
