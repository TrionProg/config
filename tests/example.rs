extern crate config;

#[test]
fn simple_document(){
    let text="name = \"Jacob\"
    id : -1
    \"info\": {
        data=[],
    }
    games:[
        Game{
            name= WoT ;
            developer = \"wargaming.net\",
        },
        Nothing
        ,
    ],
    ";

    let config=config::Config::parse(text).unwrap();

    println!("{}",config.get_text("name").unwrap().text);
    config.get_list("games").unwrap();

    println!("hi");
}
