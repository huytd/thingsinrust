struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>
}

impl<'a> Node<'a> {
    fn render(&self) -> String {
        let mut output = String::new();
        match self.l {
            Some(ref ln) => { output += "("; output += ln.render().as_str() },
            None => {}
        }
        output += self.val;
        match self.r {
            Some(ref rn) => { output += rn.render().as_str(); output += ")" },
            None => {}
        }
        return output;
    }
}

macro_rules! node {
    ( $($props:ident : $value:expr),* ) => { 
        Some(Box::new(Node {
            $($props: $value),*
        })) 
    }
}

#[allow(unused_variables)]
fn main() {
    // (5x + 3) / (2x + 2)
    let n = Node { 
        val: "/", 
        l: node!( 
            val: "+", 
            l: node!(
                val: "*",
                l: node!(
                    val: "5",
                    l: None,
                    r: None
                ),
                r: node!(
                    val: "x",
                    l: None,
                    r: None
                )
            ), 
            r: node!(
                val: "3",
                l: None,
                r: None
            )
        ), 
        r: node!( 
            val: "+", 
            l: node!(
                val: "*",
                l: node!(
                    val: "2",
                    l: None,
                    r: None
                ),
                r: node!(
                    val: "x",
                    l: None,
                    r: None
                )
            ), 
            r: node!(
                val: "2",
                l: None,
                r: None
            ) 
        ), 
    };
    println!("{}", n.render());
}
