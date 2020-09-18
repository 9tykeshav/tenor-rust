extern crate reqwest;

fn main() {
    println!("this was written by owl");
    //Creating a new client 
    let client : Client = Client::new("keshav" , "your_token" );
    println!("{} with token {}" , client.name , client.token);
    println!("{}" , client.get_gif("owl").unwrap());

}

//The struct for the main impl
struct Client {
    name : String ,
    token : String ,
    req : reqwest::blocking::Client
}

//Impl for client
impl Client {

    //Func for making a new instance of Client
    fn new (name : &str , token : &str ) -> Client {
        //Converting para to String from &str 
        let name = name.to_string();
        let token = token.to_string() ;
        Client { name : name , token : token , req : reqwest::blocking::Client::new()}

    }


    //Get gif function still don know why i made this 
    fn get_gif (&self , search_term : &str) -> Result<String , reqwest::Error> {
    //Constructing the query 
    let query = format!("https://api.tenor.com/v1/search?q={}&key={}&limit=1",search_term ,self.token);
    //Requesting Tenor 
    let gifs = self.req.get(&query).send()?
        .text()?;

    //Aight return em if Ok 
    Ok(gifs)

    }
}
//Ignore noob moves made for learning Purposes 
