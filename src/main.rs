enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}

enum Major {
    ComputerScience,
    ElectricalEngineering,
}

struct Student {
    name:String,
    grade:GradeLevel,
    major:Major,
}

impl Student {
    fn new(name:String, grade:GradeLevel, major:Major) -> Self {
        Student {
            name,
            grade,
            major,
        }
    }

    fn introduce_yourself(&self) {

        let grade_msg = match self.grade {
            GradeLevel::Bachelor => "I am bachelor",
            GradeLevel::Master => "I am master",
            GradeLevel::PhD => "I am PhD",
        };

        let major_msg = match self.major {
            Major::ComputerScience => "Major in Computer Science",
            Major::ElectricalEngineering => "Major in Electrical Engineering",
        };


        println!("My name is {}", self.name);
        println!("{}", grade_msg);
        println!("{}", major_msg);
    }
}

fn main() {
    let s1 = Student::new("John".to_string(),
                          GradeLevel::Bachelor,
                          Major::ComputerScience);
    s1.introduce_yourself();
}