pub enum List {
    Empty,
    Elem(i32, Box<List>)
}

pub enum IpAddrKind {
    v4,
    v6,
}