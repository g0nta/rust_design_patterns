// Sizedトレイトはコンパイル時にわかる固定サイズを持つ。
// 特に指定しないかぎりはSizedトレイトは暗黙的に実装されている
// なんでここでわざわざ書いてるの？？
//
// Copyトレイトはコピーができる
// struct Foo;
// let x = Foo;
// let y = x;
// 上の例はFooがCopyトレイトを実装していない。このままxを使おうとするとmoveされてるのでerrorになる
// #[derive(Copy, Clone)]
// で簡単に実装できる。
struct Container<T: Sized + Copy> {
    buf: Vec<T>,
    index: usize,
}

impl<T: Copy> Container<T> {
    fn new() -> Container<T>{
        Container {
            buf: Vec::new(),
            index: 0,
        }
    }

    fn add(&mut self, t: T){
        self.buf.push(t);
    }
}

impl<T: Copy> Iterator for Container<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        match self.index < self.buf.len() {
            true => {
                let t = Some(self.buf[self.index]);
                self.index += 1;
                t
            }
            false => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Container;

    #[test]
    fn iterate_test(){
        let mut c: Container<usize> = Container::<usize>::new();
        c.add(1);
        c.add(2);
        assert_eq!(c.next().unwrap(), 1);  //1
        assert_eq!(c.next().unwrap(), 2);  //2
    }

    #[test]
    #[should_panic]
    fn boundary_value_test(){
        let mut c: Container<usize> = Container::<usize>::new();
        c.add(1);
        c.add(2);
        c.next().unwrap();  //1
        c.next().unwrap();  //2
        c.next().unwrap();  // panic!
    }
}
