use x11rb::{connect, connection::Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;
    loop {
        println
    }
    Ok(())
}
