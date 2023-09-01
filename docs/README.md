# 开发文档

本程序的工作步骤为
1. `cmd`: 指令数据结构
2. `compile`: 编译指令
3. 输出结果

## CmdKind 指令类型
Command是程序已经定义好的指令类型，MIPS架构下有R、I和J三种指令类型，因此采用enum类型存储。

每种指令类型下，主要的数据类型为
* `name`: 指令的名称
* `funct/op`: 指令唯一的表示符
* `operands`: 操作数类型

### to_code()
该函数主要是用于将给定的操作数转换成对应的机器指令。

## Cmd 指令解析结构
Cmd与Command不同，Cmd是程序解析后的中间结果，是用于存储解析后的数据。
> tips: 为什么有了Command，我们还需要Cmd呢？
> 
> 本程序的编译过程包括：读取-> 解析-> 检查-> 转换。因此我们需要一个数据结构存储中间的解析结果，以方便我们下一步检查。
>
> 当然，如果我们对于每一条指令，直接解析、检查再转换就可以了，但是转换的开销是远大于检查的，如果在转换过程中检查出错，那么先前的转换都白费了。因此我们考虑将上面的部分都解耦开。

## Operand 操作数类型
MIPS架构中的操作数类型相对固定，因此我们使用enum类型定义，并提供一下接口。

### info()
枚举类型在rust属于一种数据类型，并不能直接存储信息，因此我们引入一个新的结构体`OperandInfo`，再通过match关键字选取返回。

以下是各操作数中的信息。

|操作数|长度(length)|偏移量(offset)|指令类型|
|--|----|----|--|
|OP| 6|26|R, I|
|RS|5|21|R,I|
|RT|5|16|R,I|
|RD|5|11|R|
|SHAMT|5|6|R|
|FUNCT|6|0|R|
|IMM|17|0|I|

### convert_num(num: usize)
此函数用于将对应操作数上的值转换到对应位置上。

### check(num: usize)
此函数用于验证值是否超出了操作的大小。

## CommandManager 指令管理器
指令管理器是所有指令的集合，其中主要的数据结构是以指令名称(String)为key，指令类型(CmdKind)为value的哈希表。

TODO: 规定指令管理中字符的大小写问题

### new(read_path)
管理器的初始化需要输入存储指令类型信息的Json文件路径，然后程序会依次解析Json文件，并输出为上述的哈希表。

### get(cmd_name)
根据给定的指令名称，从哈希表中读取指令类型。
> 注意：指令名称统一使用小写字母

### cmd_num()
返回管理器中存在的指令数量。

### check(cmd)
验证输入的指令结构，判断该指令是否满足以下条件
1. 指令名称是否存在
2. 操作数数目是否相同
3. 操作数大小是否在限制内