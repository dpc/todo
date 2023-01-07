# dpc-todo

dpc's "productivity" system

## Introduction

I'm not much of a TODO" list person. I think managing these is not worth it - takes too much time, where my email is already a natural TODO list. However, I do somethimes want to remind myself about something I need to do. Usually when I'm in the middle of writting code, etc.

So my productivity system is simple and built around that.

I have a file in a directory that syncs accross machines over `syncthing`.

I make my machines auto-open that file when I start the desktop environment with `~/.config/autostart/todo.desktop`:

```
[Desktop Entry]
Name=todo
Comment=todo
Exec=xdg-open /home/dpc/todo.md
StartupNotify=false
Terminal=false
Type=Application
```
This is mostly to give me a reminder about the existance of the TODO list.

In my shell config I set `export TODO_FILE_PATH=$HOME/todo.md`.

Whenever I have something that I want to records in my TODO list, I use the command from this repo and run:

```
todo Fix that stupid bug in project foo.
```

This adds that entry on top of my TODO file.

When I want to check/edit my TODO list manually I run:

```
todo --open
```

And that's it.
