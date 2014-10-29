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

Rust是一种系统编程语言,它关注的焦点是高级的,裸机编程(bare-metal programming):在获得低级编程语言才有的对系统的控制能力的同时获得零消耗的高级抽象机制.

为了向您展示如何使用Rust编程,我们首先向你展示一个传统的"hello world!"程序.接着我向你介绍编写真实世界中Rust程序和库所用到的工具:Cargo.再后面我会介绍Rust的基本知识,展示一些简单的小程序,最后再学习更多高级的知识.

听着不错是吗?让我们开始吧.

###<span id="1 安装Rust">1 安装Rust</span>

使用Rust的第一步就是在你的系统中安装Rust!有多种方式安装Rust,但最简单的方式莫过于使用`rustup`脚本了.如果你在Linux或Mac系统下你所要做的事情就是在命令行中输入以下命令:

    $ curl -s https://static.rust-lang.org/rustup.sh | sudo sh

如果你在Windows下,请下载[ 32-bit installer](https://static.rust-lang.org/dist/rust-nightly-i686-w64-mingw32.exe)或[ 64-bit installer](https://static.rust-lang.org/dist/rust-nightly-x86_64-w64-mingw32.exe)然后运行.

如果你不再需要Rust了可以通过以下方式将它删除:

    $ curl -s https://static.rust-lang.org/rustup.sh | sudo sh -s -- --uninstall

如果你在Windows下,则再次运行你所下载的安装文件然后选择删除.

你可以通过再次执行脚本来将Rust更新到最新版本.

###<span id="2 Hello, world!">2 Hello, world!</span>

现在你已经安装好了Rust,让我们开始编写第一个Rust程序.如同我们在学习其它语言时的传统,我们的第一个程序就是向屏幕打印一行"Hello World!".通过这个简单的程序可以让你确定你的编译器不仅安装好了而且可以正常的工作.

首先我们要创建一个文件将代码码进去.通常我喜欢在我们home目录下建立一个`projects`目录,把我的所有工程都放到这个目录下.Rust不关心你的代码到底在哪.

这个指南假设你熟悉基本的命令行命令.Rust不需要你具备完整的命令行知识,但在语言已经到达一个完成的状态之前我们都缺乏IDE的支持.

现在让我们在projects目录下新建一个目录.

    $ mkdir ~/projects
    $ cd ~/projects
    $ mkdir hello_world
    $ cd hello_world
    
如果你在Windows下且没有使用PowerShell,那么`~`将无法工作.请参看你的shell文档获得详细的帮助.

接着我们创建一个新的源文件.我用`editor filename`来表示编辑一个文件.我将新创建的文件命名为`main.rs`:

    $ editor main.rs
    
Rust源文件的后缀为`.rs`.如果你想将文件命明为超过一个词,那么建议在词的中间使用下划线作为分割.也就是说应该是`hello_world.rs`而不是`helloworld.rs`.

现在把下面的内容添加到你的文件中:

    fn main() {
        println!("Hello, world!");
    }

保存文件然后在命令行中输入以下命令:

    $ rustc main.rs
    $ ./main # or main.exe on Windows
    Hello, world!
    
成功了!现在让我们回头看看里面的细节.

    fn main() {
    
    }
    
这几行代码定义了一个Rust函数.`main`函数是一个特殊的函数,所有的Rust程序都从这里开始执行.第一行代码标识,声明一个叫做`main`的函数,这个函数没有输入参数也不返回任何东西.如果想为它添加输入参数,输入参数必须放在括号里面.

你可能还注意到,函数体被包裹在花括号之间.通常来说,将左花括号与函数声明放在同一行,并在它们之间留一个空格的位置是一种良好的编程风格.

接下来是下面这一行:

    println!("Hello, world!");
    


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

类似的,`read_line()`并不是一定会返回从标准输入中读取到的正行数据.它可能成功也可能失败.例如如果我们的程序并不是运行在终端环境中,而是作为一个corn任务运行,或者在其它没有标准输入的环境中运行,`read_line`就会失败.因此,`read_line`的返回值是一个与`OptionalInt` 有点类似的类型:`IoResult<T>`.我们还没有讨论过`IoResult<T>`类型,它是比`OptionalInt`更通用的一种形式.但在此,我们可以先简单的认为它们二者几乎是等价的.除了`T`可以是任何类型,不仅仅是`int`.

Rust为`IoResult<T>`提供了一个名叫`ok()`的方法,这个方法做的事情跟我们上面的match语句类似,除了它假设我们应该有一个合法值.
之后我们在`ok()`的返回值上调用`expect()`,如果没有获得一个有效值程序就会终止.对于目前的情况来说,如果不能获得输入我们的程序就无法继续工作下去,所以这样是合理的.但在通常的情况下,我们需要手动去处理错误情况.`expect`允许我们传递一个字符串来描述错误情况,当程序崩溃的时候可以提示用户.

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

    $ cargo run
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
         Running `target/guessing_game`
    Guess the number!
    The secret number is: 57
    Please input your guess.
    3
    You guessed: 3
    
这回正常了,我们接着要实现`secret_number`与玩家猜测数的比较.
    
#### 14.4 与猜测值比较

你可能还记得我们在前面的章节中实现过用于比较两个数字大小的`cmp`函数.让我们把`cmp`加进去,再添加一段代码将玩家的猜测值与`secret_number`做比较:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
    
    
        println!("You guessed: {}", input);
    
        match cmp(input, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => println!("You win!"),
        }
    }
    
    fn cmp(a: int, b: int) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }

如果我们尝试编译这段代码,我们会得到如下错误:

    $ cargo build
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
    src/main.rs:20:15: 20:20 error: mismatched types: expected `int` but found `collections::string::String` (expected int but found struct collections::string::String)
    src/main.rs:20     match cmp(input, secret_number) {
                                 ^~~~~
    src/main.rs:20:22: 20:35 error: mismatched types: expected `int` but found `uint` (expected int but found uint)
    src/main.rs:20     match cmp(input, secret_number) {
                                        ^~~~~~~~~~~~~
    error: aborting due to 2 previous errors
    
这在编写Rust程序的时候会经常出现,而这是Rust最大的优势之一.你尝试编译代码,如果编译不通过,Rust会提示你哪里出现了错误.在此例中,`cmp`函数需要的是整型参数,而我们提供的是无符号整型.我们可以通过将`cmp`的参数改为`uint`来修正这个问题:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
    
    
        println!("You guessed: {}", input);
    
        match cmp(input, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => println!("You win!"),
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
再尝试编译代码:

    $ cargo build
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
    src/main.rs:20:15: 20:20 error: mismatched types: expected `uint` but found `collections::string::String` (expected uint but found struct collections::string::String)
    src/main.rs:20     match cmp(input, secret_number) {
                                 ^~~~~
    error: aborting due to previous error    

错误与上次的类似,期望`uint`的参数,但得到的却是`String`类型.这是因为我们是从标准输入获得的猜测值,
而实际上你可以在标准输入中输入任何内容,例如:

    $ ./target/guessing_game
    Guess the number!
    The secret number is: 73
    Please input your guess.
    hello
    You guessed: hello
    
好吧,我们获得的是`String`类型,而我们需要的是`uint`.怎么办呢?有一个函数可以为我们处理这种情况:

    let input = io::stdin().read_line()
                           .ok()
                           .expect("Failed to read line");
    let input_num: Option<uint> = from_str(input.as_slice());
    
`from_str`函数将一个`&str`的值转换成其它类型.我们需要为它提供一个提示使得它能正确的完成转换.还记得我们给`random()`的提示吗?像这个样子:

    rand::random::<uint>();
    
还有另外一种提示方式,将类型声明添加到`let`中:

    let x: uint = rand::random();
    
这相当于显式的通告`x`是`unit`类型,所以Rust可以正确的告诉`random()`函数要生成什么类型的值.类似的,下面两种方式都是合法的:

    let input_num = from_str::<uint>("5");
    let input_num: Option<uint> = from_str("5");
    
现在让我们将输入转换成数值型,代码如下:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
        let input_num: Option<uint> = from_str(input.as_slice());
    
        println!("You guessed: {}", input_num);
    
        match cmp(input_num, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => println!("You win!"),
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
编译:

    $ cargo build
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
    src/main.rs:22:15: 22:24 error: mismatched types: expected `uint` but found `core::option::Option<uint>` (expected uint but found enum core::option::Option)
    src/main.rs:22     match cmp(input_num, secret_number) {
                                 ^~~~~~~~~
    error: aborting due to previous error
    
好吧,`input_num`的类型是`Option<uint>`而不是`uint`.我们需要将`Option`中的内容解包.如果你还记得前面的内容,`match`用来处理这种情况最好不过了:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
        let input_num: Option<uint> = from_str(input.as_slice());
    
        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("Please input a number!");
                return;
            }
        };
    
    
        println!("You guessed: {}", num);
    
        match cmp(num, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => println!("You win!"),
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    } 
    
我们通过`match`来提取`Option`中的`uint`值,如果失败输出提示消息并退出程序.让我们试下:

    $ cargo run
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
         Running `target/guessing_game`
    Guess the number!
    The secret number is: 17
    Please input your guess.
    5
    Please input a number!
    
还是失败了.当我们从`stdin()`中获取输入时,我们实际上获得了所有的键盘输入,包括你按下的回车符.所以当`from_str()`看到"5\n"的时候,会抱怨这里没有数字.幸运的是`&str`有一个`trim()`方法可以处理这种情况.我们对代码做如下修改:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");
        let input_num: Option<uint> = from_str(input.as_slice().trim());
    
        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("Please input a number!");
                return;
            }
        };
    
    
        println!("You guessed: {}", num);
    
        match cmp(num, secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => println!("You win!"),
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
再试一下:

    $ cargo run
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
         Running `target/guessing_game`
    Guess the number!
    The secret number is: 58
    Please input your guess.
      76
    You guessed: 76
    Too big!
    
漂亮!这次程序正常的工作了.

Rust编译器对我们的工作提供了极大的帮助!这种技术被称为“从编译器中学习”.通过编译器输出的错误提示来指引我们修正错误.

现在我们的猜谜游戏已经基本完成,唯一的问题是我们只能猜一次,让我们为它添加一个循环处理来解决这个问题.

#### 14.5 添加循环

如我们先前讨论过的,`loop`关键字为我们提供了无限循环.我们将它添加到代码中:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        loop {
    
            println!("Please input your guess.");
    
            let input = io::stdin().read_line()
                                   .ok()
                                   .expect("Failed to read line");
            let input_num: Option<uint> = from_str(input.as_slice().trim());
    
            let num = match input_num {
                Some(num) => num,
                None      => {
                    println!("Please input a number!");
                    return;
                }
            };
    
    
            println!("You guessed: {}", num);
    
            match cmp(num, secret_number) {
                Less    => println!("Too small!"),
                Greater => println!("Too big!"),
                Equal   => println!("You win!"),
            }
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
虽然我们添加了一个无限循环,但记住当输入非数字的时候我们使用了`return`,这会导致程序的退出:

    $ cargo run
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
         Running `target/guessing_game`
    Guess the number!
    The secret number is: 59
    Please input your guess.
    45
    You guessed: 45
    Too small!
    Please input your guess.
    60
    You guessed: 60
    Too big!
    Please input your guess.
    59
    You guessed: 59
    You win!
    Please input your guess.
    quit
    Please input a number!
    
输入quit的时候我们退出了程序,而实际上你输入任何非数字都会导致程序退出.现在让我们调整下代码,当成功猜到数字的时候让程序退出:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        loop {
    
            println!("Please input your guess.");
    
            let input = io::stdin().read_line()
                                   .ok()
                                   .expect("Failed to read line");
            let input_num: Option<uint> = from_str(input.as_slice().trim());
    
            let num = match input_num {
                Some(num) => num,
                None      => {
                    println!("Please input a number!");
                    return;
                }
            };
    
    
            println!("You guessed: {}", num);
    
            match cmp(num, secret_number) {
                Less    => println!("Too small!"),
                Greater => println!("Too big!"),
                Equal   => {
                    println!("You win!");
                    return;
                },
            }
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
通过在`You win!`的后面添加`return`,当我们胜利的时候程序就会结束.然后我们希望当玩家输入非数字的时候不是退出程序,而只是忽略这个输入.我们可以通过将第一个`return`改成`continue`来实现:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        println!("The secret number is: {}", secret_number);
    
        loop {
    
            println!("Please input your guess.");
    
            let input = io::stdin().read_line()
                                   .ok()
                                   .expect("Failed to read line");
            let input_num: Option<uint> = from_str(input.as_slice().trim());
    
            let num = match input_num {
                Some(num) => num,
                None      => {
                    println!("Please input a number!");
                    continue;
                }
            };
    
    
            println!("You guessed: {}", num);
    
            match cmp(num, secret_number) {
                Less    => println!("Too small!"),
                Greater => println!("Too big!"),
                Equal   => {
                    println!("You win!");
                    return;
                },
            }
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
让我们再来尝试一下:

    $ cargo run
       Compiling guessing_game v0.0.1 (file:///home/you/projects/guessing_game)
         Running `target/guessing_game`
    Guess the number!
    The secret number is: 61
    Please input your guess.
    10
    You guessed: 10
    Too small!
    Please input your guess.
    99
    You guessed: 99
    Too big!
    Please input your guess.
    foo
    Please input a number!
    Please input your guess.
    61
    You guessed: 61
    You win!
    
漂亮!还差最后一点我们就完成猜迷游戏了.不再将`secret_number`输出到屏幕.下面就是完整的代码:

    use std::io;
    use std::rand;
    
    fn main() {
        println!("Guess the number!");
    
        let secret_number = (rand::random::<uint>() % 100u) + 1u;
    
        loop {
    
            println!("Please input your guess.");
    
            let input = io::stdin().read_line()
                                   .ok()
                                   .expect("Failed to read line");
            let input_num: Option<uint> = from_str(input.as_slice().trim());
    
            let num = match input_num {
                Some(num) => num,
                None      => {
                    println!("Please input a number!");
                    continue;
                }
            };
    
    
            println!("You guessed: {}", num);
    
            match cmp(num, secret_number) {
                Less    => println!("Too small!"),
                Greater => println!("Too big!"),
                Equal   => {
                    println!("You win!");
                    return;
                },
            }
        }
    }
    
    fn cmp(a: uint, b: uint) -> Ordering {
        if a < b { Less }
        else if a > b { Greater }
        else { Equal }
    }
    
#### 14.6 完结

到此,首先恭喜完成了猜谜游戏的构建!

你已经了学会了Rust的基本语法.所有的这些应该与你以前接触过的其它程序设计语言有点相似.这些基本的语法和语义元素将是你后续对Rust进行更深入学习的基石.

###<span id="15 包装和模块">15 包装和模块</span>

Rust提供了一种强大的模块化系统,它的工作方式与别的程序设计语言不同.Rust模块系统的两个核心组件是:包装和模块.

包装是Rust独立的编译单元.Rust通常每次编译一个包装,生成一个库或者一个可执行文件.但是,通常来说可执行文件会依赖于一些库,而库又可能依赖于其它的库.为了对此提供支持,包装可以依赖其它的包装.

每个包装中都包含了模块的层级树.这棵树起始于一个唯一的模块,被称为包装的根.我们可以在包装的根里声明其它模块,在这些模块中又可以包含其它的模块.

注意我们到现在都没有提到文件相关的事情.Rust不强制要求文件系统结构和模块结构之间有特定的关系.也就是说,Rust可以按常规的惯例在文件系统中查找模块,也可以不按这种惯例查找.

现在让我们来实践一下!我们建立一个名为`modules`的新项目.

    $ cd ~/projects
    $ cargo new modules --bin
    $ cd modules
    
让我们检查一下看是否能通过编译:

    $ cargo run
       Compiling modules v0.0.1 (file:///home/you/projects/modules)
         Running `target/modules`
    Hello, world!
    
现在我们已经有了一个包装:`src/main.rs`就是一个包装.在那个文件中的任何东西都在包装的根中.在这里我们创建了一个产生可执行文件的包装,它的根中只定义了一个`main`函数.

让我们在这个包装中定义一个新的模块.编辑`src/main.rs`如下:

    fn main() {
        println!("Hello, world!");
    }
    
    mod hello {
        fn print_hello() {
            println!("Hello, world!");
        }
    }
    
现在在我们的包装根中有了一个名为`hello`的模块.模块使用`snake_case`的命名法,跟函数和变量绑定一样.

我们在`hello`模块内定义了`print_hello`函数.这个函数用于输出"hello world".模块让你可以将一个程序的代码按功能和职责分成不同的小块.每一小块只包含相关的东西把其它不相关的隔离在外.

为了调用`print_hello`,我们需要使用双冒号:

    hello::print_hello();   

这种用法我们在`io::stdin()` 和`rand::random()`中就已经见到过.Rust模块系统还有一个可见性规则,用于控制谁可以访问一个模块中定义的函数.默认情况下,一个模块中定义的任何东西都是私有的,也就是说默认情况下一个模块中定义的函数或变量只能被同一个模块中定义的其它函数访问.所以下面的代码将无法通过编译:

    fn main() {
        hello::print_hello();
    }
    
    mod hello {
        fn print_hello() {
            println!("Hello, world!");
        }
    }
    
编译会出现以下的错误:

       Compiling modules v0.0.1 (file:///home/you/projects/modules)
    src/main.rs:2:5: 2:23 error: function `print_hello` is private
    src/main.rs:2     hello::print_hello();
                      ^~~~~~~~~~~~~~~~~~
                      
我们可以使用关键字`pub`将`print_hello`变成公有的:

    fn main() {
        hello::print_hello();
    }
    
    mod hello {
        pub fn print_hello() {
            println!("Hello, world!");
        }
    }
    
使用`pub`有时被称为导出,因为我们让被标记为`pub`的函数可以在其它模块中调用.所以现在编译可以通过了:

    $ cargo run
       Compiling modules v0.0.1 (file:///home/you/projects/modules)
         Running `target/modules`
    Hello, world!
    
模块相关的内容还有很多,例如将它移动到单独的文件中.但目前为止的介绍已经足够了.

