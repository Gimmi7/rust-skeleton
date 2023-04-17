pub trait BigTarget: Show + Prime {}

pub trait Show {
    fn show(&self);
}

pub trait Prime {
    type BnType;

    fn next_prime(&self) -> Self::BnType;
}

pub struct MyBig {
    name: &'static str,
    value: i32,
}

pub struct CBig {
    name: &'static str,
    value: i32,
}


impl Show for MyBig {
    fn show(&self) {
        println!("{}:{}", self.name, self.value);
    }
}

impl Prime for MyBig {
    type BnType = MyBig;

    fn next_prime(&self) -> MyBig {
        
        MyBig {
            name: self.name,
            value: self.value + 1,
        }
    }
}

impl BigTarget for MyBig {}


impl Show for CBig {
    fn show(&self) {
        println!("{}:{}", self.name, self.value);
    }
}

impl Prime for CBig {
    type BnType = CBig;

    fn next_prime(&self) -> CBig {
        
        CBig {
            name: self.name,
            value: self.value + 1,
        }
    }
}

impl BigTarget for CBig {}

fn showbn(bn: &impl BigTarget) {
    bn.show();
}

fn nextbn<B: BigTarget>(bn: &B) -> impl BigTarget
    where <B as Prime>::BnType: BigTarget
{
    bn.next_prime()
}

fn main() {

}

#[cfg(test)]
mod test {
    use crate::{CBig, MyBig, nextbn, showbn};

    #[test]
    fn test_traits() {
        let mybn = MyBig {
            name: "mybig",
            value: 55,
        };
        showbn(&mybn);

        let cbn = CBig {
            name: "cbig",
            value: 43,
        };
        showbn(&cbn);


        let my_next = nextbn(&mybn);
        showbn(&my_next);
    }
}