// https://www.elitesecurity.org/t154369-Zadatak-Jednostruko-spregnuta-lista?fbclid=IwAR0VpPZvwbvVIYj4hVxzaSsDssa_JXdFUUcdjpWErvBpXj4qvlANveCVq1k

// ZADATAK : Napisati program koji demonstrira realizaciju
// osnovnih operacija sa dinamickom strukturom podataka
// oblika jednostruko spregnute liste. Element liste se
// sastoji od dva polja: podatak koji predstavlja ceo
// broj i pokazivac na sledeci element liste.
// Operacije koje treba realizovati sa jednostruko
// spregnutom listom su:
// -Unos novog broja, po izboru korisnika(na pocetak ili
// kraj liste)
// NAPOMENA: Zadovoljiti uslov da se u listi ne moze
// nalaziti vise od 5 istih brojeva)
// -Modifikacija (umesto jednog broja treba upisati drugi
// broj)
// -Trazenje broja u listi
// -Brisanje broja iz liste (ako ima vise istih,
// izbrisati ih sve)
// DODATNO:
// Napraviti tabelarni prikaz pojavljivanja brojeva u listi.


#[derive(Clone)]
enum Address{
    Address(Box<Node>),
    Nil,
}
#[derive(Clone)]
struct Node{
    value: i32,
    next: Address
}
impl Node{
    fn new(v: i32, nx: Address) -> Self{
        Self{
            value: v,
            next: nx,
        }
    }
    fn push_back(&mut self, v: i32){
        let count = self.count_values(v);
        if count<5 {
            self.push_back_value(v);

        }else{
            println!("Cannot add more than 5 same {} values!", v)
        }
    }

    fn push_back_value(&mut self, v: i32){
        match self.next {
            Address::Address(ref mut next_node) =>{
                next_node.push_back_value(v);
            },
            Address::Nil =>{
                let newNode= Node{
                    value: v,
                    next: Address::Nil
                };
                self.next = Address::Address(Box::new(newNode))
            }
        }
    }

    fn push_front_value(&mut self, v: i32){
        let newNode = Node{
            value: v,
            next: Address::Address(Box::new(self.clone()))
        };
        *self = newNode
    }

    fn push_front(&mut self, v: i32){
        let count = self.count_values(v);
        if count<5 {
            self.push_front_value(v);

        }else{
            println!("Cannot add more than 5 same {} values!", v)
        }

    }

    fn count_values(&self, v: i32) -> i32{
        let mut count = 0;
        if self.value == v{
            count +=1;
        }
        match self.next{
            Address::Address(ref next_node) => {
                return count + next_node.count_values(v);
            }
            Address::Nil => {
                return count;
            }
        }
    }

    fn delete(&mut self, v:i32){
        if self.value == v {
            match self.next {
                Address::Address(ref mut next_node) =>{
                    *self = *next_node.clone();
                    self.delete(v);
                }
                Address::Nil=>{
                    println!("Cannot delete root!");
                    return;
                }
            }
        }
        match self.next{
            Address::Address(ref mut next_node) => {
                if next_node.value == v{
                    self.next = next_node.next.clone();
                    self.delete(v); //da bi se brisalo sve
                } else{
                    next_node.delete(v); // ponovo se poziva
                }

            }
            Address::Nil =>{
                //obrisali smo sve sto treba
                println!("Deleted");
            }
        }
    }

    

    fn find(&self, v:i32) -> i32{
        let mut i = 0;
        let mut j = self;
        let count = self.count_values(v);
        if count!=0 {
            loop{
                if j.value == v {

                    return i;
                }
                match j.next{
                    Address::Address(ref next_node) =>{
                        j = next_node;
                        i +=1;
                        println!("Here {}.", i);
                    },
                    Address::Nil=>{
                        println!("Not found value {}", v);
                        return -1;
                    }
                }
            }
        }else{
            println!("There is no value {}.", v);
            return -1;
        }
    }


    fn update(&mut self, idx: i32, new_value: i32){
        let mut i = 0;
        let mut j = self;
        if i == idx {
            j.value = new_value;
        }

        while i<idx {
            match j.next{
                Address::Address(ref mut nex_node) => j = nex_node,
                Address::Nil => {
                    println!("There is no index!");
                    return;
                }
            }
            i +=1; //ako nema indeks doda na kraj
        }
        j.value = new_value
        
    }

    fn print(&self){
        print!("{} ", self.value);
       /* if Some(a) = self.next {
            self.next.print();
        }*/
        match self.next{
            Address::Address(ref next_address) =>{
                next_address.print();
            }
            Address::Nil =>{
                println!("\nDone!")
            }
        }


    }
}



fn main() {
    let mut list = Node::new(3, Address::Nil);
    list.push_back(5);
    list.push_back(5);
    list.push_back(5);
    list.push_back(5);
    list.push_back(5);
    list.push_back(5);

    list.push_front(5);
    list.push_front(2);
    list.push_front(2);
    list.push_front(2);
    
    list.print();
    list.update(6,17);
    list.print();


    println!("Number of 3 is {}. ", list.count_values(3));
    println!("Number 5 can be found on index {}. ", list.find(5));
    println!("Number 13 can be found on index {}. ", list.find(13));

    list.delete(2);
    list.print();
    list.delete(5);
    list.print();
    list.delete(17);
    list.delete(5);
    list.delete(0);
    list.print();

}
