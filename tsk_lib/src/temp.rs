#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct List {
    pub tasks: Vec<Task>,
}

impl IntoIterator for List {
    type Item = Task;
    type IntoIter = ::std::vec::IntoIter<Task>;

    fn into_iter(self) -> Self::IntoIter {
        self.tasks.into_iter()
    }
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(jsn) => write!(f, "{}", jsn),
            Err(_) => write!(f, "ERROR: Unable to serialize list"),
        }
    }
}

impl List {
    pub fn new(tasks: Vec<Task>) -> List {
        let mut list = List { tasks: tasks };

        list.tasks.sort();
        list
    }

    /// Return a reference to the "current" task.
    pub fn current<'a>(&'a mut self) -> Option<&'a mut Task> {
        let mut ind = 0;
        for (task_id, task) in self.enumerate() {
            if !task.completed() {
                ind = task_id;
                break;
            }
        }

        self.find_by_ind(ind)
    }

    /// Add a task to the List, will sort after adding.
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
        self.tasks.sort();
    }

    /// Add multiple tasks to the List, this is more efficient than calling add multiple times
    /// since only one sort is performed. It will empty the given vector.
    pub fn add_multiple(&mut self, tasks: &mut Vec<Task>) {
        self.tasks.append(tasks);
        self.tasks.sort();
    }

    /// Return a reference to the task at the given ID / index.
    pub fn find_by_ind<'a>(&'a mut self, id: usize) -> Option<&'a mut Task> {
        if self.tasks.len() < id {
            return None;
        }

        let t: &'a mut Task = &mut self.tasks[id];
        Some(t)
    }

    /// Return a reference to the first task with the given title.
    pub fn find_by_title<'a>(&'a mut self, title: &str) -> Option<&'a mut Task> {
        let mut ind = None;
        for (i, task) in self.enumerate() {
            if task.title == title {
                ind = Some(i);
                break;
            }
        }

        match ind {
            Some(i) => self.find_by_ind(i),
            None => None,
        }
    }

    /// Return an enumerated iterator over the tasks in this list.
    pub fn enumerate(&self) -> iter::Enumerate<slice::Iter<Task>> {
        self.tasks.iter().enumerate()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn context(&self, context: &str) -> List {
        List::new(
            self.tasks
                .iter()
                .filter(|x| &x.context == context)
                .map(|x| x.clone())
                .collect(),
        )
    }

    pub fn search<'a>(&'a mut self, query: AST) -> Vec<&'a Task> {
        match query.expression {
            Expression::StrLiteral(Token::Str(s)) => self
                .tasks
                .iter()
                .filter(|t| match t.body.as_ref() {
                    Some(body) => t.title.contains(&s) || body.contains(&s),
                    None => t.title.contains(&s),
                })
                .collect(),
            exp => self.eval(exp),
        }
    }

    fn eval<'a>(&'a mut self, exp: Expression) -> Vec<&'a Task> {
        self.tasks.iter().filter(|_t| true).collect()
    }
}

#[cfg(test)]
mod tests {}
