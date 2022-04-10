use std::fmt::{Display, Formatter, Result};
use std::num;

pub struct Car {
    gear: i8,
    speed: i32,
    occupents: u8,
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Car gear: {}, speed: {}, occupents: {}",
            self.gear, self.speed, self.occupents
        )
    }
}

impl Car {
    pub fn new() -> Self {
        Self {
            gear: 0,
            speed: 0,
            occupents: 0,
        }
    }

    pub fn add_occupent(&mut self) {
        self.occupents += 1;
    }

    pub fn upshift(&mut self) {
        if self.occupents > 0 {
            if self.speed < 0 {
                println!("Can't upshift if you're going backwards");
            } else if self.gear == 5 {
                println!("Can't upshift when you're already in 5th gear.")
            } else {
                self.gear += 1;
                println!("Successfully upshifted into gear: {}", self.gear);
            }
        } else {
            println!("A car that's empty can't be upshifted.");
        }
    }

    pub fn downshift(&mut self) {
        if self.occupents > 0 {
            if self.speed > 0 && self.gear > -1 {
                println!("Can't downshift if you're going not stopped");
            } else if self.gear == -1 {
                println!("Can't downshift when you're already in reverse.")
            } else {
                self.gear -= 1;
                println!("Successfully downshifted into gear: {}", self.gear);
            }
        } else {
            println!("A car that's empty can't be downshifted.");
        }
    }

    pub fn accelarate(&mut self) {
        match self.gear {
            gear if gear > 0 => {
                if self.speed < self.gear as i32 * 10 {
                    self.speed += 10;
                    println!("You just sped up")
                } else {
                    println!("Gear's already reved out!");
                }
            }
            gear if gear == 0 => {
                println!("You need to put it in gear before you can speed up.")
            }
            gear if gear < 0 => {
                if self.speed != 10 {
                    self.speed += 10;
                    println!("Forward acceleration when going backwards is slowing down!")
                } else {
                    println!("You need to put it in a forwards gear before you can accelerate since you're stopped!");
                }
            }
            _ => println!("What'd you do?!?! Weird acceleration."),
        }
    }

    pub fn decelarate(&mut self) {
        match self.gear {
            gear if gear > 0 => {
                if self.speed > 0 {
                    self.speed -= 10;
                    println!("You just slowed down!")
                } else {
                    println!("You're already stopped, downshift to go in reverse");
                }
            }
            gear if gear == 0 => {
                println!("You need to put it in gear before you can go backeards or something.")
            }
            gear if gear < 0 => {
                if self.speed == 0 {
                    self.speed -= 10;
                    println!("You're now going backwards backwards!")
                } else {
                    println!("Reverse is already reved out!");
                }
            }
            _ => println!("What'd you do?!?! Weird deceleration."),
        }
    }

    // pub fn decelerate(&mut self) {
    //     if spee
    // }
}
