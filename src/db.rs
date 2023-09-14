use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub struct Stretch{

    pub id: String,
    pub date: String,
    pub text: String,

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
