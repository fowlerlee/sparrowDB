use buffer::bufferpoolmanager::BufferPoolManager;
use buffer::catalog::Catalog;
use buffer::query_types::{get_demo_schema, get_demo_table_heap_with_n_page_m_tuples_each, Tuple};
use common::transaction::Transaction;
use std::sync::{Arc, Mutex};

use std::io::{self, Write};

#[allow(dead_code)]
enum Statements {
    SELECT(String),
    CREATE(String),
}

fn make_kestreldb_logo(){
    print!(
    r#"
....................... ...........................
..................           .....  .  ............
............  .  .   @@@@@@@    .......  ..........
..............    :@  @@@@  @     ....... .  ......
...........  .   @% @  @@@ @ @   .  .  .. ... .....
........  .    @@      @@  @@@        . .. ........
..........    @ #@@#@@@  @@              .  ..  ...
........    +@  @@ @   @  @. @@@@@@@@@@@      .. ..
......     @@*    @ @  @# @  @@@@@@@@@@@@@@@@    . 
........  @@@    @ @ @ @ *@   ..@@@@..... @@@@@  ..
......    @ +  @   @  @  @    .  .@@@.  ... @@@@@  
..... .  @*@   @@   @@  @ #      .  @@@. .... @@@@ 
..... .  @ @   .  @@@ @@ %@  .  .    @@@    .  @@@ 
....    @#@   @@@@@  @@ @@@      ... .@@@ .    @@@ 
.....   @@@@@@ @@@  @   @@@ .  .   .  @@@  . .-@@@.
.....  :@ @@@ @@%  @    @@@     ..    @@@.    @@@  
...    @# @ @@@@@@@@@   @@@          .=@@   @@@@  .
..    @ @@ @@@ @    @@@@@@        .   :@@@@@@@:    
.. . @@@ @@@   @  @@=  @@@@  @@@@@@@@@@@@@@@@    . 
..  #@@ @@@ @@ @  @     @@@   .........@@*@@@@@@   
.   @ @@ %@ @ @@        @@@   .  .  .  @@@   @@@@  
.  @@@@ @@@ @ @@      . @@@ ..  .... ..@@@... @@@@.
  @@ @.@ @  @ @         @@@    .  ..   @@@  .  @@@@
.   @@*  @ @@ @..   ..  @@@  ..  .  ...@@:.  . +@@@
. ...    @ @  @  ..   . @@@ .  ..  .  @@@ .. . @@@@
...   . @@ @  @ .  .  . @@@ .  ....  @@@ .     @@@=
.. .. . @  @ -@ .  .    @@@  .    .@@@ ..  .. @@@@ 
......  @ @@@@  ..      @@@  .  @@@@    .   *@@@@  
...  . :@ @@@    .. ..  @@@  @@@#   ..... @@@@@:   
...... @@:@    .      . @@@@@@@@@@@@@@@@@@@@@      
....   =@@     .. ..  . @@@@@@@@@@@@@@@@@          
....... @   ... ...... .  . ....... .  ... ........
......    ............ .. .     . .  ... ..........
........  .. ..........  ........ ..... ...........
................... ...... ........ ...............
    
              KestrelDB

    "#

    )
}

fn main() {
    print!("start-up");
    make_kestreldb_logo();

    println!("Enter a command (SELECT, CREATE, or EXIT to quit):");

    loop {
        print!(" > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let upper = input.to_uppercase();
        let input = upper.split_whitespace().collect::<Vec<&str>>();

        let mut bpm = BufferPoolManager::new(10, 2);
        bpm.table_heap = get_demo_table_heap_with_n_page_m_tuples_each(10, 10);
        let mut catalog = Arc::new(Mutex::new(Catalog::new()));
        catalog.lock().unwrap().bpm = bpm;
        let fake = Arc::clone(&catalog);

        match input[0] {
            "/DT" => show_table(fake),
            "SELECT" => handle_select(fake, input.clone()),
            "CREATE" => handle_create(fake, input.clone()),
            "EXIT" => {
                println!("Goodbye from KestrelDB!");
                break;
            }
            _ => println!("Unknown command. Try SELECT, CREATE, or EXIT."),
        }
    }
}

fn show_table(catalog: Arc<Mutex<Catalog>>) {
    let guard = catalog.lock().unwrap();
    guard.get_table(None);
}

fn handle_select(catalog: Arc<Mutex<Catalog>>, input: Vec<&str>) {
    let guard = catalog.lock().unwrap();

    if input[2].to_string() != "FROM" {
        return;
    }
    let _table = guard.get_table(Some(input[3].to_string()));
    println!(
        "{:?}",
        guard
            .bpm
            .table_heap
            .data
            .iter()
            .flat_map(|table| table.data.clone())
            .collect::<Vec<Tuple>>()
    )
}

fn handle_create(catalog: Arc<Mutex<Catalog>>, input: Vec<&str>) {
    let transaction = Transaction::new();
    let table_name = input[1].to_string();
    let schema = get_demo_schema();
    let mut guard = catalog.lock().unwrap();
    let table_info = guard.create_table(transaction, table_name, schema, true);
    println!("{:?}", table_info);
}
