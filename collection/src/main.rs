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
        println!("Max value of v : {}", max(&v));


        let last = v[v.len()-1];
        let _v_immu = vec![1,2,3];

        println!("The first element of v is : {}", last);
        v.push(24);
        add_ten(&mut v);
        println!("The first element of v is : {:?}", v); 
    }

    {//String part
        let mut s = String::from("Bonsoir");
        s.push('h');
        s.pop();
        let mut s2 = 5.to_string();
        println!("s : {}", s);
        println!("s2 : {}", s2);
        let s3 = s;
        let s3 = s3 + &s2;
        println!("s3 : {}", s3);
        //println!("s : {}", s); //s n'est plus valide car s3 à pris la propiété
        s2 += " 37"; // la modification de s2 ne modifie pas s3
        println!("s3 : {}", s3);

        let s4 = String::from("tic");
        let s5 = String::from("tac");
        let s6 = String::from("toe");
        let s7 = format!("{}-{}-{}",s4,s5,s6); //format don't take ownership of any parameters
        println!("s7 : {}", s7);

        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
    }
}


fn add_ten(v : &mut Vec<i32>){
    assert!(v.len() > 0);
    for el in v {
        *el+=10
    }
}

fn max(v: &Vec<i32>) -> &i32 {
    assert!(v.len()>0); //Some defensive programing

    // code 1
    let mut res = &v[0];
    for el in v {
        if el > res {res = el}
    }

    //code 2
    //let res  = &30; //Le resultat est supérieur dà tous les éléments de v mais n'est pas contenue dans v

    //code 3
    //let res = &17; //Le resultat est bien dans v mais il n'est pas supérieur à tous les éléments de v

    assert!(v.contains(res));//le résultat est bien contenue dans v
    assert!({
        let mut ver = true;
        for el in v {
            ver = ver && el <= res; //le résultat est bien supérieur ou égale à l'ensemble des éléments de v
        }
        ver
    });
    return res;
}