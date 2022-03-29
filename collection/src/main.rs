use std::collections::HashMap;

fn main() {

    {//Vector part
        let v: Vec<i32> = Vec::new();
        println!("Value of v : {:?}", v);

        let v:Vec<i32> = vec![1,2,3];
        println!("Value of v : {:?}", v);

        let mut v : Vec<i32> = Vec::new();

        v.push(18);
        v.push(17);
        v.push(18);
        v.pop();
        println!("Value of v : {:?}", v);
        println!("Max value of v : {}", max(&v));


        let last = v[v.len()-1];
        let _v_immu: Vec<i32> = vec![1,2,3];

        println!("The first element of v is : {}", last);
        v.push(24);
        add_ten(&mut v);
        println!("The first element of v is : {:?}", v); 
    }

    {//String part
        let mut s = String::from("Bonsoir");
        s.push('h');
        s.pop();
        let mut s2:String = 5u32.to_string();
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

    {//hash map part
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("green"), 18);
        let blue_score = match scores.get("blue"){
            Some(score) => score,
            None => &0,
        };
        println!("The score of the blue team is : {}", blue_score);

        scores.entry(String::from("yellow")).or_insert(50); //yellow is not in scores map so it's added with the value 50
        scores.entry(String::from("blue")).or_insert(50); // blue is in scores so is value is returned

        println!("Scores : {:?}", scores);


        let text = "Hello there! Hello general kenobi";

        let mut map:HashMap<&str, i32> = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1; 
        }

        println!("{:?}", map);
    }


    {//exo 1 given a list of integer, return median and mode
        let l = [2,5,4,3,8,4,3,6,5,2,8,1,7];
        let lsorted:Vec<u32> = sorte(&Vec::from(l));
        println!("mediane : {}", lsorted[((lsorted.len()/2)+1)]);
        let mut map: HashMap<u32,u32> = HashMap::new();
        for el in &lsorted{
            let element = map.entry(*el).or_insert(0);
            *element += 1;
        }
        let mut max: (u32,u32) = (0,0); //u32 min value is 0
        for (k, v) in &map {
            if max.1 < *v {max = (*k,*v)} 
        }
        println!("The value most present in l is {}", max.0);
    }
}


fn sorte(v: &Vec<u32>) -> Vec<u32> {
    assert!(v.len()>0); //Liste non vide


    let mut res = Vec::new();
    let mut temp = Vec::new();
    for el in v {
        temp.push(*el);
    }
    while temp.len() > 0 {
        let min = min(&temp);
        res.push(*min);
        temp = delete(&temp, *min);
    }



    assert_eq!(v.len(), res.len());
    assert!({
        let mut all_present = true;
        for el in v {
            all_present = all_present && res.contains(el); // tous les élements de v sont bien dans res
        }
        all_present
    });
    assert!({
        let mut all_present = true;
        for el in &res {
            all_present = all_present && v.contains(el); // tous les éléments de res sont bien dans v 
        }
        all_present
    });
    assert!({
        let mut all_min = true;
        let size : usize = res.len();
        let mut count = 0;
        while count < size {
            let current = res.get(count);
            let mut sub_count = count;
            while sub_count<size {
                all_min = all_min && current <= res.get(sub_count); //le résultat est bien trié 
                sub_count+=1;
            }
            count += 1;
        }
        all_min
    });

    
    res
}

fn min(v :&Vec<u32>) -> &u32 {
    assert!(v.len()>0); //v est non vide


    let mut min = &v[0];
    for el in v {
        if el < min {min = el}
    }


    assert!(v.contains(min)); //resultat est bien dans v
    assert!({
        let mut ver = true;
        for el in v {ver = ver && el >= min} // le resutlat est bien inférieur à tous les éléments de v
        ver
    });
    min
}

fn delete(v: &Vec<u32>, x :u32) -> Vec<u32> {
    assert!(v.len()>0); //v non vide
    assert!(v.contains(&x)); // x est bien contenue dans v


    let mut res :Vec<u32> = Vec::new();
    let mut as_been_removed = false;
    for el in v {
        if el == &x && !as_been_removed { as_been_removed = true}
        else {res.push(*el)}
    }


    assert_eq!(v.len() , res.len()+1); // on a bien retiré qu'un seule élément
    assert!({
        let mut ver = true;
        for el in &res { ver = ver && v.contains(&el)} // tous les éléments du résultat sont bien contenue dans v
        ver
    });
    res
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