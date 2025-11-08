use prism3_function::{Predicate, RcPredicate};

// 测试新的 2 参数版本宏是否能正确生成转换方法
fn main() {
    let pred = RcPredicate::new(|x: &i32| *x > 0);
    
    // 测试生成的转换方法
    let _box_pred = pred.into_box();
    let _rc_pred = pred.into_rc();
    let _fn_pred = pred.into_fn();
    
    // 由于 pred 被 move 了，我们重新创建一个来测试 to_* 方法
    let pred2 = RcPredicate::new(|x: &i32| *x > 0);
    let _box_pred2 = pred2.to_box();
    let _rc_pred2 = pred2.to_rc();
    let _fn_pred2 = pred2.to_fn();
    
    println!("新宏功能测试通过！");
}
