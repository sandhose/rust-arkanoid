pub struct Point {pub x: f32, pub y: f32}
pub type Pixels = f32;

pub fn distance(p1: &Point, p2: &Point) -> f32{
    let l = (p1.x - p2.x).abs();
    let h = (p1.y - p2.y).abs();
    return (l*l + h*h).sqrt();
}

#[cfg(test)]
mod utils_test {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = (332, 434);
        let p2 = (143, 302);
        assert_eq!(distance(p1, p2), 230);
    }
}
