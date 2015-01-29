
##表达式vs语句

Rust主要是一种基于表达式(expression)的语言.在Rust只有两种类型的语句(statements).其余皆是表达式.

表达式与语句的主要区别在于,表达式返回值而语句不返回(实际返回`()`).

Rust中的两种语句形式是`let`和`;`

因为let是一个语句,所以以下代码是非法的.

    let x = (let y = 5);

第一个let需要=号右边是一个值,而=号右边是一个let语句并不返回值.

下面再来看`;`,在一个表达式末尾加上`;`将导致表达式的值被丢弃返回`()`.而`()`不是一个有效的值类型.

看如下例子:

    fn f1() -> i32 {
        1
    }
    
    fn f2() -> i32 {
        1;
    }

其中f1是合法的,f1返回1,而f2则是非法的,`;`导致1被丢弃返回`()`,而这与返回值类型`i32`是不符的.


 