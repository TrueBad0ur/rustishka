use std::{io, thread, time};
use std::io::{stdout, Write, Read};

extern crate rusqlite;
use rusqlite::{Connection, params, Result};
use sha2::{Sha512, Digest};

struct User {
    id: u64,
    username: String,
    password: String,
    money: u64,
}

struct Item {
    id: u64,
    product: String,
    price: u32,
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn add_money(conn: &Connection, global_username: &String) -> Result<()> {
    //let amount:u64 = get_user_input().parse::<i32>().unwrap();
    let query = "UPDATE users SET money = money + ?1 WHERE username = ?2";

    conn.execute(
        query, params![5, &global_username],
    )?;

    println!("5 points added to your account!");
    pause(3);
    clear_console();
    Ok(())
}

fn get_profile(conn: &Connection, global_username: &String) -> Result<()> {

    let query = "SELECT id, username, password, money from users WHERE username = ?1";

    let mut stmt = conn.prepare(
        query,
    )?;

    let users = stmt.query_map(params![&global_username.as_str()], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            money: row.get(3)?,
        })
    })?;
    clear_console();
    for user in users {
        let item = user.unwrap();
        println!("Username: {}\nMoney: {}\n", item.username, item.money);
    }
    pause(3);
    clear_console();
    Ok(())
}

fn logo() {
    print!("--- rustishka ---")
}

#[allow(dead_code)]
fn pause_v2() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn pause(seconds: u64) {
    let _time = time::Duration::from_secs(seconds);
    thread::sleep(_time);
}

fn encode_password(password: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut hash_string = "".to_string();

    for byte in result.iter() {
        hash_string = format!("{}{:02x}", hash_string, byte);
    }

    hash_string
}

fn get_data_from_db(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password, money from users;",
    )?;

    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            money: row.get(3)?,
        })
    })?;

    for user in users {
        let item = user.unwrap();
        println!("Id: {} Username: {} Password: {} Money: {}", item.id, item.username, item.password, item.money);
    }
    pause(3);
    clear_console();
    Ok(())
}

fn register_user(conn: &Connection) -> Result<()> {
    print!("Enter your username: ");
    let username = get_user_input();
    print!("Enter your password: ");
    let password = encode_password(get_user_input());

    let query = "SELECT id, username, password, money from users WHERE username = ?1";
    let mut stmt = conn.prepare(
        query,
    )?;

    let user = stmt.query_map(params![&username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            money: row.get(3)?,
        })
    })?;

    let number_of_rows = user.count();
    if number_of_rows == 0 {
        conn.execute(
            "INSERT INTO users (username, password, money) values (?1, ?2, ?3)",
            &[&username.to_string(), &password.to_string(), &0.to_string()],
        ).expect("Unexpected sql error!");
    } else {
        println!("Such user already registered!\nTry another one!");
        pause(2);
    }

    clear_console();
    Ok(())
}

fn register_admin(conn: &Connection) -> Result<()> {
    let username = String::from("admin");
    let password = encode_password(String::from("admin"));

    conn.execute(
        "INSERT OR IGNORE INTO users (username, password, money) values (?1, ?2, ?3)",
        &[&username.to_string(), &password.to_string(), &0.to_string()],
    ).expect("Unexpected sql error!");


    clear_console();
    Ok(())
}

fn login_user(conn: &Connection, logged: &mut bool, global_username: &mut String) -> Result<()> {
    print!("Enter your username: ");
    let username = get_user_input();
    print!("Enter your password: ");
    let password = encode_password(get_user_input());

    let query = "SELECT id, username, password, money from users WHERE username = ?1";

    let mut stmt = conn.prepare(
        query,
    )?;

    let user = stmt.query_map(params![&username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            money: row.get(3)?,
        })
    })?;

    for line in user {
        let to_compare = line.unwrap().password;
        if password == to_compare {
            *global_username = username.clone();
            *logged = true;
            clear_console();
            println!("User {} successfully authorized!", global_username);
            pause(2);

            clear_console();
            return Ok(())
        }
    }
    println!("Wrong username or password!");
    pause(2);

    clear_console();
    Ok(())
}

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists users (
             id integer primary key,
             username text not null unique,
             password text not null,
             money integer
         )",
        [],
    )?;

    conn.execute(
        "create table if not exists shop (
             id integer primary key,
             product text not null unique,
             price integer
         )",
        [],
    )?;

    let items = [
        ("item1", 100),
        ("item2", 200),
        ("item3", 300),
    ];

    for (item, price) in items {
        conn.execute(
            "INSERT OR IGNORE INTO shop (product, price) values (?1, ?2)",
            &[&item.to_string(), &price.to_string()],
        ).expect("Unexpected sql error!");
    }

    clear_console();
    Ok(())
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    let _=stdout().flush();
    io::stdin().read_line(&mut user_input).expect("error: unable to read user input");
    if let Some('\n')=user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r')=user_input.chars().next_back() {
        user_input.pop();
    }

    user_input
}

fn logout_user(logged: &mut bool) -> Result<()> {
    *logged = false;
    clear_console();
    Ok(())
}

fn buy(conn: &Connection) -> Result<()> {
    loop {
        shop(conn).expect("Printing shop failed!");
        println!("Enter 0 if you wanna exit shop");
        print!("Enter ID of the item you wanna buy> ");
        let id = get_user_input().chars().nth(0).unwrap();
        if id as u64 <= 0 {
            break
        }
        print!("Enter the amount of item you wanna buy> ");
        let amount = get_user_input().chars().nth(0).unwrap();
        if amount as u64 <= 0 {
            break
        }
    }

    todo!();

    Ok(())
}

fn shop(conn: &Connection) -> Result<()> {
    clear_console();
    let mut stmt = conn.prepare(
        "SELECT id, product, price from shop;",
    )?;

    let items = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            product: row.get(1)?,
            price: row.get(2)?,
        })
    })?;

    print!("
┏━━━━┳━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━┓
┃id  ┃product              ┃price    ┃
┣━━━━╋━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━┫");
    for item in items {
        let item_unwr = item.unwrap();
        print!("
┃{}   ┃{}                ┃{}      ┃\n┣━━━━╋━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━┫", item_unwr.id, item_unwr.product, item_unwr.price);
    }
    print!("
┗━━━━┻━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━┛\n");

    Ok(())
}

fn main() -> Result<()> {
    clear_console();
    let mut logged_in = false;
    let mut global_username: String = "".to_string();
    let conn = Connection::open("users.db")?;
    setup_db(&conn).expect("Setup DB failed!");
    register_admin(&conn).expect("User admin registering failed!");

    loop {
        if !logged_in {
            logo();
            print!("\n1 - register\n2 - login\n0 - exit\n\n>");
            let letter = get_user_input().chars().nth(0).unwrap();
            match letter {
                '1' => register_user(&conn).expect("User registering failed!"),
                '2' => login_user(&conn, &mut logged_in, &mut global_username).expect("User login failed!"),
                '0' => break,
                _ => println!("wrong!")
            }
        } else if &global_username == "admin" && logged_in {
            logo();
            println!("\nProfile: {}", global_username);
            print!("\n1 - my profile\n2 - get money\n3 - logout\n4 - get users data from db\n5 - shop\n0 - exit\n\n>");
            let letter = get_user_input().chars().nth(0).unwrap();
            match letter {
                '1' => get_profile(&conn, &global_username).expect("Getting profile failed!"),
                '2' => add_money(&conn, &global_username).expect("Getting money failed!"),
                '3' => logout_user(&mut logged_in).expect("User logout failed!"),
                '4' => get_data_from_db(&conn).expect("Data selecting failed!"),
                '5' => buy(&conn).expect("Opening shop failed!"),
                '0' => break,
                _ => println!("wrong!")
            }
        } else if logged_in {
            logo();
            println!("\nProfile: {}", global_username);
            print!("\n1 - my profile\n2 - get money\n3 - logout\n4 - shop\n0 - exit\n\n>");
            let letter = get_user_input().chars().nth(0).unwrap();
            match letter {
                '1' => get_profile(&conn, &global_username).expect("Getting profile failed!"),
                '2' => add_money(&conn, &global_username).expect("Getting money failed!"),
                '3' => logout_user(&mut logged_in).expect("User logout failed!"),
                '4' => buy(&conn).expect("Opening shop failed!"),
                '0' => break,
                _ => println!("wrong!")
            }
        }
    }

    //register_user(&conn).expect("User adding failed!");
    //get_data_from_db(&conn).expect("Select data failed!");

    let _err=conn.close();
    Ok(())
}
