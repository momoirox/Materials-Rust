#[derive(Clone)]
enum Address {
    Address(Box<MyList>),
    Nil
}

#[derive(Clone)]
struct Transaction {
    id: u32,
    sender: String,
    reciever: String,
    amount: f32,
}

#[derive(Clone)]
struct MyList{
    value: Transaction,
    next: Address
}

impl MyList {
    
    fn append(&mut self,new_transaction: Transaction){
        if self.value.amount > new_transaction.amount {
            
            let list = MyList{
                value: new_transaction,
                next: Address::Address(Box::new(self.clone()))
            };
            
            *self = list
        }else{
            match self.next {
                Address::Address(ref mut next_node)=>{
                    if new_transaction.amount > next_node.value.amount{
                        next_node.append(new_transaction)
                    }else{
                        
                        let list = MyList{
                            value: new_transaction,
                            next: Address::Address(next_node.clone())
                        };
                    
                        self.next = Address::Address(Box::new(list))

                    }
                }
                Address::Nil=>{
                    
                    let list = MyList{
                        value: new_transaction,
                        next: Address::Nil
                    };
                    
                    self.next = Address::Address(Box::new(list))
                }
            }
        }
    }
    fn delete(&mut self, id_val :u32){
        if self.value.id == id_val { 
            self.pop()
        }else{
            match self.next {
                Address::Address(ref mut next_node)=>{
                    if next_node.value.id == id_val {                        
                        
                        self.next = next_node.next.clone();
                    }else {
                        next_node.delete(id_val);
                    }
                },
                Address::Nil=>{
                    println!("There is no node with id {} in the list",id_val);
                }
            }
        }
        
    }
    fn pop(&mut self){
        match self.next {
            Address::Address(ref mut next_node)=>{
                let new_list = MyList{
                    value: next_node.value.clone(),
                    next: next_node.next.clone()
                };
                *self = new_list;
            },
            Address::Nil=>{
             
                println!("U cannot delete a root element.");
            }
        }
    }
    fn print(&self){
        println!("|\tID\t|Sender    \t|Reciever     \t|Amount   \t|");
        println!("----------------------------------------------------------------");
        self.print_table()
    }
    fn print_table(&self){
        println!("|\t{}\t|{}\t|{}\t|{}      \t|",self.value.id, self.value.sender, self.value.reciever, self.value.amount);
        println!("-----------------------------------------------------------------");
        match self.next {
            Address::Address(ref next_node)=>{
                next_node.print_table()
            },
            Address::Nil=>{}
        }
    }
    fn size_of(&self)-> i32{

        match self.next {
            Address::Address(ref next_node)=>{
                1 + next_node.size_of()
            },
            Address::Nil=>{
                1
            }
        }
    }
}


fn main() {

    let mut list = MyList{
        value: Transaction{
            id: 1,
            sender: String::from("Maja Blagic"),
            reciever: String::from("Andrea Sabo"),
            amount: 11.22,
        },
        next: Address::Nil
    };

    let new_trans2 = Transaction{
        id: 2,
        sender: String::from("Simke Jovana"),
        reciever: String::from("Paja Patak"),
        amount: 66.22,
    };
    let new_trans3 = Transaction{
        id: 3,
        sender: String::from("Mijat Jovan"),
        reciever: String::from("Miki Mouse"),
        amount: 19.22,
    };
    let new_trans4 = Transaction{
        id: 4,
        sender: String::from("Stajic Stele "),
        reciever: String::from("Winnie Pough"),
        amount: 96.22,
    };
    let new_trans5 = Transaction{
        id: 5,
        sender: String::from("Bozen Ruljic"),
        reciever: String::from("Goofy Goofy"),
        amount: 9.22,
    };
    list.append(new_trans2);
    list.append(new_trans3);
    list.append(new_trans4);
    list.append(new_trans5);
    list.delete(2);
    // list.delete(5);
    // list.delete(6);
    // list.delete(1);
    // list.delete(4);
    // list.delete(3);
    //list.pop();
    list.pop();
    println!("Number of elements inclusding root node is - {}",  list.size_of());

    list.print();
    
}
