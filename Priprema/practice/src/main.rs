use std::collections::VecDeque;

struct Thread {
    id: i32,
}
impl Thread {
    fn print(&self){
        println!("ID:{}", self.id)
    }
}
struct MyQueue {
    queue: Vec<Thread>
}
impl MyQueue {
    fn new()->Self{
        MyQueue {
            queue:vec![Thread{id:0}]
        }
    }
}
fn main() {
    let mut q = MyQueue::new();
    q.queue.push(Thread{id:2});
    q.queue.push(Thread{id:3});
    q.queue.push(Thread{id:4});
    for i in &mut q.queue{
        i.id+=50;
    }

    q.queue.remove(0);
    for (i, ele) in q.queue.iter().enumerate(){
        println!("Index {}",i);
        ele.print();
    }
    match q.queue.get(0){
        Some(val)=>{
            println!("Val found id:{} ",val.id);
        },
        _=> println!("Val not found"),
    }

    match q.queue.get(43){
        Some(val)=>{
            println!("Val found id:{} ",val.id);
        },
        _=> println!("Val not found at index"),
    }

    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(3);
    queue.insert(1,2);
    queue.pop_back();
    queue.pop_front();
    for v in queue.range_mut(..){
        println!("{}",v);
    }
    
    
}
