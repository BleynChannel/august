#[cfg(test)]
mod main {
    use august_plugin_system::variable::Variable;
    use codegen::Function;

    extern crate august_plugin_system;

	//TODO: Создать новый вид генерации функции
    // mod Functions {
    // 	#[function(name = "It's name (optional)", description = "It's description (optional)")]
    // 	pub fn add(_: (), (a, b): (Vec<i32>, String)) -> Vec<i32> {
    // 		let mut c = [0; 1];
    //         c[0] = a[0] + b.parse::<i32>().unwrap();
    //         println!("{} + {} = {}", a[0], b, c[0]);
    //         c.to_vec()
    // 	}

    // 	#[test]
    // 	fn main() {
    // 		let function = add.as_function();
    // 		function.call(&[], &[vec![1].into(), "2".into()]);
    // 	}
    // }
	
    #[derive(Function)]
    struct Add {
        a: Vec<i32>,
        b: String,
        #[output]
        c: Vec<i32>,
    }

    impl Add {
        fn call(_: (), (a, b): (Vec<i32>, String)) -> Vec<i32> {
            let mut c = [0; 1];
            c[0] = a[0] + b.parse::<i32>().unwrap();
            println!("{} + {} = {}", a[0], b, c[0]);
            c.to_vec()
        }
    }

    #[derive(Function)]
    struct Sub(i32, i32, #[output] i32);

    impl Sub {
        fn call(_: (), (a, b): (i32, i32)) -> i32 {
            let c = a - b;
            println!("{} - {} = {}", a, b, c);
            c
        }
    }

    #[derive(Function)]
    struct ContainA {
        strs: Vec<String>,
        #[output]
        result: Variable,
    }

    impl ContainA {
        fn call(_: (), strs: Vec<String>) -> Variable {
            for s in strs {
                if !s.contains("a") {
                    return Variable::Null;
                }
            }
            true.into()
        }
    }

    #[derive(Function)]
    struct Print(String, #[external] Option<String>);

    impl Print {
        fn call(opt: &Option<String>, s: String) {
            println!("{s}: {opt:?}");
        }
    }

    #[test]
    fn serialize_add() {
        let add = Add::as_function();
        let result = add.call(&[], &[vec![1].into(), "2".into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Variable::List(vec![3.into()])));
    }

	#[test]
	fn serialize_sub() {
		let sub = Sub::as_function();
        let result = sub.call(&[], &[3.into(), 2.into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1.into()));
	}

	#[test]
	fn serialize_contain_a() {
		let contain_a = ContainA::as_function();

        let mut result = contain_a.call(&[], &[vec!["apple", "banana"].into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(true.into()));

        result = contain_a.call(&[], &[vec!["moon", "sun"].into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(Variable::Null));
	}

	#[test]
	fn serialize_print() {
		let print = Print::as_function();
        let mut result = print.call(&[Box::new(Some("Foo".to_string()))], &["hello".into()]);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

		result = print.call(&[Box::new(Option::<String>::None)], &["hello".into()]);

		assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
	}
}
