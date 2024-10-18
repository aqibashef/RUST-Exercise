use std::fmt::{Result, Formatter, Display};

struct Student {
    name: String,
    age: u8,
    grade: f32
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\n\tname: {}\n\tage: {}\n\tgrade: {}\n}}", self.name, self.age, self.grade)
    }
}

fn filter_student_by_age(student_list: Vec<Student>, age: u8) -> Vec<Student> {
    let mut students = Vec::<Student>::new();
    for s in student_list {
        if s.age >= age {
            students.push(s);
        }
    }
    return students;
}

fn main() {
    let mut student_vec = Vec::<Student>::new();
    student_vec.push(Student{
        name: "Roy Harper".to_string(),
        age: 17,
        grade: 3.9
    });
    student_vec.push(Student{
        name: "David Shane".to_string(),
        age: 22,
        grade: 2.9
    });
    student_vec.push(Student{
        name: "Donald Ackerman".to_string(),
        age: 18,
        grade: 3.5
    });
    student_vec.push(Student{
        name: "Sarah Mackinzie".to_string(),
        age: 17,
        grade: 2.7
    });
    student_vec.push(Student{
        name: "Jenny Olsen".to_string(),
        age: 15,
        grade: 3.4
    });

    student_vec = filter_student_by_age(student_vec, 18);
    for s in student_vec {
        println!("{}", s);
    }
}