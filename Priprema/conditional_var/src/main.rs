struct Thread {
    id: u32,
    message: String,
    priority: u32,
}
impl Thread{
    fn print(&self){
        println!("|Priority: {}| ID: {}| Message: {}|", self.priority, self.id, self.message);
    }
}
struct MyQueue {
    queue: Vec<Thread>
}
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            queue: vec![]
        }
    }
    fn print(&self){
        for (i,element) in self.queue.iter().enumerate() {
            println!("Index: {}", i);
            element.print();
        }
    } 
    fn wait(&mut self, thread:Thread){
        self.queue.push(thread);
    }
    fn notify_one(&mut self){
        let max_priority = self.find_max_priority();
        //println!("Max priroty is {}", max_priority);
        
        for (i,element) in &mut self.queue.iter().enumerate() {
            if element.priority == max_priority {
               println!("Removing thread ...");
               element.print();
               self.queue.remove(i);
               break;
            }
        }



    }
    fn find_max_priority(&self)-> u32{
        let mut max_priority = 4;
        for i in &self.queue {
            if i.priority < max_priority {
                max_priority = i.priority
            }
        }
        max_priority
    }
}
fn main() {
   
    let mut cond_var = MyQueue::new();
    for i in 0..10{
        cond_var.wait(Thread {  
            id: i,
            message:  String::from(" Thread no.".to_owned() + &i.to_string()),
            priority: i%5
        }
        );
    }
    //cond_var.print();
    for _ in 0..10{
        cond_var.notify_one();
    }

    cond_var.print();

}
