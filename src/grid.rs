#[derive(Serialize, Deserialize)]
pub struct Grid {
    pub grid: Vec<Vec<i8>>
}

impl Grid {
    pub fn new() -> Grid {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        g
    }
    
    pub fn update(&mut self, pos_x: usize, pos_y: usize, value: i8) {
        println!(">>>>>>> {} <=> {} => {}", pos_x, pos_y, value);
        match self.grid.get_mut(pos_x) {
            Some(x) => {
                x[pos_y] = value;
            },
            None => {
                return
            }
        }
    }
}

fn exist(slice: &[Vec<i8>], pos_x: usize, pos_y: usize, value: i8) -> bool {
    match slice.get(pos_x) {
        Some(x) => {
            match x.get(pos_y) {
                Some(y) => {
                    println!("ok {}, {} <==> {}", pos_x, *y, value);
                    if *y == value {
                        return true;
                    }
                    return false;
                },
                None => {
                    return false;
                }
            };
        },
        None => {
            return false;
        }
    };
}

pub fn win(slice: &[Vec<i8>], pos_x: usize, pos_y: usize, new_value: i8) -> bool {
    // horizontal test
    let mut x_min = pos_x;
    let mut x_max = pos_x;
    loop {
        if x_min == 0 {
            break;
        }
        x_min -= 1;
        if !exist(slice, x_min, pos_y, new_value) {
            x_min += 1;
            break;
        }
    }
    loop {
        x_max += 1;
        if !exist(slice, x_max, pos_y, new_value) {
            x_max -= 1;
            break;
        }
    }
    println!("#1 {} <=> {} => {}", x_max, x_min, x_max - x_min);
    if x_max - x_min >= 3 {
        return true;
    }
    // vertical test
    let mut y_min = pos_y;
    let mut y_max = pos_y;
    loop {
        if y_min == 0 {
            break;
        }
        y_min -= 1;
        if !exist(slice, pos_x, y_min, new_value) {
            y_min += 1;
            break;
        }
    }
    loop {
        y_max += 1;
        if !exist(slice, pos_x, y_max, new_value) {
            y_max -= 1;
            break;
        }
    }
    println!("#2 {} <=> {} => {}", y_max, y_min, y_max - y_min);
    if y_max - y_min >= 3 {
        return true;
    }
    // diagonal 1
    x_min = pos_x;
    x_max = pos_x;
    y_min = pos_y;
    y_max = pos_y;
    loop {
        if x_min == 0 || y_min == 0 {
            break;
        }
        x_min -= 1;
        y_min -= 1;
        if !exist(slice, x_min, y_min, new_value) {
            x_min += 1;
            y_min += 1;
            break;
        }
    }
    loop {
        x_max += 1;
        y_max += 1;
        if !exist(slice, x_max, y_max, new_value) {
            x_max -= 1;
            y_max -= 1;
            break;
        }
    }
    println!("#3 {} <=> {} , {} <=> {} => {} <=> {}", x_min, x_max, y_min, y_max, x_max - x_min, y_max - y_min);
    if x_max - x_min >= 3 && y_max - y_min >= 3 {
        return true;
    }
    // diagonal 2
    x_min = pos_x;
    x_max = pos_x;
    y_min = pos_y;
    y_max = pos_y;
    loop {
        if x_min == 0 {
            break;
        }
        x_min -= 1;
        y_max += 1;
        if !exist(slice, x_min, y_max, new_value) {
            x_min += 1;
            y_max -= 1;
            break;
        }
    }
    loop {
        if y_min == 0 {
            break;
        }
        x_max += 1;
        y_min -= 1;
        if !exist(slice, x_max, y_min, new_value) {
            x_max -= 1;
            y_min += 1;
            break;
        }
    }
    println!("#4 {} <=> {} , {} <=> {} => {} <=> {}", x_min, x_max, y_min, y_max, x_max - x_min, y_max - y_min);
    if x_max - x_min >= 3 && y_max - y_min >= 3 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_test1() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(false, win(&g.grid, 2, 2, 2))
    }

    #[test]
    fn horizontal_test2() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 1, 1, 1, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(true, win(&g.grid, 1, 1, 1))
    }

    #[test]
    fn vertical_test1() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 1, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 1, 0, 1, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 0, 1, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 0, 0, 0, 1, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(true, win(&g.grid, 1, 4, 1))
    }

    #[test]
    fn vertical_test2() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 1, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 1, 0, 1, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 0, 1, 0, 1, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(false, win(&g.grid, 3, 3, 1))
    }

    #[test]
    fn vertical_test3() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 1, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 1, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(false, win(&g.grid, 3, 3, 1))
    }
    
    #[test]
    fn diagonal_test1() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 1, 1, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 1, 0, 1, 0];
        g.grid.push(vec2);
        let vec3 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![1, 0, 1, 0, 1, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(true, win(&g.grid, 2, 1, 1))
    }
    
    #[test]
    fn diagonal_test2() {
        let mut g = Grid { grid: Vec::new() };
        let vec1 = vec![0, 0, 0, 0, 1, 0];
        g.grid.push(vec1);
        let vec2 = vec![0, 0, 0, 0, 1, 0];
        g.grid.push(vec2);
        let vec3 = vec![1, 0, 0, 0, 0, 0];
        g.grid.push(vec3);
        let vec4 = vec![0, 1, 0, 0, 1, 0];
        g.grid.push(vec4);
        let vec5 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec5);
        let vec6 = vec![0, 0, 0, 1, 0, 0];
        g.grid.push(vec6);
        let vec7 = vec![0, 0, 0, 0, 0, 0];
        g.grid.push(vec7);
        assert_eq!(true, win(&g.grid, 4, 2, 1))
    }
}
