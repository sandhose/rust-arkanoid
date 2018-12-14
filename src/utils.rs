pub type Point = (u32, u32);
pub type Pixels = u32;

pub fn distance(p1_u32: Point, p2_u32: Point) -> u32{
    let p1 = (p1_u32.0 as f64, p1_u32.1 as f64);
    let p2 = (p2_u32.0 as f64, p2_u32.1 as f64);
    let l = (p1.0 - p2.0).abs();
    let h = (p1.1 - p2.1).abs();
    return (l*l + h*h).sqrt() as u32;
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
