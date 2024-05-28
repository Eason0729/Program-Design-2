
pub struct MyFloat{
    denominator: i32,
    numerator: i32,
}

impl MyFloat{
    pub fn new(denominator: i32, numerator: i32) -> Self{
        Self{
            denominator,
            numerator,
        }
    }
}

impl PartialEq for MyFloat{
    fn eq(&self, other: &Self) -> bool{
        self.denominator * other.numerator == self.numerator * other.denominator
    }
}

impl Eq for MyFloat{}

impl PartialOrd for MyFloat{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>{
        Some((self.numerator * other.denominator).cmp(&(self.denominator * other.numerator)))
    }
}

impl std::cmp::Ord for MyFloat{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering{
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}

impl std::fmt::Display for MyFloat{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl std::fmt::Debug for MyFloat{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl std::ops::Div for MyFloat{
    type Output = Self;

    fn div(self, other: Self) -> Self{
        let denominator = self.denominator * other.numerator;
        let numerator = self.numerator * other.denominator;
        Self{
            denominator,
            numerator,
        }
    }
}

impl std::ops::Add for MyFloat{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        let denominator = self.denominator * other.denominator;
        let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        Self{
            denominator,
            numerator,
        }
    }
}

impl std::ops::Sub for MyFloat{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        let denominator = self.denominator * other.denominator;
        let numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        Self{
            denominator,
            numerator,
        }
    }
}

impl std::ops::Mul for MyFloat{
    type Output = Self;

    fn mul(self, other: Self) -> Self{
        let denominator = self.denominator * other.denominator;
        let numerator = self.numerator * other.numerator;
        Self{
            denominator,
            numerator,
        }
    }
}