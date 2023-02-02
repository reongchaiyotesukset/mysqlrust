extern crate gtk;

use mysql::*;
use gtk::prelude::*;
use mysql::prelude::*;
use gtk::{Button, Entry, Window, WindowType,Layout};

#[derive(Debug, PartialEq, Eq)]
struct example {
    id: i32,
    data: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
	       
            //let url = "mysql://root:12345678@localhost:3306/testdb";
            let   url = "mysql://cargo:12345678@192.168.1.43:3306/testdb";
			let mut pool = Pool::new(url)?;
			//let mut conn = pool.get_conn()?; 	
            let mut conn = Pool::get_conn(&pool)?;
               
			//select
			 let stack = conn
        .query_map(
            "SELECT*from example",
            |(id, data)| {
                example { id, data }
            },
        )?;
        
					println!("{:?}",stack[0].data);
					println!("{:?}",stack[1].data);
					println!("{:?}",stack[2].data);
	

      
              Ok(())
}
