use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,          // Con trỏ yếu (con biết cha, nhưng không giữ cha sống)
    children: RefCell<Vec<Rc<Node>>>,     // Cha sở hữu các con
}

fn main() {
    // Tạo node con
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // ban đầu chưa có cha
        children: RefCell::new(vec![]),
    });

    println!(
        "Trước khi tạo branch:\n  leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        // Tạo node cha và gán leaf làm con
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Gán cha cho leaf bằng Weak (không tạo vòng sở hữu)
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "\nTrong scope:\n  branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "  leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        println!(
            "  leaf.parent = {:?}",
            leaf.parent.borrow().upgrade().map(|node| node.value)
        );
    } // branch ra khỏi scope => strong_count = 0 => bị drop

    println!(
        "\nSau khi branch ra khỏi scope:\n  leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // Upgrade sẽ trả về None vì cha đã bị drop
    println!(
        "  leaf.parent = {:?}",
        leaf.parent.borrow().upgrade().map(|node| node.value)
    );
}
