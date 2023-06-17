use num::Float;

// TODO: Extend to more demensions
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Point<T: Float> {
    x: T,
    y: T,
}

impl<T: Float> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> PartialOrd for Point<T>
where
    T: PartialOrd + Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x == other.x {
            self.y.partial_cmp(&other.y)
        } else {
            self.x.partial_cmp(&other.x)
        }
    }
}

#[derive(Debug)]
pub struct Rectangle<T: Float> {
    // lower left corner
    ll: Point<T>,
    // upper right corner
    ur: Point<T>,
}

impl<T> Rectangle<T>
where
    T: Clone + Float,
{
    pub fn new(ll: Point<T>, ur: Point<T>) -> Self {
        Self { ll, ur }
    }

    pub fn contains(&self, p: &Point<T>) -> bool {
        self.ll.x <= p.x && p.x <= self.ur.x && self.ll.y <= p.y && p.y <= self.ur.y
    }

    /// Returns true if this rectangle intersects the other rectangle
    pub fn intersects(&self, other: &Rectangle<T>) -> bool {
        self.ll.x <= other.ur.x
            && other.ll.x <= self.ur.x
            && self.ll.y <= other.ur.y
            && other.ll.y <= self.ur.y
    }

    /// Returns true if this rectangle contains the other rectangle
    pub fn contains_rect(&self, other: &Rectangle<T>) -> bool {
        self.ll.x <= other.ll.x
            && other.ur.x <= self.ur.x
            && self.ll.y <= other.ll.y
            && other.ur.y <= self.ur.y
    }

    /// Return the minimum bounding rectangle that contains both rectangles
    pub fn mbr(&self, other: &Rectangle<T>) -> Rectangle<T> {
        // lower left corner
        let ll = if self.ll < other.ll {
            self.ll.clone()
        } else {
            other.ll.clone()
        };
        let ur = if self.ur > other.ur {
            self.ur.clone()
        } else {
            other.ur.clone()
        };
        Rectangle::new(ll, ur)
    }
}

#[derive(Debug)]
pub enum Node<T: Float> {
    // leaf node points to data
    Leaf {
        tid: usize,
    },
    // inner node points to the next level of nodes
    Inner {
        rect: Rectangle<T>,
        children: Vec<Node<T>>,
    },
}

#[derive(Debug)]
pub struct RTree<T: Float> {
    root: Node<T>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, 2.0);
        assert!(p1 < p2);
        assert!(p2 > p1);
        assert!(p2 != p1);
    }

    #[test]
    fn test_rectangle() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 3.0);
        let r = Rectangle::new(p1.clone(), p2.clone());

        let p3 = Point::new(1.0, 1.0);
        let p4 = Point::new(4.0, 4.0);
        let r2 = Rectangle::new(p3, p4.clone());

        assert!(r.contains(&p1));
        assert!(r.contains(&p2));
        assert!(r.intersects(&r2));

        let p5 = Point::new(0.0, 0.0);
        let p6 = Point::new(5.0, 5.0);
        let r3 = Rectangle::new(p5, p6);
        assert!(r3.contains_rect(&r));

        let mbr = r.mbr(&r2);
        assert_eq!(mbr.ll, p1);
        assert_eq!(mbr.ur, p4);
    }
}
