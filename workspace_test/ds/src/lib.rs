pub enum LList {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<LList>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl LList {
    // Create an empty list
    pub fn new() -> LList {
        // `Nil` has type `List`
        LList::Nil
    }

    // Consume a list, and return the same list with a new element at its front
    pub fn prepend(self, elem: u32) -> LList {
        // `Cons` also has type List
        LList::Cons(elem, Box::new(self))
    }

    // Return the length of the list
    pub fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            LList::Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            LList::Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    pub fn stringify(&self) -> String {
        let mut s = LList::_stringify(&self);
        let trunc_offset = s.find(", Nil").unwrap();
        s.replace_range(trunc_offset.., "");

        s
    }

    fn _stringify(&self) -> String {
        match *self {
            LList::Cons(head, ref tail) => {

                format!("{}, {}", head, tail._stringify())
            },
            LList::Nil => {
                format!("Nil")
            },
        }
    }
}
