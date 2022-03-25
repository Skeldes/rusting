fn main() {

    {//Vector part
        let v: Vec<i32> = Vec::new();
        println!("Value of v : {:?}", v);

        let v = vec![1,2,3];
        println!("Value of v : {:?}", v);

        let mut v : Vec<i32> = Vec::new();

        v.push(18);
        v.push(17);
        v.push(18);
        v.pop();
        println!("Value of v : {:?}", v);
    }
}
