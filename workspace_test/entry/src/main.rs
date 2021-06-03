use ds::LList;
fn main() {
    let mut list = LList::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    let s = list.stringify();
    println!("{}", s);
}
