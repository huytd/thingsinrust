struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>
}

impl<'a> Node<'a> {
    fn render(&self) -> String {
        let mut output = String::new();
        match self.l {
            Some(ref ln) => output += ln.render().as_str(),
            None => {}
        }
        output += self.val;
        match self.r {
            Some(ref rn) => output += rn.render().as_str(),
            None => {}
        }
        return output;
    }
}

macro_rules! some_box {
    ($expression:expr) => {
        Some(Box::new($expression))
    }
}

#[allow(unused_variables)]
fn main() {
    let n = Node { 
        val: "+", 
        l: some_box!(Node { val: "5", l: None, r: None }), 
        r: some_box!(Node { val: "15", l: None, r: None }), 
    };
    println!("{}", n.render());
}
