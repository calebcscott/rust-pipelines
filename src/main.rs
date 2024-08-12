



// Idea, pipeline is made up of tasks
// each task should have input/output
//

use std::{collections::HashMap, error::Error, fmt};


trait Task {
    fn run(&self, msg: &[HashMap<String, String>]) ->  Result< Vec<HashMap<String, String>>, Box<dyn Error> >;
}


struct BasicTask {
    id: String,
    f: Box<dyn Fn(&[HashMap<String, String>]) -> Result< Vec<HashMap<String,String>>, Box<dyn Error>>>,
}

impl BasicTask {
    pub fn new(name: String, f: Box<dyn Fn(&[HashMap<String, String>]) -> Result< Vec<HashMap<String,String>>, Box<dyn Error>>>) -> BasicTask {
        BasicTask{ id: name, f: f }
    }
}

impl Task for BasicTask {
    fn run(&self, msg: &[HashMap<String,String>]) -> Result< Vec<HashMap<String,String>>, Box<dyn Error>  > {
        match (self.f)(msg) {
            Ok(m) => Ok(m),
            Err(e) => Err(Box::new(MyError::new(&format!("Error in basic task {:?} with message: {:?}", &self.id, e.to_string())))),
        }
    }
}


struct Pipeline<T: Task> {
    tasks: Vec<T>,
}


impl<T: Task> Pipeline<T> {
    pub fn new(tasks: Vec<T>) -> Pipeline<T> {
        Pipeline{ tasks: tasks }
    }

    fn run_task(tasks: &[T], msgs: Vec<HashMap<String, String>>) -> Result<Vec<HashMap<String,String>>, Box<dyn Error>> {
        if tasks.len() <= 0 {
            return Ok( msgs )
        }
        match tasks[0].run(&msgs) {
            Ok(m) => Pipeline::run_task(&tasks[1..], m),
            Err(e) => panic!("Failed with error {:?}", e),
        }
    }

    pub fn start(&self) -> Result<Vec<HashMap<String,String>>, Box<dyn Error>> {
        println!("Starting Pipeline");

        Pipeline::run_task(&self.tasks, Vec::new())
    }
}

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}



fn simple_task(msg: &[HashMap<String, String>]) -> Result< Vec<HashMap<String,String>>, Box<dyn Error>> {
    println!("Hello, world!");

    let mut v = Vec::new();
    let mut map = HashMap::new();
    map.insert("output".to_string(), "Hello, world!".to_string());

    v.extend_from_slice(msg);
    v.push(map);

    Ok(v)
}


fn simple_error(_msg: &[HashMap<String, String>]) -> Result< Vec<HashMap<String,String>>, Box<dyn Error>> {
    Err(Box::new(MyError::new("uknown")))
}


fn main() {
    let tasks = vec![BasicTask::new("hello world".to_string(), Box::new(simple_task)), BasicTask::new("kiss my ass".to_string(), Box::new(simple_error))];
    let pipe = Pipeline::new(tasks);

    let _ = pipe.start();
}
