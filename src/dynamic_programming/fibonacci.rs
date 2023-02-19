

#[derive(Debug)]
pub struct Fibonacci {
    a:u128,
    b:u128,
}

impl Iterator for Fibonacci {
    // 指定迭代器元素的类型
    type Item = u128;

    // 实现next方法
    fn next(&mut self) -> Option<Self::Item> {
        // 保存当前的斐波那契数
        let current = self.a;
        // 更新a和b的值
        self.a = self.b;
        self.b = current + self.b;
        // 返回当前的斐波那契数
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { a: 0, b: 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fibonacci(){
        let mut fib = fibonacci();
        for _i in 0..100 {
            println!("{:?}",fib.next());
        }
    }
}