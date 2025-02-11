fn main(){
    let mut groups: [[&str; 4]; 6] = [[""; 4]; 6];
    groups[0]=["Bob", "Carol", "Eric", "Matt"];
    groups[1]=["Jim", "Lucy", "Terry", "Brenda"];
    groups[2]=["Susan", "Brad", "Jim", "Matt"];
    groups[3]=["Sue", "Wendy", "Sam", "Brad"];
    groups[4]=["Kate", "Jack", "James", "Sydney"];
    groups[5]=["Mary", "John", "Ricky", "Wendy"];

    searchMember("Leander", &groups);
}

fn searchMember(member_name: &str, groups: &[[&str; 4]; 6]) {

    let mut groups_in: Vec<(i32, bool)> = Vec::new();

    for (i,group) in groups.iter().enumerate() {
        for (j, name) in group.iter().enumerate() {
            if member_name == *name {
                let mut leadership = false;
                
                if j == 0 {
                    leadership = true;
                }
                else {
                    leadership = false;
                }
                groups_in.push((i as i32, leadership));

            }
        }
    }

    if groups_in.len() > 0 {
        for i in groups_in.iter() {
            println!("{} in group {} as {}", member_name, i.0.to_string(), if i.1 {"leader"} else {"non-leader"});
        }
    }
    else {
        println!("{} not in groups", member_name);
    }
}
