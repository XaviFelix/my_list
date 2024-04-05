use std::mem;

pub struct List<T> {
    head: Link<T>,
}

//Type alias
type Link<T> = Option<Box<Node<T>>>;

// enum Link {
//     Empty,
//     More(Box<Node>),
// }

//a node struc that contains an element and the next Link enum
struct Node<T> {
    element: T,
    next: Link<T>,
}

//Self is an alias for "that type, in this case 'List' "
impl<T> List<T> {
    //Providing a static method which is just a normal function inside an impl
    pub fn new() -> Self {
        //We refer to variants of enum using '::' which is the namespacing operator
        List { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    // Todo: Creat a test for this
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

impl<T> Drop for List<T> {
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
