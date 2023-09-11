use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct Stretch{

    pub id: String,
    pub date: String,
    pub text: String,

}

pub fn sql_connect(url: String) {

    let url = Opts::from_url(&url).unwrap();
    let pool = Pool::new(url);
    let p = match pool{
        Ok(p) => p,
        Err(err) =>{
            eprintln!("Error pooling url: {}", err);
            return;
        }
    };

    
    match p.get_conn(){

        Ok(_) => println!("Connected !"),
        Err(err) => {
            eprintln!("Error obtaining database conn: {}", err);
                return;
        }
    };

   return 


}

pub fn sql_query(query: String, url: String) -> std::result::Result<(), Box <dyn std::error::Error>>{

    let url = Opts::from_url(&url).unwrap();
    let pool = Pool::new(url)?;
    
    let mut conn = pool.get_conn()?;

    let q: Vec<Stretch> = conn.query_map("select * from text",
                                           |(id, date, text)| {
                                               Stretch{
                                                   id,
                                                   date,
                                                   text,
                                               }
                                           })?;

    for res in q{
        println!("id {} date {} text {}", res.id, res.date, res.text);
    }

    Ok(())

}

pub fn sql_insert(url: String, data: Stretch) -> std::result::Result<(), Box <dyn std::error::Error>>{

    let url = Opts::from_url(&url).unwrap();
    let pool = Pool::new(url)?;
    
    let mut conn = pool.get_conn()?;


    let mut trn = conn.start_transaction(TxOpts::default())?;
    trn.exec_drop("INSERT INTO text (id, date, text) VALUES (?, ?, ?)", (
            data.id, data.date, data.text,))?;

    let _ = trn.commit();

    Ok(())



}
