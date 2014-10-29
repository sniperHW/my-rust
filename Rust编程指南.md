*	[1 安装Rust](#1 安装Rust)
*	[2 Hello, world!](#2 Hello, world!)
*	[3 Hello, Cargo!](#)
*	[4 变量绑定](#变量绑定)
*	[5 If](#If)
*	[6 函数](#函数)
*	[7 注释](#注释)
*	[8 复合数据类型](#复合数据类型)
*	[9 Match](#)
*	[10 循环](#循环)
*	[11 Strings](#11 Strings)
*	[12 Vectors](#12 Vectors)
*	[13 标准输入](#13 标准输入)
*	[14 猜迷游戏](#14 猜迷游戏)
*	[15 Crates and Modules](#)
*	[16 测试](#)
*	[17 指针](#)
*	[18 模式匹配](#模式匹配)
*	[19 Method Syntax](#)
*	[20 闭包](#)
*	[21 迭代器](#)
*	[22 范型](#)
*	[23 Traits](#)
*	[24 Tasks](#)
*	[25 宏](#)
*	[26 Unsafe](#)
*	[27 尾声](#尾声)




###<span id="安装Rust">安装Rust</span>

###<span id="12 Vectors">12 Vectors</span>

如大多数编程语言一样,Rust也提供了内置的列表类型,用于存放一组元素.如strings一样,Rust使用了3种不同的类型去具现化这种数据结构分别是:
`Vec<T>`(向量),`[T, .. N]`(数组)和`&[T]`(切片).

向量类型与Strings类似:可变的容量,你可以通过宏`vec!`创建一个象量:

	let nums = vec![1i, 2i, 3i];
    
你可能注意到了,在这里,我们没有像对宏`println!`一样使用圆括号,而是使用了方括号(`[]`).Rust允许使用任意一种方式,在这里使用方括号是为了遵守编程约定.

我们可以通过以下形式创建一个数组:

    let nums = [1i, 2i, 3i];
    let nums = [1i, ..20]; // Shorthand for an array of 20 elements all initialized to 1
    
它们之间有什么不同呢?数组是一种定长的数据结构,所以你不能向它添加或删除元素:

    let mut nums = vec![1i, 2i, 3i];
    nums.push(4i); // works

    let mut nums = [1i, 2i, 3i];
    nums.push(4i); //  error: type `[int, .. 3]` does not implement any method
                   // in scope named `push`
                   
`push()`函数用于向向量的尾部添加元素.因为数组是定长的,向其添加元素没有任何意义.从编译的错误提示中你可以看到数组的真实类型:`[int, .. 3]`,一个长度为3,的整型数组.

现在再来看下切片,如`&str`一样,切片实际上是对另一个数组的引用.可以通过`sa_slice反法`获得一个向量的切片:

    let vec = vec![1i, 2i, 3i];
    let slice = vec.as_slice();
    
上述的三种类型都实现了`iter()`方法,`iter()`会返回一个迭代器.我们会在后面详细的讨论迭代器,在这里,我们只要知道`iter()`方法使得我们可以写一个`for`循环打印一个向量,数组或切片中的所有元素:

    let vec = vec![1i, 2i, 3i];

    for i in vec.iter() {
        println!("{}", i);
    }
    
我们可以通过下标表示法访问一个向量,数组或切片中的任意一个元素:

    let names = ["Graydon", "Brian", "Niko"];

    println!("The second name is: {}", names[1]);
    
如大多数编程语言一样,下标起始于0,所以`name`中的第一个元素是`names[0]`第二个元素是`names[1]`.上面的代码将打印name中的第二个元素:Brian.

`Vectors`的内容远不止这些,但上面的介绍已经足够让我们开始使用`Vectors`了.现在我们已经学习了Rust的所有基本概念.我们可以开始构建一个小示例:猜谜游戏了.但在此之前我们必须先学会如何获取键盘输入.

###<span id="13 标准输入">13 标准输入</span>

从键盘获取输入非常简单,但使用了一些我们之前没接触过的概念.下面的代码片段从标准入读取,并将读取到的东西输出到标准输出:

    fn main() {
        println!("Type something!");

        let input = std::io::stdin().read_line().ok().expect("Failed to read line");

        println!("{}", input);
    }

我们现在分步的解释上述代码.

    std::io::stdin();

这段代码调用了`std::io`模块中的`stdin()`函数.如你能想象到的,`std`是Rust的标准库.我们将会在后面讨论Rust的模块系统.

由于每次都使用冗长的限定符是一件烦人的事,我们可以通过`use`语句导入我们使用到的东西:

    use std::io::stdin;

    stdin();
    
但是,导入单独的函数不是一种好的编程风格,我们通常导入整模块,只使用一级的限定符:

    use std::io;

    io::stdin();

我们使用上述风格改写之前的代码片段:

    use std::io;

    fn main() {
        println!("Type something!");

        let input = io::stdin().read_line().ok().expect("Failed to read line");

        println!("{}", input);
    }
    
接下来:

	.read_line()
    
我们在`stdin()`返回的结果上调用`read_line`方法以获取标准输入中输入的整行内容.很简单吧.

	.ok().expect("Failed to read line");
    
还记得下面这段代码吗?

    enum OptionalInt {
        Value(int),
        Missing,
    }

    fn main() {
        let x = Value(5);
        let y = Missing;

        match x {
            Value(n) => println!("x is {:d}", n),
            Missing  => println!("x is missing!"),
        }

        match y {
            Value(n) => println!("y is {:d}", n),
            Missing  => println!("y is missing!"),
        }
    }
    
我们每次都检查,看是否获得了一个值.对于这个示例来说,我们已经知道了`x`是有值的.但`match`语句强迫我们处理`missing`的情况.在99%的情况下这可能就是我们想要的,但在有些情况下,我们比编译器更清除自己想要的是什么.

类似的,`read_line()`并不是一定会返回从标准输入中读取到的正行数据.它可能成功也可能失败.例如如果我们的程序并不是运行在终端环境中,而是作为一个corn任务运行,或者在其它没有标准输入的环境中运行,`read_line`就会失败.因此,`read_line`的返回值是一个与`OptionalInt` 有点类似的类型:`IoResult<T>`.我们还没有讨论过`IoResult<T>`类型,它是比`OptionalInt`更通用的一种形式.但在此,我们可以先简单的认为它们俩是一样的.

Rust provides a method on these IoResult<T>s called ok(), which does the same thing as our match statement, but assuming that we have a valid value. We then call expect() on the result, which will terminate our program if we don't have a valid value. In this case, if we can't get input, our program doesn't work, so we're okay with that. In most cases, we would want to handle the error case explicitly. expect() allows us to give an error message if this crash happens.

我们将会在后面的内容中详细讨论这些机制是如何工作的.但就目前来说,以上的介绍已经足够让你了解这段代码是如何工作的了.

让我们回到代码:

    use std::io;

    fn main() {
        println!("Type something!");

        let input = io::stdin().read_line().ok().expect("Failed to read line");

        println!("{}", input);
    }

这段代码中有一行显得有些太长了,不是太利于阅读,Rust允许我们将代码改写如下:

    use std::io;

    fn main() {
        println!("Type something!");

        let input = io::stdin()
                      .read_line()
                      .ok()
                      .expect("Failed to read line");

        println!("{}", input);
    }
    
以上就是与标准输入相关的基本知识.后面我们将要开始介绍我们的猜谜游戏,虽然这个游戏的实现不太复杂,但我还是将它分成不同的小节.


###<span id="14 猜迷游戏">14 猜迷游戏</span>

我们现在已经获得了Rust的基本知识,现在让我们来写一个稍微复杂点的程序.

作为我们的第一个项目,我们将实现一个对于编程初学者来说的经典问题:猜谜游戏.

我们的程序将会产生一个1到100之间的随机数,然后在屏幕中提示你输入一个猜测的数字.当我们输入猜测值之后,程序会提示我们的输入是太小还是太大.如果我们输入的值正好就是程序产生的随机数,程序会祝贺我们并把那个数输出到屏幕上。

#### 14.1 设置

让我们开始设置我们的新项目.首先进入到我们的项目目录.还记得是怎么为我们的`hello_world`项目创建目录结构和`Cargo.toml`文件么.Cargo有一个命令可以帮我们完成这些事,让我们来看一下:

    $ cd ~/projects
    $ cargo new guessing_game --bin
    $ cd guessing_game
    
我们把项目的名字传给`cargo new`,并使用`--bin`标记,因为我们要创建的是一个可执行文件而不是一个库文件.

让我们来检查一下生成的`Cargo.toml`文件:

    [package]

    name = "guessing_game"
    version = "0.0.1"
    authors = ["Your Name <you@example.com>"]
    
Cargo从当前环境中获取这些信息,如果有不对的地方你可以自己去修改.

最后,Cargo还为我们生成了`src/main.rs`文件,内容只是输出一行"hello,world":

    fn main() {
        println!("Hello, world!");
    }
    
现在让我们来编译一下这个项目:

    $ cargo build
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
       
一切正常!现在用你熟悉的编辑器打开`src/main.rs`.现在我们暂时将所有的代码都写在这个文件里面.多文件的项目我们将会在后面介绍.

在继续之前,让我再向你展示另一个Cargo命令:`run.cargo`,`run`与`cargo build`很相似,除了它在编译完项目之后会继续启动执行编译完成的可执行程序.让我们试一下:

    $ cargo run
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
         Running `target/guessing_game`
    Hello, world!
    
很棒!不是吗.

#### 14.2 处理猜测

首先我们要让玩家可以输入猜测的数字,将下面代码复制到你的`src/main.rs`中:

    use std::io;

    fn main() {
        println!("Guess the number!");

        println!("Please input your guess.");

        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");

        println!("You guessed: {}", input);
    }
    
你应该在上一章中看过与上面类似的代码.我们通过`use`将`std:io`模块导入,然后在`main`函数中包含了所有的程序逻辑.我们输出一行介绍,然后提示玩家输入一个猜测的数字,获取输入然后将输入输出到标准输出中.

输入相关的内容我们在上一章中都已经介绍过了,所以在本章不会再介绍,如果你需要再温习一下,请重新阅读上一章.

#### 14.3 产生一个秘密的数字

接下来,我们要生成一个秘密数字.为此,我们要使用Rust的随机数生成函数.Rust标准库中包含了很多有趣的函数,当你要实现某个功能的时候,首先要先查看一下标准库,没准在标注库中已经有现成的了.在本例中,我们知道标准库中提供了随机数生成函数,但我们还不知道如何去使用它.

在Rust的主页中,有标准库的详细介绍.你可以从[这里](http://doc.rust-lang.org/0.12.0/std/)跳转过去.这个页面中包含了相当多的信息,但最有用的是右上角的搜索栏.如果你在搜索栏中输入'random',页面将会更新成[这样](http://doc.rust-lang.org/0.12.0/std/?search=random).其中第一条链接跳转到[std::rand::random](http://doc.rust-lang.org/0.12.0/std/rand/fn.random.html).当我们点击这个链接就会进入`std::rand::random`的详细文档.

这个文档向我们展示了很多东西:函数的签名,一些描述性的文字和一个简单的示例.现在让我们改写上述代码把`random`函数添加进去:

    use std::io;
    use std::rand;

    fn main() {
        println!("Guess the number!");

        let secret_number = (rand::random() % 100i) + 1i;

        println!("The secret number is: {}", secret_number);

        println!("Please input your guess.");

        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");


        println!("You guessed: {}", input);
    }
    
首先我们通过`use std::rand`导入rand模块,接着将生成的随机数绑定到`secret_number`变量,然后将这个变量输入到屏幕.

你可能会奇怪为什么要将`%`作用在`rand:random()`的返回值上.这个操作符叫做取模,它返回两数相除的余数.通过将`rand:random()`的返回值对100取模,我们将值的范围限定在0到99之间.然后我们再对这个值加1将`secret_number`的值限定在1到100之间.取模运算会有一点小的误差,但对本例而言并不严重.

现在让我们编译代码:

    $ cargo build
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
    src/main.rs:7:26: 7:34 error: the type of this value must be known in this context
    src/main.rs:7     let secret_number = (rand::random() % 100i) + 1i;
                                           ^~~~~~~~
    error: aborting due to previous error

生成失败!Rust提示"the type of this value must be known in this context."这到底是怎么回事?原因就是,`rand::random()`可以产生多种类型的随机值而不仅仅是整型,而在当前的上下文下,Rust无法确定`random()`到底要生成什么类型的随机数.所以我们必须为编译器提供提示.对于字面量而言,我们可以通过在末尾添加`i`来提示编译器这是整型的,但这种做法对函数不起作用.我们需要特殊的语法形式:

	rand::random::<int>();
    
这样就告诉编译器请给我一个整型的随机数.我们调整下我们的代码:

    use std::io;
    use std::rand;

    fn main() {
        println!("Guess the number!");

        let secret_number = (rand::random::<int>() % 100i) + 1i;

        println!("The secret number is: {}", secret_number);

        println!("Please input your guess.");

        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");


        println!("You guessed: {}", input);
    }

编译并运行程序几次：

    

