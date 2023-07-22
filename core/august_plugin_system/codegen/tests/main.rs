#[cfg(test)]
mod main {
    use august_plugin_system::variable::Variable;

    extern crate august_plugin_system;

    mod functions {
        use august_plugin_system::variable::Variable;
        use codegen::function;

        #[function]
        fn add(_: (), (a, b): (Vec<i32>, String)) -> Vec<i32> {
            let mut c = [0; 1];
            c[0] = a[0] + b.parse::<i32>().unwrap();
            println!("{} + {} = {}", a[0], b, c[0]);
            c.to_vec()
        }

        #[function(name = "Sub function")]
        fn sub(_: (), (a, b): (i32, i32)) -> i32 {
            let c = a - b;
            println!("{} - {} = {}", a, b, c);
            c
        }

        #[function(description = "Checks the letter `A` in the array of strings")]
        fn contain_a(_: (), strs: Vec<String>) -> Variable {
            for s in strs {
                if !s.contains("a") {
                    return Variable::Null;
                }
            }
            true.into()
        }

        #[function(
            name = "Logging",
            description = "Outputs the transmitted string with a mark at the beginning"
        )]
        fn log(opt: &Option<String>, s: String) {
            println!("{s}: {opt:?}");
        }
    }

    #[test]
    fn serialize_add() {
        let add = functions::add();
        println!(
            "`add` name: {}\n`add` description: {}",
            add.name(),
            add.description()
        );

        let result = add.call(&[], &[vec![1].into(), "2".into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Variable::List(vec![3.into()])));
    }

    #[test]
    fn serialize_sub() {
        let sub = functions::sub();
        println!(
            "`sub` name: {}\n`sub` description: {}",
            sub.name(),
            sub.description()
        );

        let result = sub.call(&[], &[3.into(), 2.into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1.into()));
    }

    #[test]
    fn serialize_contain_a() {
        let contain_a = functions::contain_a();
        println!(
            "`contain_a` name: {}\n`contain_a` description: {}",
            contain_a.name(),
            contain_a.description()
        );

        let mut result = contain_a.call(&[], &[vec!["apple", "banana"].into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(true.into()));

        result = contain_a.call(&[], &[vec!["moon", "sun"].into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Variable::Null));
    }

    #[test]
    fn serialize_log() {
        let log = functions::log();
        println!(
            "`log` name: {}\n`log` description: {}",
            log.name(),
            log.description()
        );

        let mut result = log.call(&[Box::new(Some("Hello world".to_string()))], &["[INFO]".into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        result = log.call(&[Box::new(Option::<String>::None)], &["[WARN]".into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}
