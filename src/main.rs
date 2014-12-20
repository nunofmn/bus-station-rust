use std::io;

fn main() {
    let mut reader = io::stdin();

    let mut sum: int = 0;
    let mut num: int = 0;

    let numbergroups: int = from_str(reader.read_line().unwrap().trim()).unwrap();

    let groups: Vec<int> = reader.read_line().unwrap().split(' ')
                                 .map(|s| {
                                     num = from_str(s).unwrap();
                                     sum = sum + num;
                                     return num;
                                 }).collect();

    let mut occupied: int;

    for i in range(1,sum/2+1) {
        occupied=0;

        if sum % i != 0 {
            continue;
        }

        for group in groups.iter() {
            occupied=occupied+*group;
            if occupied == i {
                occupied=0;
            }else if occupied > i {
                break;
            }
        }

        if occupied == 0 {
            print!("{} ", i);
        }

    }

    print!("{}", sum);

}
