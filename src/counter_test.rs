pub fn counter_test2() {
    let mut str1: String= "haha".parse().unwrap();
    //let xs: [i32; 5];
    let mut ch1: char = 'a';
    let mut int_count: i32 = 0;
    //let mut ary:[char;123456789*3] = [0;30];

    loop {
        int_count += 1;
        if int_count > 123456789 {
            break;
        }
        //ch1 = 'b';
        //str1.push_str("abc");
        str1.push('🐲');
        str1.push('b');
        str1.push('c');


//        ary.get(0)="a";
//        ary[int_count*3]='a';
//        ary[int_count*3+1]='b';
//        ary[int_count*3+2]='c';
    }
    println!("{}", str1.len());
}

pub fn counter_test() {
    let mut str1 = String::from("中文字串很多人");
    //let xs: [i32; 5];
    for _n in 1..123456789 {
        //str1.push_str("abc");
        str1 += "abc";
    }
    println!("{}", str1.len());
}

