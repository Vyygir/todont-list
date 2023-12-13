use std::io;
use std::io::Write;

mod item;
mod task_list;
mod task_interface;

use crate::task_list::TaskList;
use crate::task_interface::{TaskInterface, TaskInterfaceAction, TaskInterfaceActionMapItem};

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().expect("Couldn't flush buffer");
}

fn clean_user_input(input: String) -> String {
    // @todo This is kinda dirty so it needs cleaning up at some point.
    input.strip_suffix("\r\n").unwrap().to_owned()
}

fn setup_actions(interface: &mut TaskInterface) {
    interface.add_action(TaskInterfaceActionMapItem {
        command: String::from("list"),
        action: TaskInterfaceAction::List,
    });

    interface.add_action(TaskInterfaceActionMapItem {
        command: String::from("add"),
        action: TaskInterfaceAction::Add,
    });

    interface.add_action(TaskInterfaceActionMapItem {
        command: String::from("update"),
        action: TaskInterfaceAction::Update,
    });

    interface.add_action(TaskInterfaceActionMapItem {
        command: String::from("complete"),
        action: TaskInterfaceAction::Complete,
    });

    interface.add_action(TaskInterfaceActionMapItem {
        command: String::from("delete"),
        action: TaskInterfaceAction::Delete,
    });

    interface.add_action(TaskInterfaceActionMapItem {
        command: String::from("quit"),
        action: TaskInterfaceAction::Quit,
    });
}

fn main() {
    clear_screen();

    let task_list: TaskList = TaskList::new();
    let mut task_interface: TaskInterface = TaskInterface::new(task_list);

    setup_actions(&mut task_interface);

    loop {
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Couldn't read input");
        task_interface.process_input(user_input);
    }
}
