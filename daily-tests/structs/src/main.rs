#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
    priority: u8,
}

impl Task {
    fn new(title: String, done: bool, priority: u8) -> Self {
        Self { title, done, priority }
    }

    fn complete(&mut self) {
        self.done = true;
    }

    fn label(&self) -> String {
        let status = if self.done {"✅"} else {"❌"};
        format!("[{}] {} (priority {})", status, self.title, self.priority)
    }
}

struct Project {
    name: String,
    tasks: Vec<Task>,
}

impl Project {
    fn new(name: String) -> Self {
        Self { name, tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remaining(&self) -> usize {
        self.tasks.iter()
            .filter(|t| !t.done)
            .count()
    }
}

fn main() {
    let t1 = Task::new(String::from("Learn Rust"), true, 1u8);
    let t2 = Task::new(String::from("Previous lessons"), true, 2u8);
    let mut t3 = Task::new(String::from("Practice structs"), false, 3u8);
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
    println!("{}", t3.label());
    t3.complete();
    println!("{:?}", t3);
    println!("{}", t3.label());

    do_bonus();
}

fn do_bonus() {
    let mut proj = Project::new(String::from("Learn Rust"));
    proj.add_task(Task::new(String::from("structs"), false, 1));
    proj.add_task(Task::new(String::from("Enums"), false, 2));
    proj.add_task(Task::new(String::from("Traits"), false, 3));

    proj.tasks[0].complete();

    println!("{} remaining", proj.remaining());
}
