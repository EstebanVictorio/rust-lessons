struct Player {
    game_id: u32,
    gamer_name: String,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);


fn main() {
    let player1 = Player { game_id: 12345, gamer_name: String::from("Niphel"), active: false };
    let player2 = Player { game_id: 67890, gamer_name: String::from("Aura"), active: true };
    let player3 = Player { game_id: 11111, gamer_name: String::from("Blaster"), ..player2 };
    let black = Color(0,0,0);
    let point = Point(0,0,0);
    println!("ID: {}, Name: {}, Active: {}", player1.game_id, player1.gamer_name, player1.active);
    println!("ID: {}, Name: {}, Active: {}", player2.game_id, player2.gamer_name, player2.active);
    println!("ID: {}, Name: {}, Active: {}", player3.game_id, player3.gamer_name, player3.active);
    println!("Point: {}, {}, {}", point.0, point.1, point.2);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);
}
