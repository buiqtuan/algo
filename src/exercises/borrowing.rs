mod borrowing {
    use std::cell::{Ref, RefCell, Cell};

    
    #[test]
    fn test_borrow() {
        let mut a = 10;
        let b = &a;

        println!("b: {}", b);
        {
            let c = &mut a;
            *c = 20;
        }

        println!("a: {}", a);

        let mut v = vec![1,2,3,4];

        for i in 0..v.len() {
            let e = v[i]*2;

            v.push(e);
        }

        println!("{:?}", v);
    }

    #[test]
    fn test_refcell() {
        let ref_cell = RefCell::new(5);

        {
            let mut cell_ref = ref_cell.borrow_mut();
            *cell_ref = 123;
        }

        print!("{:?}", ref_cell);

        let cell = Cell::new(5);

        cell.set(10);

        print!("{}", cell.get());
    }
}

mod health_statistic {
    pub struct User {
        name: String,
        age: u32,
        height: f32,
        visit_count: usize,
        last_blood_pressure: Option<(u32, u32)>
    }

    pub struct Measurements {
        height: f32,
        blood_pressure: (u32, u32)
    }

    pub struct HealthReport<'a> {
        patient_name: &'a str,
        visit_count: u32,
        height_change: f32,
        blood_pressure_change: Option<(u32, u32)>
    }

    impl User {
        fn new(name: String, age: u32, height: f32) -> Self {
            Self {
                name, age, height, visit_count: 0, last_blood_pressure: None
            }
        }

        fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
            let Measurements {
                height, 
                blood_pressure
            } = measurements;

            self.visit_count += 1;
            self.height = height;
            self.last_blood_pressure = Option::Some(blood_pressure);

            HealthReport {
                patient_name: &self.name,
                visit_count: self.visit_count as u32,
                height_change: self.height - height,
                blood_pressure_change: match self.last_blood_pressure {
                    Some(lbp) => Option::Some((
                        lbp.0 - blood_pressure.0,
                        lbp.1 - blood_pressure.1
                    )),
                    None => None
                }
            }
        }
    }
}