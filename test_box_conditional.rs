use prism3_function::consumers::*;

#[test]
fn test_box_conditional_consumer_once_and_then() {
    let result = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let r1 = result.clone();
    let r2 = result.clone();

    let consumer1 = BoxConsumerOnce::new(move |x: &i32| {
        r1.lock().unwrap().push(*x * 2);
    });

    let consumer2 = BoxConsumerOnce::new(move |x: &i32| {
        r2.lock().unwrap().push(*x + 100);
    });

    let conditional = consumer1.when(|x: &i32| *x > 0);
    let chained = conditional.and_then(consumer2);

    chained.accept(&5);

    let final_result = result.lock().unwrap();
    assert_eq!(*final_result, vec![10, 105]); // 5*2 = 10, then 5+100 = 105
}

#[test]
fn test_box_conditional_consumer_once_or_else() {
    let result = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let r1 = result.clone();
    let r2 = result.clone();

    let consumer1 = BoxConsumerOnce::new(move |x: &i32| {
        r1.lock().unwrap().push(*x);
    });

    let consumer2 = BoxConsumerOnce::new(move |x: &i32| {
        r2.lock().unwrap().push(-*x);
    });

    let conditional = consumer1.when(|x: &i32| *x > 0);
    let or_else_consumer = conditional.or_else(consumer2);

    or_else_consumer.accept(&5);
    assert_eq!(*result.lock().unwrap(), vec![5]); // Condition satisfied

    let result2 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let r3 = result2.clone();
    let r4 = result2.clone();

    let consumer3 = BoxConsumerOnce::new(move |x: &i32| {
        r3.lock().unwrap().push(*x);
    });

    let consumer4 = BoxConsumerOnce::new(move |x: &i32| {
        r4.lock().unwrap().push(-*x);
    });

    let conditional2 = consumer3.when(|x: &i32| *x > 0);
    let or_else_consumer2 = conditional2.or_else(consumer4);

    or_else_consumer2.accept(&-5);
    assert_eq!(*result2.lock().unwrap(), vec![5]); // Condition not satisfied, execute else
}
