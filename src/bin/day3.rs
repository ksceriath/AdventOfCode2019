use std::fs;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
    wire_length: i32,
}

impl LineSegment {
    fn new(start: Point, end: Point, wire_length: i32) -> LineSegment {
        LineSegment {
            start,
            end,
            wire_length,
        }
    }
}

fn process_input(input: String) -> Vec<Vec<LineSegment>> {
    let input = input
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    input
        .iter()
        .map(|s| {
            let mut current = Point { x: 0, y: 0 };
            let mut wire_length = 0;
            s.split(",")
                .map(|m| {
                    let mut m = String::from(m);
                    let direction = m.remove(0);
                    let distance = m.parse::<i32>().unwrap();
                    let target = match direction {
                        'R' => Point {
                            x: current.x + distance,
                            y: current.y,
                        },
                        'U' => Point {
                            x: current.x,
                            y: current.y + distance,
                        },
                        'L' => Point {
                            x: current.x - distance,
                            y: current.y,
                        },
                        'D' => Point {
                            x: current.x,
                            y: current.y - distance,
                        },
                        _ => panic!("Move out of the world!"),
                    };
                    wire_length =
                        wire_length + (target.x - current.x).abs() + (target.y - current.y).abs();
                    let line = LineSegment::new(
                        Point {
                            x: current.x,
                            y: current.y,
                        },
                        Point {
                            x: target.x,
                            y: target.y,
                        },
                        wire_length,
                    );
                    current = target;
                    line
                })
                .collect::<Vec<LineSegment>>()
        })
        .collect::<Vec<Vec<LineSegment>>>()
}

fn main() {
    let mut input = fs::read_to_string("resources/day3.input").unwrap();
    input.pop();

    let line_segments = process_input(input);

    let closest_intersection =
        smallest_wirelength_intersection(&line_segments[0], &line_segments[1]);

    println!("intersection distance: {}", closest_intersection);
}

fn smallest_wirelength_intersection(set1: &Vec<LineSegment>, set2: &Vec<LineSegment>) -> i32 {
    let mut smallest_wirelength_intersection = i32::max_value();
    for line1 in set1 {
        for line2 in set2 {
            if let Some(point) = intersect_lines(&line1, &line2) {
                let wirelength = get_total_wirelength(&line1, &line2, &point);

                if smallest_wirelength_intersection > wirelength && (point.x != 0 || point.y != 0) {
                    smallest_wirelength_intersection = wirelength;
                }
            }
        }
    }
    smallest_wirelength_intersection
}

fn get_segment_length(line: &LineSegment) -> i32 {
    if is_vertical(line) {
        (line.end.y - line.start.y).abs()
    } else {
        (line.end.x - line.start.x).abs()
    }
}

fn manhattan_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.y - point2.y).abs() + (point1.x - point2.x).abs()
}

fn get_total_wirelength(wire1: &LineSegment, wire2: &LineSegment, intersection: &Point) -> i32 {
    let wire1_segment = get_segment_length(wire1);
    let wire2_segment = get_segment_length(wire2);

    wire1.wire_length - wire1_segment + wire2.wire_length - wire2_segment
        + manhattan_distance(&wire1.start, intersection)
        + manhattan_distance(&wire2.start, intersection)
}

fn _manhattan_point(set1: &Vec<LineSegment>, set2: &Vec<LineSegment>) -> Point {
    let mut manhattan_distance = i32::max_value();
    let mut manhattan = Point { x: 0, y: 0 };
    for line1 in set1 {
        for line2 in set2 {
            if let Some(point) = intersect_lines(&line1, &line2) {
                if manhattan_distance > point.x.abs() + point.y.abs()
                    && (point.x != 0 || point.y != 0)
                {
                    manhattan_distance = point.x.abs() + point.y.abs();
                    manhattan = point;
                }
            }
        }
    }
    manhattan
}

fn point_of_intersection(vertical: &LineSegment, horizontal: &LineSegment) -> Option<Point> {
    let (h_start, h_end) = sort_points(horizontal);
    let (v_start, v_end) = sort_points(vertical);
    if v_start.x < h_start.x || v_start.x > h_end.x || h_start.y > v_end.y || h_start.y < v_start.y
    {
        None
    } else {
        Some(Point {
            x: v_start.x,
            y: h_start.y,
        })
    }
}

fn is_vertical(line: &LineSegment) -> bool {
    line.start.x == line.end.x
}

fn sort_points<'a>(line1: &'a LineSegment) -> (&'a Point, &'a Point) {
    if line1.start.x < line1.end.x || line1.start.y < line1.end.y {
        (&line1.start, &line1.end)
    } else {
        (&line1.end, &line1.start)
    }
}

fn intersect_lines(line1: &LineSegment, line2: &LineSegment) -> Option<Point> {
    let (line1_low, line1_high) = sort_points(line1);
    let (line2_low, line2_high) = sort_points(line2);

    match (is_vertical(line1), is_vertical(line2)) {
        (true, true) => {
            if line1_low.x == line2_low.x {
                if line1_low.y <= line2_low.y && line1_high.y >= line2_low.y {
                    Some(Point {
                        x: line2_low.x,
                        y: line2_low.y,
                    })
                } else if line2_low.y <= line1_low.y && line2_high.y >= line1_low.y {
                    Some(Point {
                        x: line1_low.x,
                        y: line1_low.y,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
        (false, false) => {
            if line1_low.y == line2_low.y {
                if line1_low.x <= line2_low.x && line1_high.x >= line2_low.x {
                    Some(Point {
                        x: line2_low.x,
                        y: line2_low.y,
                    })
                } else if line2_low.x <= line1_low.x && line2_high.x >= line1_low.x {
                    Some(Point {
                        x: line1_low.x,
                        y: line1_low.y,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
        (true, false) => point_of_intersection(line1, line2),
        (false, true) => point_of_intersection(line2, line1),
    }
}
