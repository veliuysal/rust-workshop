fn main() {

    //################ STRUCT ################
    // --- Declaring a structure ---
    //struct Name_of_structure {
    //    field1:data_type,
    //    field2:data_type,
    //    field3:data_type
    // }

    // --- Initializing a structure ---
    //struct Employee {
    //    name:String,
    //    company:String,
    //    age:u32
    // }
    //let emp1 = Employee {
    //       company:String::from("OYK"),
    //       name:String::from("Veli UYSAL"),
    //       age:36
    //    };
    //    println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);

    // --- Modifying a struct instance ---
    //let emp1 = Employee {
    //       company:String::from("OYK"),
    //       name:String::from("Veli UYSAL"),
    //       age:36
    //    };
    //emp1.age = 37;
    // println!("Name is :{} company is {} age is
    // {}",emp1.name,emp1.company,emp1.age);

    // --- Passing a struct to a function ---
    //struct Employee {
    //    name:String,
    //    company:String,
    //    age:u32
    // }
    //let emp1 = Employee {
    //       company:String::from("OYK"),
    //       name:String::from("Veli UYSAL"),
    //       age:36
    //    };
    //let emp2 = Employee {
    //       company:String::from("OYK"),
    //       name:String::from("Aydın YAKAR"),
    //       age:42
    //    };
    // display(emp1);
    //    display(emp2);
    //fn display( emp:Employee){
    //    println!("Name is :{} company is {} age is
    //    {}",emp.name,emp.company,emp.age);
    // }

    // --- Returning struct from a function ---
    //fn who_is_elder (emp1:Employee,emp2:Employee)->Employee {
    //    if emp1.age>emp2.age {
    //       return emp1;
    //    } else {
    //       return emp2;
    //    }
    // }
    //let emp1 = Employee {
    //       company:String::from("OYK"),
    //       name:String::from("Veli UYSAL"),
    //       age:36
    //    };
    //let emp2 = Employee {
    //       company:String::from("OYK"),
    //       name:String::from("Aydın YAKAR"),
    //       age:42
    //    };
    //let elder = who_is_elder(emp1,emp2);

    // --- Method in Structure ---
    //struct Rectangle {
    //    width:u32, height:u32
    // }
    //impl Rectangle {
    //    fn area(&self)->u32 {
    //       //use the . operator to fetch the value of a field via the self keyword
    //       self.width * self.height
    //    }
    // }
    //et small = Rectangle {
    //       width:10,
    //       height:20
    //    };
    //    //print the rectangle's area
    //    println!("width is {} height is {} area of Rectangle
    //    is {}",small.width,small.height,small.area());


}
