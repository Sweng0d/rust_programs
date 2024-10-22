struct Student {
    name: String,
    age: u32,
    final_note: u32,
}

impl Student {
    fn add_student(studentsvector: &mut Vec<Student>, name: String, age: u32, final_note: u32) {
        studentsvector.push(Student{name, age, final_note});
    }

    fn remove_student(studentsvector: &mut Vec<Student>, name:&str) {
        studentsvector.retain(|Student| Student.name != name);
    }

    fn show_students(VecStudents: &Vec<Student>) {
        if VecStudents.is_empty() {
            println!("There is no student.");
        } else {
            for Student in VecStudents {
                println!("Name: {}, Age: {}, Final Note: {}", Student.name, Student.age, Student.final_note);
            }
        }
    }
}

fn main () {
    let mut students: Vec<Student> = Vec::new();

    Student::add_student(&mut students, "Bruno".to_string(), 24, 85);
    Student::add_student(&mut students, "Bob".to_string(), 44, 91);
    Student::add_student(&mut students, "Alice".to_string(), 18, 99);

    Student::show_students(&students);

    Student::remove_student(&mut students, "Bob");
    println!("After the removal of Bob, the studens are");
    Student::show_students(&students);

}



