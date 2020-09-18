extern crate reqwest;

fn main() {
    println!("this was written by owl");
    let client : Client = Client::new("keshav" , "your_token" );
    println!("{} with token {}" , client.name , client.token);
    println!("{}" , client.get_gif("owl").unwrap());

}


struct Client {
    name : String ,
    token : String ,
    req : reqwest::blocking::Client
}


impl Client {


    fn new (name : &str , token : &str ) -> Client {
        let name = name.to_string();
        let token = token.to_string() ;
        Client { name : name , token : token , req : reqwest::blocking::Client::new()}

    }


    fn get_gif (&self , search_term : &str) -> Result<String , reqwest::Error> {

    let query = format!("https://api.tenor.com/v1/search?q={}&key={}&limit=1",search_term ,self.token);

    let gifs = self.req.get(&query).send()?
        .text()?;


    Ok(gifs)

    }
}
