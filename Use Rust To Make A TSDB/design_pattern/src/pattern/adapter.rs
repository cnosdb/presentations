
// 圆柱体
trait RoundPeg {
    fn radius(&self) -> f64;
}

// 孔洞
trait RoundHole {
    fn radius(&self) -> f64;
    fn fit(&self, round_peg: &dyn RoundPeg) -> bool {
        self.radius() > round_peg.radius()
    }
}

trait RoundHoleFit {
    fn fit(&self, round_peg: &dyn RoundPeg) -> bool;
}

// 正方体
trait SquarePeg {
    fn width(&self) -> f64;
}

// 圆柱体适配器?
impl RoundPeg for dyn SquarePeg {
    fn radius(&self) -> f64 {
        self.width() * 2.0_f64.sqrt() / 2.0
    }
}

impl RoundPeg for &dyn SquarePeg {
    fn radius(&self) -> f64 {
        self.width() * 2.0_f64.sqrt() / 2.0
    }
}








struct RoundPegImpl {
    radius: f64
}

impl RoundPegImpl {
    pub fn new(radius: f64) -> Self {
        Self {
            radius
        }
    }
}

impl RoundPeg for RoundPegImpl {
    fn radius(&self) -> f64 {
        self.radius
    }
}

struct RoundHoleImpl {
    radius: f64
}

impl RoundHoleImpl {
    pub fn new(radius: f64) -> Self {
        Self {
            radius
        }
    }
}

impl RoundHole for RoundHoleImpl {
    fn radius(&self) -> f64 {
        self.radius
    }
}

struct SquareImpl {
    width: f64
}

impl SquareImpl {
    pub fn new(width: f64) -> Self {
        Self {
            width
        }
    }
}

impl SquarePeg for SquareImpl {
    fn width(&self) -> f64 {
        self.width
    }
}

#[test]
fn test() {
    let round_peg = RoundPegImpl::new(1.0);
    let round_hole = RoundHoleImpl::new(2.0);
    assert!( round_hole.fit(&round_peg));


    let square_peg = SquareImpl::new(1.0);
    assert!( round_hole.fit(&(&square_peg as &dyn SquarePeg)))
}



