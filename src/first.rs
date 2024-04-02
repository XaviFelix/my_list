use std::mem;

//a struct named list that contains a Link enum.
//The enum contains the state of our structure:
//Either Empty or a Node pointer that points to a struct:
//of type Node containing an element or a next Link element
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

//a node struc that contains an element and the next Link enum
struct Node {
    element: i32,
    next: Link,
}


//Self is an alias for "that type, in this case 'List' "
impl List {

    //Providing a static method which is just a normal function inside an impl
    pub fn new() -> Self {
        //We refer to variants of enum using '::' which is the namespacing operator
        List { head: Link::Empty }
    }

        
    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(
            Node {
                element: element,
                next: mem::replace(&mut self.head, Link::Empty),
            }
        );

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        //Check empty list behaves right
        assert_eq!(list.pop(), None);

        //Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        //Check pop function
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //push some more to make sure nothing is corrupted
        list.push(4);
        list.push(5);

        //Check normal pop functionality
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
//There are 3 primary forms of ownership that a self can take:
//1) self, &mut self, &self
//- self: Value (represents true ownership)
//      -Can do whatever you want with it: move it, destroy it, mutate it or load it out via a reference
//      -passing by value then it is moved to the new location and new location now owns the value and the old location can't access it
//      -For that reason most methods don't want self
//- &mut self: mutable reference (temporary exclusive access to a value that you don't own)
//- &self: shared reference (temporary shared access to a value that you don't own)