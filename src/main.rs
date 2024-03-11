use std::io;
use std::io::{stdout, Write};

extern crate rusqlite;
use rusqlite::{Connection, Result};
use sha2::{Sha512, Digest};

struct User {
    id: u64,
    username: String,
    password: String,
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
        "SELECT id, username, password from users;",
    )?;

    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    for user in users {
        let item = user.unwrap();
        println!("Id: {} Username: {} Password: {}", item.id, item.username, item.password);
    }

    Ok(())
}

fn register_user(conn: &Connection, username: String, password: String) -> Result<()> {
   conn.execute(
       "INSERT INTO users (username, password) values (?1, ?2)",
       &[&username.to_string(), &password.to_string()],
   )?;

   Ok(())
}

fn setup_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists users (
             id integer primary key,
             username text not null unique,
             password text not null
         )",
        [],
    )?;

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

fn main() -> Result<()> {
    print!("Enter your username: ");
    let username = get_user_input();
    print!("Enter your password: ");
    let password = get_user_input();

    let conn = Connection::open("users.db")?;
    setup_db(&conn).expect("Setup DB failed!");
    register_user(&conn, username, encode_password(password)).expect("User adding failed!");
    get_data_from_db(&conn).expect("Select data failed!");

    Ok(())
}
