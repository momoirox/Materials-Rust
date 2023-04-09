#[derive(Clone)]
enum Address {
    Address(Box<MyList>), 
    Nil,
}
#[derive(Clone)]
struct MyList {
    value: u32,
    next: Address,
}

impl MyList {
   fn append(&mut self, val:u32){
    match self.next {
        Address::Address(ref mut node)=>{
            node.append(val)
        }
        Address::Nil => {
            let node = MyList{
                value:val,
                next : Address::Nil
            };
            self.next = Address::Address(Box::new(node))
        }
    }
   }
   fn delete(&mut self, elem: u32) {
    match self.next {
        Address::Address(ref mut next_address) => {
            if next_address.value == elem {
                println!("Deleting value {}", next_address.value);
                self.next = next_address.next.clone();
            } else {
                next_address.delete(elem);
            }
        }
        Address::Nil => {
            if self.value == elem {
                self.value = 0;
            } else {
                println!("Element {} does not exist in the linked list", elem);
            }
        }
    }
    }
    fn update(&mut self,val:u32, new_val: u32){
        match  self.next{
            Address::Address(ref mut node)=>{
                if node.value == val {
                    node.value = new_val
                }else{
                    node.update(val, new_val);
                }
            }
            Address::Nil=>{
                if self.value == val{
                self.value = new_val;
                } else {
                    println!("Value {} was not found",val);
                }
            }
            
        }
    }
    fn count(&self) -> u32{
        match self.next {
            Address::Address(ref node) => 1 + node.count(),
            Address::Nil => 0,
        }
    }
    
    fn list(&self){
        println!("{}",self.value);
        match self.next {
            Address::Address(ref val)=>{
                val.list()
            }
                Address::Nil=> {}
        }
    }
   
}

fn main() {
    let mut head = MyList{
        value: 1,
        next: Address::Nil
    };
    head.append(2);
    head.append(3);
    head.append(4);
    head.append(5);
    head.delete(2);
    head.update(4,8);
    head.list();
    println!("The size of the list is {}", head.count());
}
