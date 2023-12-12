// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student1 = Student {
        name: String::from("John"),
        locker: Some(1),
    };

    let student2 = Student {
        name: String::from("James"),
        locker: None,
    };

    let student3 = Student {
        name: String::from("Jill"),
        locker: Some(2),
    };

    let students = vec![student1, student2, student3];

    for student in students {
        match student.name {
            name => println!("name: {:?}", name),
        }
        match student.locker {
            Some(locker_number) => println!("locker number: {:?}", locker_number),
            None => println!("student has no locker"),
        }
    }
}
