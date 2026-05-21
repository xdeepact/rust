struct Contact {
    name: String,
    email: String,
    title: String,
}

fn main() {
    // 1. What is the length of the contacts vector?

    let mut contacts: Vec<Contact> = vec![];

    // 2. Add the following people to the contacts vector.

    let jake = Contact {
        name: String::from("Jake Overall"),
        email: String::from("jake.overall@boisecodeworks.com"),
        title: String::from("founder"),
    };

    let matt = Contact {
        name: String::from("Matt Overall"),
        email: String::from("matt.overall@boisecodeworks.com"),
        title: String::from("founder"),
    };

    let tony = Contact {
        name: String::from("Mark Ohnsman"),
        email: String::from("mark@boisecodeworks.com"),
        title: String::from("instructor"),
    };

    let andrew = Contact {
        name: String::from("Darryl Kilzer"),
        email: String::from("darryl@boisecodeworks.com"),
        title: String::from("instructor"),
    };

    let tom = Contact {
        name: String::from("Tom Day"),
        email: String::from("tom@boisecodeworks.com"),
        title: String::from("instructor"),
    };

    // 3. Woops after adding all of those people to the same contacts list you realized you need a list just the instructors.
    // create a new variable named instructors and populate it using the contacts vector.
}
