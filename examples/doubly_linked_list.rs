use unsafe_rust::doubly_linked_list::{DList, DListIter};

fn main() {
    let mut list: DList<&str> = DList::new();
    list.push_front("A");
    list.push_front("B");
    list.push_front("C");
    list.push_back("1");

    {
        let iter = DListIter::new(list.head());
        for v in iter {
            println!("read: {} from DList Iterator", *v);
        }
    }

    let item = list.pop_front();
    println!("pop front from list: {:?}", item); // C
    assert_eq!(item, Some("C"));

    let item = list.pop_back();
    println!("pop back from list: {:?}", item); // 1

    let item = list.pop_back();
    println!("pop back from list: {:?}", item); // A
}
