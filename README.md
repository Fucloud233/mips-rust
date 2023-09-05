# Project: MIPS编译器

使用Rust语言实现简单地MIPS汇编指令到机器指令的编译器。

> 例如：对于MIPS架构32位R型指令的构成为（括号内的为对应操作数所占位数）
>
> OP(6) + rs(5) + rt(5) + rd(5) +shamt(5) + funct(6)
>
> 所以对于指令`add rd rs rt`，只需要通过移位运算符将对应的操作数值填充到对应位置上，就可以完成汇编指令向机器代码的转换。
