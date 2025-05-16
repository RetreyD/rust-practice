#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, // верхня ліва
    b: Point, // нижня права
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    if xs.is_empty() {
        return 0;
    }

    // Знаходимо межі
    let min_x = xs.iter().map(|r| r.a.x.min(r.b.x)).min().unwrap();
    let max_x = xs.iter().map(|r| r.a.x.max(r.b.x)).max().unwrap();
    let min_y = xs.iter().map(|r| r.a.y.min(r.b.y)).min().unwrap();
    let max_y = xs.iter().map(|r| r.a.y.max(r.b.y)).max().unwrap();

    // Розмір сітки
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    // Сітка зайнятості (false - вільно, true - зайнято)
    let mut grid = vec![vec![false; width]; height];

    for rect in xs {
        // Корекція координат у сітці
        let left = rect.a.x.min(rect.b.x) - min_x;
        let right = rect.a.x.max(rect.b.x) - min_x;
        let top = max_y - rect.a.y.max(rect.b.y);    // y-координата "зверху" (інвертуємо по y)
        let bottom = max_y - rect.a.y.min(rect.b.y); // y-координата "знизу"

        for y in top..=bottom {
            for x in left..=right {
                grid[y as usize][x as usize] = true;
            }
        }
    }

    // Підрахунок зайнятих пікселів
    grid.iter()
        .map(|row| row.iter().filter(|&&occupied| occupied).count() as i32)
        .sum()
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
