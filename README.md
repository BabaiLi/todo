# todo: A command line to-do app written in Rust
---

![image](https://raw.githubusercontent.com/BabaiLi/todo/main/media/todoo.gif)

# usage
---

```
A command line to-do app written in Rust
Available commands:
    - add [TASK]
        adds new tasks
        Example: todo add "giant panda" "polar bear" "brown bear"
    - list
        lists all tasks
        Example: todo list
    - done
        marks task as done which state is 1
        Example: todo done
    - clear
        clear all tasks
        Example: todo clear
    - update [INDEX] [STATE]
        update task state, 
            * 0 for new/incomplete, and font color will be green
            * 1 for complete, and font color will be red
            * 2 for suspend, and font color will be cyan
        Example: todo update 1 1
                 // make task 1 be complete
```
