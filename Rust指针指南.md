*	[1 概述](#1 概述)
*	[2 引用](#2 Hello, world!)
*	[3 Boxes](#)
*	[4 Rc和Arc](#变量绑定)
*	[5 原始指针](#If)
*	[6 返回一个指针](#函数)
*	[7 Creating your own Pointers](#注释)
*	[8 Patterns and ref](#复合数据类型)
*	[9 Cheat Sheet](#)
*	[10 Related resources](#循环)

Rust的指针有其独特和引人注目的特性,也是容易令Rust新人感到迷惑的主题之一.更有甚者,即使你曾经接触过其它支持指针的程序设计语言,如C++.它也会让你产生一定的迷惑.
本指南的目的就是帮助你理解Rust中的这个重要的主题.

对指针的使用必须经过深思熟虑,而不是为了让编译器感到爽.每一种指针类型都有其特定的使用场合.

你可能会对[备忘录]()感兴趣,它提供了各种指针类型的说明和使用目的.

###<span id="1 概述">1 概述</span>

如果你对指针的概念不太熟悉,那么本章非常适合你阅读.指针是系统编程语言中一个非常基础的概念,所以你有必要将其搞懂.

####1.1 指针基础

当你创建一个新的变量绑定,你相当于给存储在栈上某个位置的值取了一个名字,可以通过这个名字来引用那个值.(如果你搞不清"栈"和"堆的"区别,请看[this Stack Overflow question](),因为后边的内容我们假设你理解它们的区别.):

    let x = 5i;
    let y = 8i;
    
    
    location	value
    0xd3e030	5
    0xd3e028	8
    
在这里我们把变量的内存地址展示出来,内存中存放的只是简单的数值.我们的变量`x`对应于内存地址`0xd3e030`,这个地址的内存中存放的内容是`5`.因此当我们引用`x`的时候,我们得到的是其对应的数值`5`.

现在让我们来介绍指针.在一些程序设计语言中,只有一种类型的指针,但在Rust中,我们有许多种类型.在此我们先介绍Rust中的引用,它是Rust中最简单的指针类型.

    let x = 5i;
    let y = 8i;
    let z = &y;
    
    location	value
    0xd3e030	5
    0xd3e028	8
    0xd3e020	0xd3e028

看到差别了吗?指针中存放的是`y`的内存地址.`x`和`y`的类型是`int`,而`z`的类型是`&int`.我们可以用格式字符串`{:p}`来打印这个地址:

    let x = 5i;
    let y = 8i;
    let z = &y;
    
    println!("{:p}", z);
    
这段代码会打印出`0xd3e028`.

因为`int`和`&int`是不同的类型,所以我们不能将二者相加,例如:

    let x = 5i;
    let y = 8i;
    let z = &y;
    
    println!("{}", x + z);   
    
这会导致编译错误:

    hello.rs:6:24: 6:25 error: mismatched types: expected `int` but found `&int` (expected int but found &-ptr)
    hello.rs:6     println!("{}", x + z);
                                      ^     
                                      
我们可以用`*`操作符对一个指针解引用.解引用意味着访问指针指向的内存中存放的值,所以以下代码可以正常工作:
    
    let x = 5i;
    let y = 8i;
    let z = &y;
    
    println!("{}", x + *z);
    
输出13.

好了,这就是指针的全部内容:指向内存位置的一个变量.我们已经讨论了什么是指针,现在我们来看下为什么要使用指针.

####1.2 使用指针

指针在Rust中非常有用,但与其它一些系统程序语言又有些不同.后面我们会介绍Rust指针的实践,现在我们先来看看在其它语言中是怎么使用指针的:

在C中,字符串实际上是一个指向一个以null结尾的`char`数组的指针.要使用字符串你首先必须熟悉指针.

当指向非栈上内存的时候,指针将非常有用.例如,在我们上面的例子中使用了两个栈上变量,因此我们可以给变量绑定名字.但如果我们是在堆上分配的内存,我们就无法给它命名了.在C中,`malloc`用于在堆上分配内存,并返回指向那块内存的指针.

对于上面两点的一个更通用的变体,你需要用指针去指向一个可以改变大小的结构体.你无法在编译期指定你要分配多大的内存,而只能用指针指向那块在运行时分配出来的内存.

相对于通过引用传递的语言,指针在通过值传递的语言中显得额外的有用.

    fn foo(x) {
        x = 5
    }
    
    fn main() {
        i = 1
        foo(i)
        // what is the value of i here?
    }

在值传递的语言中,`foo`的到的将是`i`的拷贝,所以原始的`i`并没有被改变依旧保持了值1.而对于引用传递的语言,`foo`获得的是`i`的引用,因此`i`的值被改变成了`5`.

那么指针的用处又体现在哪呢.

    fn foo(&int x) {
        *x = 5
    }
    
    fn main() {
        i = 1
        foo(&i)
        // what is the value of i here?
    }
    
因为指针变量中存放的是内存地址,所以上述代码即使在值传递的语言中,`i`也会被改变为`5`.这中模式被成为通过值传递引用.        

####1.3 指针的一般问题

上面我们说了这么多指针的优点,现在是时候打击它一下了.现在我们先说一下指针在其它语言中存在的问题:

使用未初始化的指针:

    &int x;
    *x = 5; // whoops!
    
我们声明了一个指针但没有将它指向任何地方,然后将它指向的内存位置的值设置为`5`.那么它指向的内存地址到底是哪里?没有人知道.如果你足够幸运它可能是无害的,但更大的可能是对数据造成破坏.

当我们将指针和函数结合使用的时候,经常会意外的访问指向无效地址的指针,例如:

    fn make_pointer(): &int {
        x = 5;
    
        return &x;
    }
    
    fn main() {
        &int i = make_pointer();
        *i = 5; // uh oh!
    }

`x`是`make_pointer`中的局部变量,当`make_pointer`返回后,它的内存位置将会变得无效.而在函数`make_pointer`中我们却将`x`的地址返回给调用者并在`main`中访问这个地址.这个情况与上例是一样的.

最后我们将介绍别名导致的问题.当两个指针指向同一个内存地址时我们将它们称之为别名.例如:

    fn mutate(&int i, int j) {
        *i = j;
    }
    
    fn main() {
      x = 5;
      y = &x;
      z = &x; //y and z are aliased
    
    
      run_in_new_thread(mutate, y, 1);
      run_in_new_thread(mutate, z, 100);
    
      // what is the value of x here?
    }

在这个例子中,`run_in_new_thread`启动一个新的线程执行它参数中的函数.因为我们有两个线程且都在操作`x`的别名,我们无法指定那个线程先操作完成,所以我们无法得知x的最终值到底是什么.更坏的情况是,如果其中一个线程将指针指向内内存破坏了将会反生什么?

####1.4 结论

以上内容是指针的基本概念的一个概览.如我们在早前提及的,Rust中不只有一中类型的指针,它的其它指针类型将会减轻我们在上面提到的问题.这意味着Rust的指针将会比其它语言更加复杂一点,但随着后续的讲解,你会发现为这些额外的复杂性付出是值得的.



###<span id="2 引用">2 引用</span>

在Rust中最基本的指针类型被称为引用.Rust引用看上去是这个样子:

    let x = 5i;
    let y = &x;
    
    println!("{}", *y);
    println!("{:p}", y);
    println!("{}", y);

我们说`y`引用`x`.第一个`println!`通过对`y`使用解引用操作符`*`打印`y`所引用的值.第二个`println!`通过指针格式串打印出引用所指向的内存地址.而第三个`println!`打印的也是`y`所引用的值,因为`println!`会自动帮我们执行解引用.

下面是一个接受一个引用作为参数的函数:

    fn succ(x: &int) -> int { *x + 1 }
    
你还可以将`&`作为操作符创建一个引用,因此我们可以以两种不同的方式调用这个函数:

    fn succ(x: &int) -> int { *x + 1 }
    
    fn main() {
    
        let x = 5i;
        let y = &x;
    
        println!("{}", succ(y));
        println!("{}", succ(&x));
    }
    
两个`println!`函数都会打印`6`.

但然在真实世界的代码中这个函数不应该使用引用传递参数,而应该是下面这个样子:

    fn succ(x: int) -> int { x + 1 }        
    
默认情况下,引用是不可变的:

    let x = 5i;
    let y = &x;
    
    *y = 5; // error: cannot assign to immutable dereference of `&`-pointer `*y`    

我们可以使用`mut`关键字来创建可被改变的引用,前提是被引用的变量本身也是可被改变的.这段代码可以正常工作:

    let mut x = 5i;
    let y = &mut x;    
    
而这段则不行:

    let x = 5i;
    let y = &mut x; // error: cannot borrow immutable local variable `x` as mutable

不可变对象的指针可以存在别名:

    let x = 5i;
    let y = &x;
    let z = &x;

而可变对象则不行:

    let mut x = 5i;
    let y = &mut x;
    let z = &mut x; // error: cannot borrow `x` as mutable more than once at a time   

 尽管它很安全,引用在运行时的实现其实如C程序中的普通指针一样.完全没有增加任何运行时负担,所有的安全性检测都是在编译期完成的. 为此提供安全性检测的理论最初被称为"region pointers".而它逐渐演化成了我们今天称之为生存期的概念.
 
下面是一个简单的解释,你认为以下代码通过编译吗?

    fn main() {
        println!("{}", x);
        let x = 5;
    } 
 
 显然不能.因为名字`x`只在其被声明到它离开作用域的这段时间内是有效的.所以你知道这段代码会产生错误.我们将变量的有效期称为生存期.让我们看一个复杂一点的例子:
 
     fn main() {
        let x = &mut 5i;
    
        if *x < 10 {
            let y = &x;
    
            println!("Oh no: {}", y);
            return;
        }
    
        *x -= 1;
    
        println!("Oh no: {}", x);
    }    
    
我们在`if`语句块中借用了`x`.编译器会检测到这个借用在我们改变`x`的时候已经离开了作用域,所以让我们通过编译.但下面的代码则无法通过:

    fn main() {
        let x = &mut 5i;
    
        if *x < 10 {
            let y = &x;
            *x -= 1;
    
            println!("Oh no: {}", y);
            return;
        }
    
        *x -= 1;
    
        println!("Oh no: {}", x);
    }     
    
编译器发出以下抱怨:

    test.rs:5:8: 5:10 error: cannot assign to `*x` because it is borrowed
    test.rs:5         *x -= 1;
                      ^~
    test.rs:4:16: 4:18 note: borrow of `*x` occurs here
    test.rs:4         let y = &x;
                              ^~     
                              
如你能猜到的,这种检测对人来说都是相当复杂的更不用说对机器了.如果你想对生存期的细节有更多的了解请参考[guide devoted to references and lifetimes](http://doc.rust-lang.org/0.12.0/guide-lifetimes.html).

####2.1 最佳实践

优先考虑在栈而不是堆上分配对象.只要可能我们都应该使用对栈上对象的引用.这样默认使用的指针都是引用类型,除非你有特殊的理由要使用其它类型的指针.

当你需要的只是指针而不需要获得所有权时请使用指针.引用只是借用所有权,当你不需要获得所有权的情况下它显然是更好的选择.也就是说我们应该这样写:

    fn succ(x: &int) -> int { *x + 1 }

而不是

    fn succ(x: Box<int>) -> int { *x + 1 }
    
作为推论,引用允许接受更多的指针类型,这使得你不需要为接受不同指针类型实现不同的函数版本.也就是我们应该这样写:

    fn succ(x: &int) -> int { *x + 1 }    
    
而不是:

    fn box_succ(x: Box<int>) -> int { *x + 1 }
    
    fn rc_succ(x: std::rc::Rc<int>) -> int { *x + 1 }