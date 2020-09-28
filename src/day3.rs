
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Eq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

trait Distance {
    fn length(&self) -> i32;
}
trait PointDistance {
    fn steps_to(&self, point:&Self) -> i32;
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} - {})", self.start, self.end)
    }
}
impl Distance for Line {
    fn length(&self) -> i32 {
        self.start.steps_to(&self.end)
    }
}
impl PointDistance for Point {
    fn steps_to(&self, point:&Self) -> i32 {
        let startx  = Ord::min(self.x, point.x);
        let starty  = Ord::min(self.y, point.y);
        let endx    = Ord::max(self.x, point.x);
        let endy    = Ord::max(self.y, point.y);

        endy - starty + endx - startx
    }

}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        ( self.x.abs() + self.y.abs() ).cmp( &(other.x.abs() + other.y.abs()) )
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}


fn string_to_lines(s: &str) -> Vec<Line> {
    let mut x=0;
    let mut y=0;
    let mut vec:Vec<Line> = Vec::new();

    for l in s.split(",") {
        let dir = l.get(0..1).unwrap();
        let steps = l.get(1..).unwrap().parse::<i32>().unwrap();
        let start:Point = Point {x, y};

        match dir {
            "L" => x -= steps,
            "R" => x += steps,
            "U" => y += steps,
            "D" => y -= steps,
            _ => panic!("Invalid data")
        };
        let end:Point = Point {x, y};
        vec.push(Line{start, end});
    }
    return vec;

}


fn crosses(l1: &Line, l2: &Line) -> Option<Point> {
    
    let start1x = Ord::min(l1.start.x, l1.end.x);
    let end1x   = Ord::max(l1.start.x, l1.end.x);
    
    let start2x = Ord::min(l2.start.x, l2.end.x);
    let end2x   = Ord::max(l2.start.x, l2.end.x);
    
    let r1x = start1x..end1x+1;
    let r2x = start2x..end2x+1;

    let start1y = Ord::min(l1.start.y, l1.end.y);
    let end1y   = Ord::max(l1.start.y, l1.end.y);
    
    let start2y = Ord::min(l2.start.y, l2.end.y);
    let end2y   = Ord::max(l2.start.y, l2.end.y);
    
    let r1y = start1y..end1y+1;
    let r2y = start2y..end2y+1;

    let mut resx:Option<i32> = None;
    let mut resy:Option<i32> = None;
    
    for x in r1x {
        if r2x.contains(&x) {
            resx = Some(x);
            break;
        }
    }
    for y in r1y {
        if r2y.contains(&y) {
            resy = Some(y);
            break;
        }
    }
    // We don't want the 0,0 crossing, at least x or y should me non-zero
    match (resx, resy) {
        (Some(x), Some(y)) if x != 0 || y!= 0 => Some(Point{x:x, y:y}),
        (_, _) => None,
    }
}
fn get_crosses(vl1: &Vec<Line>, vl2: &Vec<Line>) -> Vec<Point> {
    let mut cross_points:Vec<Point> = Vec::new();
    for l1 in vl1 {
        for l2 in vl2 {
            let c = crosses(&l1, &l2);
            match c {
                Some(x) => cross_points.push(x),
                None => continue
            };
        }
    }
    
    cross_points
}

fn get_crosses2(vl1: &Vec<Line>, vl2: &Vec<Line>) -> Vec<(Point, i32)> {
    let mut cross_points:Vec<(Point, i32)> = Vec::new();
    
    let mut sofar_l1 = 0;
    for l1 in vl1 {
        let mut sofar_l2 = 0;
        for l2 in vl2 {
            match crosses(&l1, &l2) {
                Some(x) => {
                    let steps_her = l1.start.steps_to(&x) + l2.start.steps_to(&x);
                    let steps = sofar_l1 + sofar_l2 + steps_her;
            
                    cross_points.push((x, steps))
                },
                _ => sofar_l2 += l2.length()
            };
            

        }
        sofar_l1 += l1.length();
    }
    
    cross_points
}
fn get_min_eucledian(vl1: &Vec<Line>, vl2: &Vec<Line>) -> Option<i32> {
    let all_crosses = get_crosses(vl1, vl2);
    let min = all_crosses.iter().min_by(|x,y| x.cmp(y) );
    
    match min.iter().min() {
        Some(x) => Some(x.x.abs() + x.y.abs()),
        None => None
    }
}

fn steps_away(vl1: &Vec<Line>, vl2: &Vec<Line>) -> Option<i32> {
    let all_crosses = get_crosses2(vl1, vl2);
    
    let min = all_crosses.iter().min_by(|(_, n),(_, ny)| n.cmp(ny) );
    
    match min {
        Some( (_, u) ) => Some(*u) ,
        _ => None
    }
}
#[cfg(test)]
#[test]
fn test_line_crosses() {
    let l1 = Line{ start:Point{x:1, y:1}, end:Point{x:1, y:5}};
    let l2 = Line{ start:Point{x:-2, y:3}, end:Point{x:3, y:3}};
    assert_eq!(Some(Point{x:1, y:3}), crosses(&l1, &l2));
    
    let l1 = Line{ start:Point{x:1, y:1}, end:Point{x:1, y:5}};
    let l2 = Line{ start:Point{x:-10, y:3}, end:Point{x:0, y:3}};
    assert_eq!(None, crosses(&l1, &l2));
    
    let l1 = Line{ start:Point{x:1, y:1}, end:Point{x:1, y:5}};
    let l2 = Line{ start:Point{x:10, y:3}, end:Point{x:0, y:3}};
    assert_eq!(Some(Point{x:1, y:3}), crosses(&l1, &l2));
    
    let l1 = Line{ start:Point{x:1, y:5}, end:Point{x:1, y:1}};
    let l2 = Line{ start:Point{x:10, y:3}, end:Point{x:0, y:3}};
    assert_eq!(Some(Point{x:1, y:3}), crosses(&l1, &l2));
    
    let l1 = Line{ start:Point{x:1, y:1}, end:Point{x:1, y:5}};
    let l2 = Line{ start:Point{x:-2, y:6}, end:Point{x:3, y:6}};
    assert_eq!(None, crosses(&l1, &l2));

    let l1 = Line{ start:Point{x:1, y:1}, end:Point{x:5, y:1}};
    let l2 = Line{ start:Point{x:2, y:-3}, end:Point{x:2, y:3}};
    assert_eq!(Some(Point{x:2, y:1}), crosses(&l1, &l2));
    
}

#[test]
fn test_eucledian() {
    let vl1 = string_to_lines("R8,U5,L5,D3");
    let vl2 = string_to_lines("U7,R6,D4,L4");
    assert_eq!(Some(6), get_min_eucledian(&vl1, &vl2));

    let vl1 = string_to_lines("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let vl2 = string_to_lines("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(Some(135), get_min_eucledian(&vl1, &vl2));
    
    let vl1 = string_to_lines("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let vl2 = string_to_lines("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(Some(159), get_min_eucledian(&vl1, &vl2));
    
}
#[test]
fn test_steps(){
    let vl1 = string_to_lines("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let vl2 = string_to_lines("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(Some(610), steps_away(&vl1, &vl2));
    
    let vl1 = string_to_lines("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let vl2 = string_to_lines("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(Some(410), steps_away(&vl1, &vl2));
}

pub fn run() -> io::Result<()> {
    
    let file = File::open("inputs/day3.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let vl1 = match lines.next().unwrap() {
        Ok(l) => string_to_lines(&l),
        Err(e) => panic!("No line data: {}", e)
    };
    let vl2 = match lines.next().unwrap() {
        Ok(l) => string_to_lines(&l),
        Err(e) => panic!("No line data: {}", e)
    };
    
    match get_min_eucledian(&vl1, &vl2) {
        Some(x) => println!("Min eucledian distance: {}", x),
        None => println!("No result")
    };
    
    match steps_away(&vl1, &vl2) {
        Some( x ) => println!("Fastest point {} steps away", x) ,
        None => println!("No result")
    };
    
    Ok(())
}