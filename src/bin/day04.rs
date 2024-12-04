use aoc_2024::load_file;

fn main() -> anyhow::Result<()> {
    let load_file = load_file(4);
    let lines: Vec<&str> = load_file.lines().collect();

    let increment: Vec<(i8, i8)> = vec![
        (1,0), // forward
        (-1, 0), //backward
        (0,1), //down
        (0,-1), // up
        (1,1), //forward-bottom
        (-1,-1), //backward-up
        (-1,1), //backward-bottom
        (1,-1), //forward-up
    ];

    let word = "XMAS";

    let mut words = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            for inc in &increment {
                //check if possible
                let (last_x, last_y) = (x as i32 + word.len() as i32 * inc.0 as i32, y as i32 + word.len() as i32 * inc.0 as i32);
                if last_x < 0 || last_x >= lines[y].len() as i32 || last_y < 0 || last_y >= lines.len() as i32 {
                    continue; 
                }

                let mut curr_x = x as i32;
                let mut curr_y = y as i32;

                //try search
                let mut found = true;
                for c in word.chars() {
                    if lines[curr_y as usize].chars()[curr_x as usize] != c {
                        found = false;
                        break;
                    }

                    curr_x += inc.0 as i32;
                    curr_y += inc.1 as i32;
                }

                if !found {
                    continue;
                }   
                words += 1;
            }    
        }
    }
    //println!("1. star {safe}");
    //println!("2. star {safe_module}");

    Ok(())
}
