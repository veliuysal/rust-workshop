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
    //let small = Rectangle {
    //       width:10,
    //       height:20
    //    };
    //    //print the rectangle's area
    //    println!("width is {} height is {} area of Rectangle
    //    is {}",small.width,small.height,small.area());

    // --- Static Method in Structure ---
    //struct Point {
    //    x: i32,
    //    y: i32,
    // }
    //mpl Point {
    //    //static method that creates objects of the Point structure
    //    fn getInstance(x: i32, y: i32) -> Point {
    //       Point { x: x, y: y }
    //    }
    //    //display values of the structure's field
    //    fn display(&self){
    //       println!("x ={} y={}",self.x,self.y );
    //    }
    // }
    //let p1 = Point::getInstance(10,20);
    //    p1.display();

    //################ ENUM ################
    // --- Declaring a enum ---
    //enum enum_name {
    //    variant1,
    //    variant2,
    //    variant3
    // }

    // --- Using a enumaration ---
    //#[derive(Debug)]
    // enum GenderCategory {
    //    Male,Female
    // }
    // let male = GenderCategory::Male;
    //    let female = GenderCategory::Female;
    //
    //    println!("{:?}",male);
    //    println!("{:?}",female);

    // ################ OPTION ENUM ################
    //enum Option<T> {
    //    Some(T),      //used to return a value
    //    None          // used to return null, as Rust doesn't support the null keyword
    // }

    // --- Example ---
    //    let result = is_even(3);
    //    println!("{:?}",result);
    //    println!("{:?}",is_even(30));

    // fn is_even(no:i32)->Option<bool> {
    //    if no%2 == 0 {
    //       Some(true)
    //    } else {
    //       None
    //    }
    // }

    // ################ MATCH STATEMENT & ENUM ################
    //enum CarType {
    //    Hatch,
    //    Sedan,
    //    SUV
    // }
    // fn print_size(car:CarType) {
    //    match car {
    //       CarType::Hatch => {
    //          println!("Small sized car");
    //       },
    //       CarType::Sedan => {
    //          println!("medium sized car");
    //       },
    //       CarType::SUV =>{
    //          println!("Large sized Sports Utility car");
    //       }
    //    }
    // }
    //    print_size(CarType::SUV);
    //    print_size(CarType::Hatch);
    //    print_size(CarType::Sedan);

    // ################ MATCH WITH OPTION ENUM ################
    //match is_even(5) {
    //       Some(data) => {
    //          if data==true {
    //             println!("Even no");
    //          }
    //       },
    //       None => {
    //          println!("not even");
    //       }
    //    }
    //fn is_even(no:i32)->Option<bool> {
    //    if no%2 == 0 {
    //       Some(true)
    //    } else {
    //       None
    //    }
    // }

    //################ MATCH & ENUM WITH DATA TYPE ################
    //#[derive(Debug)]
    // enum GenderCategory {
    //    Name(String),Usr_ID(i32)
    // }
    //let p1 = GenderCategory::Name(String::from("Mohtashim"));
    //    let p2 = GenderCategory::Usr_ID(100);
    //    println!("{:?}",p1);
    //    println!("{:?}",p2);
    //
    //    match p1 {
    //       GenderCategory::Name(val)=> {
    //          println!("{}",val);
    //       }
    //       GenderCategory::Usr_ID(val)=> {
    //          println!("{}",val);
    //       }
    //    }

    //################ STRUCT & ENUM ################
    //#[derive(Debug)]
    // enum GenderCategory {
    //    Male,Female
    // }
    //#[derive(Debug)]
    // struct Person {
    //    name:String,
    //    gender:GenderCategory
    // }
    //let p1 = Person {
    //       name:String::from("Mohtashim"),
    //       gender:GenderCategory::Male
    //    };
    //    let p2 = Person {
    //       name:String::from("Amy"),
    //       gender:GenderCategory::Female
    //    };
    //    println!("{:?}",p1);
    //    println!("{:?}",p2);



}
