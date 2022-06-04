use std::collections::HashMap;

fn main(){
    // vec
    let mut v1 = Vec::new();
    v1.push(1);
    //初始化值
    let v2 = vec![1, 2, 3];

    // 可以通过下标取值
    // let does_not_exist = &v2[100];
    let does_not_exist = v2.get(100);
    assert_eq!(does_not_exist, None);

    //存储不同类型的元素
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }

    // hashmap vec -> hashmap
    let teams_list = vec![
        ("china".to_string(), 100),
        ("usa".to_string(), 10),
        ("japan".to_string(), 5),
    ];

    let mut teams_map = HashMap::new();
    for team in &teams_list {
        // 注意所有权的转移 insert会拿走变量所有权
        teams_map.insert(&team.0, team.1);
    }

    // let mut teams_map1: HashMap<_,_> = teams_list.into_iter().collect();
    let score = teams_map.get(&"china".to_string()).unwrap();
    println!("{:?}",score);

    //若不存在则插入新值
    let c = "canada".to_string();
    let s = teams_map.entry(&c).or_insert(50);
    *s = *s + 1;

    assert_eq!(*s, 51);
    assert_eq!(*teams_map.get(&c).unwrap(), 51);

    println!("{:?}",teams_map);

}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}
fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}