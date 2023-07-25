# todo
---

![image](https://raw.githubusercontent.com/BabaiLi/todo/main/media/todo.gif)

# usage
---

```
A command line to-do app written in Rust
Available commands:
    - add [TASK]
        adds new task
        Example: todo add "buy carrots"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2
    - clear
        clear all tasks
        Example: todo clear
```
    - update [INDEX] [STATUD]
        update task status, 
            0 for new/incomplete, and font color will be green
            1 for complete, and font color will be red
            2 for suspend, and font color will be cyan
        Example: todo update 1 1
                 // make task 1 be complete
