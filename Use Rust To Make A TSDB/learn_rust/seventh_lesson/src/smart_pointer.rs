#[cfg(test)]
mod test {
    /// 通过原始指针获得数据
    #[test]
    fn raw_pointer_1() {
        // 创建 i32 类型变量 a，赋值为 1
        let a = 1_i32;
        println!("1: a = {}", a);

        // 创建引用类型 a_ref 指向 a
        let a_ref = &a;
        println!("2: a = {}", *a_ref);

        // 创建原始指针类型 a_ptr，指向 a
        let a_ptr = a_ref as *const i32;
        unsafe {
            // 获取原始指针指向的数据，需要在 unsafe 块中进行
            println!("3: a = {}", *a_ptr);
        }
    }

    /// 通过原始指针修改数据
    #[test]
    fn raw_pointer_2() {
        // 创建 i32 类型变量 a，赋值为 1
        let mut a = 1_i32;
        println!("1: a = {}", a);

        // 创建引用类型 a_ref 指向 a
        let a_mut_ref = &mut a;
        *a_mut_ref = 2_i32;
        println!("2: a = {}", *a_mut_ref);

        // 创建原始指针类型 a_ptr，指向 a
        let a_mut_ptr = a_mut_ref as *mut i32;
        unsafe {
            // 修改原始指针指向的数据，需要在 unsafe 块中进行
            *a_mut_ptr = 3_i32;
            println!("3: a = {}", *a_mut_ptr);
        }
        println!("4: a = {}", a);
    }

    // 通过原始指针修改不可变的数据
    #[test]
    fn raw_pointer_3() {
        // 创建 i32 类型变量 a，赋值为 1
        let a = 1_i32;
        println!("1: a = {}", a);

        // 创建 usize 类型，存储内存地址的数值
        let a_addr = &a as *const i32 as usize;
        println!("2: addr of a = 0x{:x}", a_addr);

        // 将内存地址转换为原始指针，从上方代码中可以看出其指向 a
        let a_mut_ptr = a_addr as *mut i32;
        println!("3: addr of a = {:p}", a_mut_ptr);
        unsafe {
            // 修改原始指针指向的数据，需要在 unsafe 块中进行
            *a_mut_ptr = 2_i32;
            println!("4: a = {}", *a_mut_ptr);
        }
        println!("5: a = {}", a);
    }

    /// 可变引用类型 &mut T
    #[test]
    fn reference_1() {
        let mut a = 1;
        let a_ref = &mut a;
        // 通过引用类型 &mut a，对 a 进行修改
        *a_ref = 2;
        println!("c = {}", a);
    }

    /// 复杂引用类型
    #[test]
    fn reference_2() {
        // Mutable 持有 Inner 的可变引用
        #[derive(Debug)]
        struct Inner(i32);
        struct Mutable<'a> {
            inner: &'a mut Inner,
        }
        impl<'a> Mutable<'a> {
            pub fn add(&mut self, v: i32) {
                let inner_mut_ref = &mut (*self.inner);
                let inner_v_mut_ref = &mut inner_mut_ref.0;
                *inner_v_mut_ref += v;
            }
        }

        let mut inner = Inner(1);
        let mut a = Mutable { inner: &mut inner };
        a.add(1);
        println!("inner = {:?}", a.inner);
        // 问题：多个 Mutable 类型同时持有 &mut Inner 会发生什么？
        // let mut b = Mutable{inner: &mut inner};
        a.add(1);
    }

    /// 复杂引用类型，使用 Box<T>
    #[test]
    fn box_1() {
        // Mutable 持有 Box<Inner>
        #[derive(Debug)]
        struct Inner(i32);
        struct Mutable {
            inner: Box<Inner>,
        }
        impl Mutable {
            pub fn add(&mut self, v: i32) {
                // 自动解引用
                self.inner.0 += v;
                // let a = &mut (self.inner.deref_mut());
                // a.0 += v;
            }
        }

        let inner = Box::new(Inner(1));
        let mut a = Mutable { inner };
        a.add(1);
        println!("inner = {:?}", a.inner);
        // 问题：多个 Mutable 同时持有 Box<Inner> 会发生什么？
        // let b = Mutable { inner };
        a.add(1);
    }

    /// Box<T>
    #[test]
    fn box_2() {
        let a = 1;
        let b = Box::new(a);

        // 消耗掉 b，取得 b 中数据的可变引用
        let c = Box::leak(b);
        println!("c = {}", c);
    }

    /// 使用 Box<T: ?Unsized> 存放 DST 类型
    #[test]
    fn box_3() {
        trait PiProvider {
            fn echo_pi(&self) -> String;
        }
        struct PiInteger {
            pi: i8,
        }
        impl PiProvider for PiInteger {
            fn echo_pi(&self) -> String {
                format!("PI = {}", self.pi)
            }
        }
        struct PiFloat {
            pi: f32,
        }
        impl PiProvider for PiFloat {
            fn echo_pi(&self) -> String {
                format!("PI = {}", self.pi)
            }
        }
        // dyn PiProvider 可能是 PiInteger 或 PiFloat，其大小无法确定
        let services: Vec<Box<dyn PiProvider>> = vec![
            Box::new(PiInteger { pi: 3 }),
            Box::new(PiFloat { pi: 3.14 }),
        ];
        for s in services {
            println!("{}", s.echo_pi());
        }
    }

    /// 使用 Rc<T> 共享 T
    #[test]
    fn rc_1() {
        use std::rc::Rc;
        struct Service {
            f: Rc<u8>,
        }

        let data = Rc::new(1_u8);
        let a = Service { f: data.clone() };
        let b = Service { f: data.clone() };

        println!("a.f = {}", a.f);
        println!("b.f = {}", b.f);

        // 问题：将 Rc<T> 传到新线程中，会发生什么？
        // use std::thread;
        // for _i in 0..3 {
        //     let c = data.clone();
        //     let _h = thread::spawn(move || {
        //         println!("f = {}", c);
        //     });
        // }
    }

    /// Arc<T>
    #[test]
    fn arc_1() {
        use std::sync::Arc;
        struct Service {
            f: Arc<u8>,
        }
        let data = Arc::new(1_u8);
        let a = Service { f: data.clone() };
        let b = Service { f: data.clone() };

        println!("a.f = {}", a.f);
        println!("b.f = {}", *b.f);

        use std::thread;
        let mut handlers = Vec::new();
        for _i in 0..3 {
            // 创建一个对 data 的引用
            let c = data.clone();
            let handle = thread::spawn(move || {
                // 在新线程中打印该引用
                println!("f = {}", c);
            });
            handlers.push(handle);
        }
        // 等待所有线程打印完成
        for handle in handlers {
            handle.join().unwrap();
        }

        // 问题：如何通过 Arc<T> 对 T 进行修改？
        // let data_val = *data;
        // data_val += 1;
    }

    // 通过 Cell 实现共享引用的内部可变性
    #[test]
    fn cell_1() {
        use std::cell::Cell;

        let data = Cell::new(1_i32);
        // 创建多个 data 的共享引用
        let a = &data;
        let b = &data;

        println!("inner = {}", data.get());
        a.set(2);
        println!("inner = {}", data.get());
        b.set(3);
        println!("inner = {}", data.get());
        a.set(4);
        println!("inner = {}", data.get());
    }

    // 在闭包中使用 Cell
    #[test]
    fn cell_2() {
        use std::cell::Cell;
        let c = Cell::new(1_u8);
        let func_add_1 = {
            let c_ref = &c;
            move || {
                c_ref.set(c_ref.get() + 1);
            }
        };
        let func_add_2 = {
            let c_ref = &c;
            move || {
                c_ref.set(c_ref.get() + 2);
            }
        };
        println!("c = {}", c.get());
        func_add_1();
        println!("c = {}", c.get());
        func_add_2();
        println!("c = {}", c.get());
    }

    // 通过 RefCell 实现内部可变性
    #[test]
    fn ref_cell_1() {
        use std::cell::RefCell;
        struct Immutable {
            inner: RefCell<i32>,
        }
        impl Immutable {
            fn add(&self) {
                let mut f_mut = self.inner.borrow_mut();
                *f_mut += 1;
            }
        }

        let a = Immutable {
            inner: RefCell::new(0),
        };
        *a.inner.borrow_mut() += 1;
        println!("inner = {}", a.inner.borrow());

        let f = a.inner.borrow_mut();
        println!("inner = {}", f);
        drop(f);

        println!("inner = {}", a.inner.borrow());
        a.add();
        a.add();
        println!("inner = {}", a.inner.borrow());
    }

    #[test]
    fn rc_refcell() {
        use std::cell::RefCell;
        use std::rc::Rc;
        struct Service {
            f: Rc<RefCell<u8>>,
        }
        impl Service {
            fn add(&self) {
                let mut f = self.f.borrow_mut();
                *f += 1;
            }
        }

        let data = Rc::new(RefCell::new(1_u8));
        let a = Service { f: data.clone() };
        let b = Service { f: data.clone() };
        println!("f = {}", (*data).borrow());
        a.add();
        b.add();
        println!("f = {}", (*data).borrow());
    }

    // #[test]
    // fn test_drop() -> i32 {
    //     struct A {
    //
    //     }
    //     impl Drop for A {
    //
    //     }
    // }
}
