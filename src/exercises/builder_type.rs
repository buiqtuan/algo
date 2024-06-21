
mod owned_trait_object {
    struct Dog {
        name: String,
        age: i8,
    }
    struct Cat {
        lives: i8,
    }
    trait Pet {
        fn talk(&self) -> String;
    }
    impl Pet for Dog {
        fn talk(&self) -> String {
            format!("Woof, my name is {}!", self.name)
        }
    }
    impl Pet for Cat {
        fn talk(&self) -> String {
            String::from("Miau!")
        }
    }

        #[test]
    fn test_owned_trait() {
        let pets: Vec<Box<dyn Pet>> = vec![
            Box::new(Cat { lives: 9 }),
            Box::new(Dog { name: String::from("Fido"), age: 5 }),
        ];
        for pet in pets {
            println!("Hello, who are you? {}", pet.talk());
        }
    }
}

mod weak_pointer {
    use std::{ops::DerefMut, rc::Rc};

    #[test]
    fn weak_pointer() {
        let a = Rc::new("Hello".to_string());
        let b = Rc::clone(&a);
        let c = Rc::downgrade(&a);

        println!(
            "Strong: {}, Weak: {}", 
            Rc::strong_count(&a), 
            Rc::weak_count(&a)
        );

        println!("a: {}, b: {}, c: {}", a, b, c.upgrade().unwrap());
    }
}

mod boxer {
    use std::mem::size_of_val;

    #[derive(Debug)]
    enum List<T> {
        Element(T, Box<List<T>>),
        Nil
    }

    #[test]
    fn test_boxer() {
        let list: List<i32> = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));

        println!("{:?}", list);
    }

    #[test]
    fn test_box_size() {
        let just_box = Box::new(5);
        let optional_box = Some(Box::new(5));
        let none = None::<Box<i32>>;

        println!("Size of Box<i32>: {}", size_of_val(&just_box));
        println!("Size of Option<Box<i32>>: {}", size_of_val(&optional_box));
        println!("Size of None::<Box<i32>>: {}", size_of_val(&none));
    }
}

mod builder_type {
    enum Language {
        Rust,
        Java,
        Perl
    }

    struct Dependency {
        name: String,
        version_expression: String
    }

    struct Package {
        name: String, 
        version: String,
        authors: Vec<String>,
        dependencies: Vec<Dependency>,
        language: Option<Language>
    }

    impl Package {
        fn as_dependency(&self) -> Dependency {
            Dependency {
                name: self.name.clone(),
                version_expression: format!("={}", self.version)
            }
        }
    }

    struct PackageBuilder(Package);

    impl PackageBuilder {
        fn new(name: impl Into<String>) -> Self {
            PackageBuilder(
                Package {
                    name: name.into(),
                    version: "0.1.0".to_string(),
                    authors: vec![],
                    dependencies: vec![],
                    language: None
                }
            )
        }

        fn version(mut self, version: impl Into<String>) -> Self {
            self.0.version = version.into();
            self
        }

        fn author(mut self, author: impl Into<String>) -> Self {
            self.0.authors.push(author.into());
            self
        }

        fn dependency(mut self, dependency: Dependency) -> Self {
            self.0.dependencies.push(dependency);
            self
        }

        fn language(mut self, language: Language) -> Self {
            self.0.language = Some(language);
            self
        }

        fn build(self) -> Package {
            self.0
        }
    }

    #[test]
    fn test_build_full_package() {
        let _package = PackageBuilder::new("my_crate")
            .version("0.2.0")
            .author("Alice")
            .author("Bob")
            .dependency(Dependency {
                name: "other_crate".to_string(),
                version_expression: "^1.0.0".to_string()
            })
            .language(Language::Rust)
            .build();
    }
}