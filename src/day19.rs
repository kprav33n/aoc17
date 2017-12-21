pub fn trace_path(input: &str) -> String {
    let chars: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut x = 0;
    let (mut y, _) = chars[x].iter().enumerate().find(|&t| *t.1 == '|').unwrap();
    let mut direction = Direction::Down;
    let mut path: Vec<char> = vec![];

    loop {
        match &direction {
            &Direction::Down => {
                match chars[x][y] {
                    ' ' => break,
                    '-' | '|' => x += 1,
                    '+' => {
                        if y < chars[x].len() - 1 && chars[x][y + 1] != ' ' {
                            direction = Direction::Right;
                            y += 1;
                        } else if y > 0 && chars[x][y - 1] != ' ' {
                            direction = Direction::Left;
                            y -= 1;
                        } else {
                            break;
                        }
                    },
                    _ => {
                        path.push(chars[x][y]);
                        x += 1;
                    }
                }
            }
            &Direction::Up => {
                match chars[x][y] {
                    ' ' => break,
                    '-' | '|' => x -= 1,
                    '+' => {
                        if y < chars[x].len() - 1 && chars[x][y + 1] != ' ' {
                            direction = Direction::Right;
                            y += 1;
                        } else if y > 0 && chars[x][y - 1] != ' ' {
                            direction = Direction::Left;
                            y -= 1;
                        } else {
                            break;
                        }
                    },
                    _ => {
                        path.push(chars[x][y]);
                        x -= 1;
                    }
                }
            }
            &Direction::Right => {
                match chars[x][y] {
                    ' ' => break,
                    '-' | '|' => y += 1,
                    '+' => {
                        if x < chars.len() - 1 && chars[x + 1][y] != ' ' {
                            direction = Direction::Down;
                            x += 1;
                        } else if x > 0 && chars[x - 1][y] != ' ' {
                            direction = Direction::Up;
                            x -= 1;
                        } else {
                            break;
                        }
                    },
                    _ => {
                        path.push(chars[x][y]);
                        y += 1;
                    }
                }
            }
            &Direction::Left => {
                match chars[x][y] {
                    ' ' => break,
                    '-' | '|' => y -= 1,
                    '+' => {
                        if x < chars.len() - 1 && chars[x + 1][y] != ' ' {
                            direction = Direction::Down;
                            x += 1;
                        } else if x > 0 && chars[x - 1][y] != ' ' {
                            direction = Direction::Up;
                            x -= 1;
                        } else {
                            break;
                        }
                    },
                    _ => {
                        path.push(chars[x][y]);
                        y -= 1;
                    }
                }
            }
        }
    }
    path.iter().collect()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
