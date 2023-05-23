use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let mut node1: Node<i32> = Node::new(5);
    println!("{:?}", node1);
    {
        let node2: Node<i32> = Node::new_from_rc(&node1.value());
        println!("{:?}", node2);
        node1.set_next_node(node2);
    }
    println!("{:?}", node1);
    println!("{:?}", node1.get_next_node());
    println!("{:?}", node1.next_node);
    
}

#[derive(Debug)]
struct Node<T> {
    value: Rc<T>,
    next_node: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(v: T) -> Node<T> {
        Node { value: Rc::new(v), next_node: None }
    }

    fn new_from_rc(vrc: &Rc<T>) -> Node<T> {
        Node { value: vrc.clone(), next_node: None }
    }

    fn value(&self) -> Rc<T> {
       self.value.clone()
    }

    fn set_next_node(&mut self, node: Node<T>) {
        self.next_node = Some(Box::new(node));
    }

    fn get_next_node(&self) -> &Option<Box<Node<T>>> {
        &self.next_node
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::Node;

    #[test]
    fn node_value() {
        let node_one = Node::new(32);
        assert_eq!(node_one.value(), Rc::new(32));
    }

    #[test]
    fn next_node_value() {
        let mut node_one = Node::new(32);
        let node_two = Node::new(64);
        node_one.set_next_node(node_two);
        assert_eq!(node_one.get_next_node().as_ref().unwrap().value(), Rc::new(64));
    }
}

