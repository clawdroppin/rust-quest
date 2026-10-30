# Rust Quest — User Guide

Rust Quest teaches you to program from zero, in Rust, until you can build your own desktop apps.
It's one portable `.exe`: no installer and no account. Everything runs on your computer.

## 1. Install (about 10 minutes, once)

1. **Download Rust Quest.** Get `RustQuest.exe` from the
   [latest release](https://github.com/clawdroppin/rust-quest/releases/latest). Put it in its own
   folder, for example `Documents\RustQuest\`. It saves your progress and projects next to itself.
2. **Install Rust.** Go to [rustup.rs](https://rustup.rs), download `rustup-init.exe` and run it.
   - If it says you need the *Visual Studio C++ Build Tools*, choose the option that installs them for you
     (option 1). Rust needs them to build programs on Windows.
   - When it's done, **close and reopen** anything that should see Rust (Rust Quest, VS Code, terminals).
3. **Install VS Code** (needed from Module 6 on, and for the Guided Builds) from
   [code.visualstudio.com](https://code.visualstudio.com). In VS Code, open Extensions
   (`Ctrl + Shift + X`) and install **rust-analyzer**.

Start `RustQuest.exe`. At the top right you should see a green **✓ rustc 1.xx**. If it's red, see
[Troubleshooting](#7-troubleshooting).

> **"Windows protected your PC"?** The exe isn't code-signed (signing costs money), so SmartScreen
> warns about it. Click **More info → Run anyway**. You can also build it yourself from source (see the README).

## 2. The screen

| Left | Middle | Right |
|---|---|---|
| The course map: modules and lessons. A ✓ means done. | The lesson: explanations, examples, quick quizzes. | The code editor, with **Exercise** and **Playground** tabs. |

- **Course / Guided Builds** (top of the sidebar) switches between the main course and the bigger projects.
- **A− / A+** make all text smaller or bigger.
- **🤖 AI Helper** opens the optional local AI assistant (see section 5).

## 3. Lessons and exercises

- Read the lesson slowly. Code examples have a **▶ Try it** button that opens them in the
  **Playground**, a free scratch space where nothing can go wrong.
- Answer the **quick checks** as you go; they tell you why an answer is right or wrong.
- Each lesson ends with exercises:
  - **▶ Run** (`Ctrl + Enter` or `F5`) compiles and runs your code with the real Rust compiler.
  - **✓ Check** (`Ctrl + Shift + Enter`) runs it *and* tests whether it solves the exercise.
    Passing marks the exercise done.
  - **💡 Hint** gives a nudge. **Solution** shows one answer. Try for a while before you use them:
    that struggle is where the learning happens.
  - **Reset code** brings back the starting code.
  - **Program input (stdin)** under the editor lets you type input for programs that read it.
- Your code and progress are saved automatically, so you can close the app at any time.

## 4. Projects in VS Code (Module 6 onwards)

From Module 6, lessons contain **project cards**: real Cargo projects you work on in VS Code.

1. Press **Create project** on the card. Rust Quest writes it to `RustQuestProjects\<name>`
   next to the exe.
2. Press **Open in VS Code**. Open VS Code's terminal with ``Ctrl + ` `` and use:
   - `cargo run` to build and run,
   - `cargo test` to run the tests that check your work.

   (The card's **▶ cargo run** and **🧪 cargo test** buttons do the same in a new terminal window.)
3. Fill in the `todo!()`s until the tests pass.

Other buttons on a card:

- **Add N new file(s)** adds files a later step introduces. It never overwrites your files.
- **Reset project to this step's files** (under "This step's starting files") gives you a clean start.
  Your old files are first copied to `RustQuestProjects\_backups\`, never deleted.
- **Open folder** shows the project in Explorer.

The very first build of a GUI project downloads and compiles its libraries, which can take a few minutes.
Later builds are fast.

## 5. Guided Builds

The **Guided Builds** tab has four bigger apps, built step by step with Rust Quest and VS Code side by side
(`Win + ←` on one window, `Win + →` on the other):

| Build | What you make |
|---|---|
| 🍅 Focus Timer | a Pomodoro timer with a progress ring, task log, streak and weekly chart |
| 📝 Note Vault | a Markdown notes app with search, #tags, live preview and autosave |
| 🗂 File Organizer | tidies a messy folder into subfolders, with a preview and undo |
| 💬 Local AI Chat | your own chat app for an AI model running on your computer |

Finish the course up to Module 9 first. Start with **Start Here → How Guided Builds Work**. Each build
ends with a card holding the finished app so you can compare it with yours.

## 6. AI Helper (optional)

Rust Quest can talk to an AI model running **on your own computer**. It's free and works offline, and
your code never leaves your PC.

**With LM Studio (easiest)**

1. Install [LM Studio](https://lmstudio.ai).
2. In **Discover**, download a coding model: *Qwen2.5 Coder 7B Instruct* if you have 16 GB RAM or an
   8 GB graphics card; *Qwen2.5 Coder 3B* or *Llama 3.2 3B* for smaller machines.
3. In the **Developer** tab, load the model and switch the server to **Running**.
4. In Rust Quest: **🤖 AI Helper → LM Studio → Connect**.

**With Ollama:** install [Ollama](https://ollama.com), run `ollama pull qwen2.5-coder:7b`, then choose
**Ollama → Connect**. llama.cpp's `llama-server`, Jan, or any other "OpenAI-compatible" server work too:
type its address in **Server**.

Tips:

- With **Include my code & errors** ticked, the helper sees the current lesson, your code, the last
  output, and for project steps, your project's files. So you can just ask "why doesn't this compile?".
- The quick buttons (**Explain my error**, **Give me a hint**, **Review my code**, **Explain this lesson
  simply**) are good starting points.
- **Hints, not full solutions** is on by default. Leave it on while you learn.
- Small local models make mistakes. Trust the compiler over the AI.

## 7. Troubleshooting

| Problem | Fix |
|---|---|
| Red "rustc not found" | Install Rust from rustup.rs, then restart Rust Quest. It looks on PATH and in `%USERPROFILE%\.cargo\bin`. |
| `linker 'link.exe' not found` | The Visual Studio C++ Build Tools are missing. Re-run `rustup-init.exe` and let it install them, or install "Desktop development with C++" from the Visual Studio Installer. |
| "Open in VS Code" does nothing | Install VS Code and make sure `code` is on PATH (the installer's "Add to PATH" option), then restart Rust Quest. |
| First `cargo run` of a GUI project is slow | Normal: it's compiling the GUI library once. Later runs take seconds. |
| Antivirus blocks a compiled program | Freshly compiled exes sometimes trigger it. Allow the `RustQuestProjects` folder, or the temp folder Rust Quest compiles in. |
| AI Helper "can't connect" | Make sure the server is running (LM Studio: Developer tab → Running) and the address matches. Press **Connect** again. |

## 8. Where your data lives

- **Progress:** `rust_quest_progress.json` next to the exe (or `%APPDATA%\RustQuest\` if that folder
  isn't writable). Copy it to move your progress to another PC. Delete it to start over.
- **Projects:** `RustQuestProjects\` next to the exe (or in `Documents` if that folder isn't writable).
- The app makes no network connections, except to the AI server address **you** configure.
