use std::mem;

pub struct List {
    head: Link,
}

//Type alias
type Link = Option<Box<Node>>;

// enum Link {
//     Empty,
//     More(Box<Node>),
// }

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
        List { head: None }
    }

        
    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(
            Node {
                element: element,
                next: self.head.take(),
            }
        );

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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