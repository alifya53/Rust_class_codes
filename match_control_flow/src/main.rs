// MATCH CONTROL FLOW

// fn main() {
//      let a:i32 =  5;
//      match a {
//          // left arm (pattern match) =>   right arm (code execute/output)
//          1 => println!("one"),
//          2 => println!("two"),
//          3 => println!("three"),
//          _ => println!("invalid")
//         }
// }

enum Days {
    Mon,
    Tue,
    Wed,
    Thur,
    Fri,
    Sat,
    Sun
}

fn check_weekend(day:Days) -> u8{
    match day {
        Days::Mon => {
            println!("This is Weekday");
            1
        },
        Days::Tue => {
            println!("This is Weekday");
            2
        },
        Days::Wed => {
            println!("This is Weekday");
            3
        },
        Days::Thur => {
            println!("This is Weekday");
            4
        },
        Days::Fri => {
            println!("This is Weekday");
            5
        },
        Days::Sat => {
            println!("This is Weekend");
            6
        },
        Days::Sun => {
            println!("This is Weekend");
            7
        },
    }
}


fn main(){
  let check_day = Days::Sat;
  let value = check_weekend(check_day);
  println!("{}",value);

  let check_day2 = Days::Mon;
  let value_2 = check_weekend(check_day2);
  println!("{}",value_2);


}