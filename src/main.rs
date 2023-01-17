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

fn main() {
    gtk::init();

    
    let window = Window::new(WindowType::Toplevel);
    window.set_title("MySQL Example");
    window.set_default_size(350, 250);

    let layout = Layout::builder()
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
            
	let entry = Entry::builder()
            .placeholder_text("input")
            .can_default(true)
            .can_focus(true)           
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
            
     let save_button = Button::builder()
            .label("Click1")
            .margin(60)
            .margin_bottom(60)
            .margin_top(60)
            .margin_start(60)
            .margin_end(60)
            .build();
        
        layout.add(&entry);
        layout.add(&save_button);
        window.add(&layout);

        //test Error
        let text = entry.text().as_str();

        
            let url = "mysql://root:password@localhost:3306/testdb";
			let pool = Pool::new(url);
            let mut conn = pool.expect("REASON").get_conn();
                let payments = vec![
				example { id: 1, data: Some("foo".into()) },
    ];

   conn.expect("REASON").exec_batch(
			"INSERT INTO example (id,  data)
            VALUES (:id, :data)",
            payments.iter().map(|p| params! {
            "id" => p.id,
            "data" => &p.data,
        })
    );
        save_button.connect_clicked(move |_| {
		//error
		
		
		//endline	
        
		});
    
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
