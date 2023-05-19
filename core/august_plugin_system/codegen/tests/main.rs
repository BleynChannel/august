#[cfg(test)]
mod main {
    use august_plugin_system::variable::VariableData;
    use codegen::Function;

    extern crate august_plugin_system;

    #[derive(Function)]
    struct Add {
        a: Vec<i32>,
        b: String,
        #[output]
        c: Vec<i32>,
    }

    impl Add {
        fn run(a: Vec<i32>, b: String) -> Vec<i32> {
            let mut c = [0; 1];
            c[0] = a[0] + b.parse::<i32>().unwrap();
            println!("{} + {} = {}", a[0], b, c[0]);
            c.to_vec()
        }
    }

    #[derive(Function)]
    struct Sub(i32, i32, #[output] i32);

    impl Sub {
        fn run(a: i32, b: i32) -> i32 {
            let c = a - b;
            println!("{} - {} = {}", a, b, c);
            c
        }
    }

    #[derive(Function)]
    struct ContainA {
        strs: Vec<String>,
        #[output]
        result: VariableData,
    }

    impl ContainA {
        fn run(strs: Vec<String>) -> VariableData {
            for s in strs {
                if !s.contains("a") {
                    return VariableData::Null;
                }
            }
            true.into()
        }
    }

    #[derive(Function)]
    struct Print(String);

    impl Print {
        fn run(s: String) {
            println!("{}", s);
        }
    }

    #[test]
    fn function_serialize() {
        // Add
        let add = Add::as_function();
        let mut result = add.run(vec![[1].as_slice().into(), "2".into()].as_slice());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(VariableData::List(vec![3.into()])));

        // Sub
        let sub = Sub::as_function();
        result = sub.run(vec![3.into(), 2.into()].as_slice());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1.into()));

        // ContainA
        let contain_a = ContainA::as_function();

        result = contain_a.run(vec![vec!["apple", "banana"].into()].as_slice());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(true.into()));

        result = contain_a.run(vec![vec!["moon", "sun"].into()].as_slice());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(VariableData::Null));

        // Print
        let print = Print::as_function();
        result = print.run(vec!["hello".into()].as_slice());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}
