@@lesson g-start How Guided Builds Work
@@text
Welcome to the **Guided Builds**! The course taught you the pieces. Here you put them together into four real apps you'll actually want to use:

| Build | What it is | What you practise |
|---|---|---|
| 🍅 **Focus Timer** | a Pomodoro timer with a progress ring, task log and weekly chart | state machines, `Instant`/`Duration`, timers in a GUI, drawing with the painter, saving JSON |
| 📝 **Note Vault** | a notes app: Markdown notes as plain files, search, #tags, live preview | files and folders, `HashSet`/`BTreeMap`, autosave, crates |
| 🗂 **File Organizer** | tidies a messy folder: sorts files into folders by type, with a preview and **undo** | `std::fs` and paths, careful error handling, planning before acting |
| 💬 **Local AI Chat** | your own desktop chat app for an AI model running on your computer | HTTP and JSON APIs, worker threads and channels, streaming |

You can do them in any order, but they're arranged from easiest to hardest. You should have finished the course up to **Module 9** (at least through "Timers and Background Work") first.

## Side by side

In the builds, the coding happens in **VS Code**, not in this app. Put the two windows side by side:

1. Open a build's project with the **Open in VS Code** button on its project card.
2. Press **Win + ←** in Rust Quest, and **Win + →** in VS Code. Windows snaps them into two halves.
3. Read a part of the lesson on the left, write the code on the right, run it with `cargo run` in VS Code's terminal (**Ctrl + `**), and repeat.

To make room, the in-app code editor is hidden in the builds. If you'd like it back as a scratch pad, use **Show editor** at the top.

## How each build works

Each build has three or four steps. Each step:

- explains **what** you're going to build and **why** it's designed that way,
- gives you a **project card** with the starting point for that step. The first step creates the project. In later steps, if you followed along, keep going with your own code: **Add new file(s)** adds only the files that step introduces and never touches yours. If you got stuck or want a clean start, open **This step's starting files** and use **Reset project to this step's files** (your old files are backed up first, never lost).
- shows the code to write, piece by piece, with explanations. **Type it yourself** rather than copy-pasting: you'll understand and remember far more, and you'll make (and fix) small mistakes, which is how everyone learns.
- has `todo!()`s for you to fill in on your own, checked by `cargo test`.
- ends with **ideas to take it further**. That's where it becomes *your* app.

The last step of every build also has a card with the **finished app**, in its own folder, so you can compare with your version or just use it.

## Getting help from an AI model (optional)

Stuck? Rust Quest can connect to an **AI model running on your own computer**. It's free, works offline, and your code never leaves your PC. Click **🤖 AI Helper** at the top right. With "Include my code & errors" ticked, it sees the lesson you're on, and for project steps, the files of your project on disk, so you can just ask "why doesn't my `streak` function work?".

### With LM Studio (easiest)

1. Download **LM Studio** from lmstudio.ai and install it.
2. In LM Studio, open **Discover** (the magnifying glass) and download a coding model. Good choices:
   - **Qwen2.5 Coder 7B Instruct** (or a newer Qwen Coder), if your PC has 16 GB of RAM or a graphics card with 8 GB.
   - **Qwen2.5 Coder 3B** or **Llama 3.2 3B** for smaller machines (faster, but less clever).
3. Open the **Developer** tab (the `>_` icon), select your model at the top to load it, and switch **Status: Running** on. It now listens at `http://localhost:1234`.
4. In Rust Quest's AI Helper, choose **LM Studio**, press **Connect**, and ask away.

### With Ollama

1. Install **Ollama** from ollama.com.
2. In a terminal: `ollama pull qwen2.5-coder:7b` (or `qwen2.5-coder:3b` for smaller machines).
3. Ollama runs in the background automatically. In the AI Helper choose **Ollama**, then **Connect**.

llama.cpp's `llama-server` and Jan work too: choose them, or type the address of any server that speaks the "OpenAI-compatible" API.

### Using it well

- By default the helper is set to **"Hints, not full solutions"**. Keep it on while you're learning: being *told* the answer feels like progress, but you learn by *finding* it.
- Small local models make mistakes. Treat answers like advice from a helpful classmate: try them, and believe the compiler over the AI.
- Ask specific questions: "what does error E0502 mean on line 40?" works better than "it doesn't work".

You don't need an AI at all to do the builds: the lessons, hints and finished code have everything.

Ready? Start with the **Focus Timer**.
