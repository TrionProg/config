extern crate config;

fn foo(text:&str) -> Result<(),config::Error>{
    let config=config::Config::parse(text)?;

    println!("{}",config.get_text("name")?.value);
    let list=config.get_list("games")?;

    for e in list.iter(){
        let en=e.get_enum()?;

        //if en.variant_name=="Some" {
        if en.is_some() {
            for g in en.iter() {
                println!("{}",g.get_struct()?.get_text("name")?.value);
            }
        }
    }

    Ok(())
}

fn main(){
    let text="name = \"Jacob\"
    id : -1
    \"info\": {
        data=[],
    }
    games:[
        None,
        Some(Game{
            name= WoT ;
            developer = \"wargaming.net\",
        }),
        None,
        Some(
            {
                name : \"COD\",
            price: 42.65
        })
        ,
    ],
    ";

    match foo(text){
        Ok( _ ) => println!("alles gut"),
        Err( e ) => println!("error: {}",e),
    }

    println!("hi");
}
