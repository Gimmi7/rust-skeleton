pub mod amod {
    use std::sync::OnceLock;

    use super::bmod;


    static ANAME: OnceLock<&str> = OnceLock::new();

    pub fn get_name() -> &'static str {
        println!("modA: run get_name");

        ANAME.get_or_try_init(|| -> Result<&str, ()> {
            println!("modA: run get_or_try_init");
            println!("modA: read b name in amod:{}", bmod::get_name());
            Ok("i am A name")
        }).unwrap()
    }
}

pub mod bmod {
    use std::sync::OnceLock;
    use super::amod;

    static BNAME: OnceLock<&str> = OnceLock::new();

    pub fn get_name() -> &'static str {
        println!("modB: run get_name");

        BNAME.get_or_try_init(|| -> Result<&str, ()> {
            println!("modB: run get_or_try_init");
            println!("modB: read a name in bmod:{}", amod::get_name());
            Ok("i am B name")
        }).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::amod;

    #[test]
    fn test_circulate_deps() {
        println!("{}", amod::get_name());
    }
}