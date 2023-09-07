# 异常处理分析

> 根据询问老师后，最后的异常处理思路请见[此处](./thinking.md)。

## 运行逻辑

整个程序的按照模块化的方式进行划分，包含以下4个流程

1. 读取：从文件中读取源代码文件，进行简单的预处理（去除空行、空白字符、注释等），以行为单位保存在字符串数组。
2. 解析：将字符串文本逐行解析，保存为中间数据结构Cmd，并进行检查
3. 转换：将中间数据结构转换为32为无符号整型
4. 输出：将转换结果（机器指令）输出到本地文件

由于读取和输出不是本程序的核心功能，因此不在此赘述。

### 解析

**为什么需要解析部分？**

从理论上来讲，程序是可以边解析边转换的。但是转换的开销是要比转换大的，设想一下，如果遇到较长的代码量，采用边解析边转换的机制，当出现解析错误时，将花费更多的时间。因此，将解析部分和转换部分解耦，先解析，再转换，能够更快地确定代码中的错误。

**解析部分是怎么工作的？**

1. 分解为字符串流：按照MIPS汇编格式，可以按空格对一行字符串进行划分
   由于空行被剔除了，划分后的字符串肯定是>=1的
2. 解析操作数：字符串流中第一个为指令名称，后面都是操作数
   所以需要对后面的字符串，需要转换为整型数据（此处可能会出现操作数解析错误）
3. 检验解析结果：主要以三个方面来检验
   * 指令名称：指令名称是否存在
   * 操作数数量：操作数数量是否相等
   * 操作数大小：任意操作大小是否在给定的长度之内

### 转换

转换的逻辑

1. 根据Cmd名称从CmdManager中查找到对应的CmdKind
2. 使用CmdKind.to_code()将每个输入的操作数转换为32位整形
3. CmdKind.to_code()则是根据指令所拥有的操作数调用CmdOprand.to_code()

## 异常处理分析

#### 步骤1： 编译逻辑层

> 所在位置: `compile.rs`

先通过指令名称在CmdManager中找对对应指令类型，再调用其函数to_code进行转换。

```rust
let mut codes: Vec<u32> = Vec::new();
for cmd in cmds {
    let cmd_kind = MANAGER.get(cmd.name()).unwrap();
    let code = cmd_kind.to_code(cmd.nums()).unwrap();
    codes.push(code);
}
```

* 异常处理方案：忽略异常（u32）

* 原因：这是因为由于代码的逻辑设计，能够保证在这此处能够准确无误运行，因此可以放心unwrap

#### 步骤2：指令类型层

> 所在位置: `command.rs`

由于Rust语言本身的特点，对于CmdKind这种枚举类型，只能使用match的方式处理。其实现逻辑（对于R型指令）如下：

1. 嵌入操作数

2. 嵌入FUNCT

   > 在汇编指令中，每条R指令的FUCT 值是不会变的，类似于指令的唯一ID

前者我们封装了函数`embed_num`用于处理，后者直接使用Oprand.to_code函数处理即可。`embed_num`的实现，就是循环调用每个操作数的to_code函数，并将其累加进来。

为了保证接口的鲁棒性，我们还是对操作数的值的数量与操作数数量进行检验，否则直接使用运算符[]对数组取值，当数组越界时，就会出现不可恢复错误。

> 当然，这里也可以替换成使用使用get()函数
>
> 在Rust中，vector存在两种取值方式，使用运算符[]与get()函数。前者当出现越界时会产生不可恢复错误panic，后者则会返回Result，需要错误处理。当然，此处也可以使用get()函数，但笔者暂时还是这样处理。

```rust
impl CmdKind {  
    pub fn to_code(&self, nums: &Vec<usize>) -> Result<u32, CompileErrKind> {
        match self {
            CmdKind::R{funct, operands, ..} => {
                let num = embed_oprands(nums, operands)?;
                Ok(Operand::FUNCT.to_code(*funct) + num)
            },
            // 处理I, J指令类型...
        }
    } 
}

// 内联函数: 用于嵌入操作数
#[inline(always)]
fn embed_oprands(nums: &Vec<isize>, parts: &Vec<Operand>) -> Result<u32, CompileErrKind> {
    fn get_compile_err(nums: &Vec<isize>, parts: &Vec<Operand>) -> CompileErrKind {
        CompileErrKind::OperandNumError { 
            expected: parts.len(), found: nums.len() 
        }
    }

    let mut res:u32 = 0;
    for i in 0..parts.len() {
        // 此处不单独进行条件验证 而是采用出错报错的策略
        let part = parts.get(i).ok_or(get_compile_err(nums, parts))?;
        let num = nums.get(i).ok_or(get_compile_err(nums, parts))?;

        res += part.to_code(num);
    }

    Ok(res)
}


```

* 异常处理方式： 出错时报错
* 原因：在一层的逻辑中，可能出错的只有数据越界问题，这个可以通过异常处理解决。如果使用额外一层check，函数本身变得冗余。

当然，方法2只有当`len(arr1)>len(arr2)`的情况才会抛出异常。

1. 如果站在整个程序的切面，由于先前的解析层，尽管这样设计也不会有任何问题。（甚至不异常处理也不会有问题）。
2. 站在整个函数的切面，这样的处理确实也能够处理一定的异常情况。对于`len(arr1)<len(arr2)`的情况，由于nums和parts两个权重不一样，如果nums的个数大于parts，我们只需要忽略多余的操作数值就可以了。

> 其实check的方式并不算真正的异常处理
>
> 异常处理的方式：（如果一开始知道是否正确，何必等到后面真正运行的时候？）
>
> 1. 直接在程序的开始进行条件验证
> 2. 当真正出现问题的时候，才进行条件验证

#### 步骤3：操作数层

>  所在位置: `operand.rs`

在操作数层的to_code中，我们采取的实现策略就有所不同。我们并没有显式进行异常处理，而是通过直接取模的方式的方式。

解决方案：强制转换

原因：在操作数转换的时候，（大多时候）不会出现异常。如果对输入不管不顾的话，最终输出的结果可能也会发生错误。但是在底层函数，我们希望不需要太多的异常回复，而是更加宽容的输出。

> 但其实如果采用强制转换策略，产生的结果可能并非用户的预期

```rust 
impl Operand {
    // 将数字转换到对应位置上
    pub fn to_code(&self, num: usize) -> u32 {
        let max_range = (1<<self.info().length) - 1;
        // 当数组超出给定范围时 值取前32位
        let res: u32 = (num & max_range).try_into().unwrap();
        res << self.info().offset
    } 
} 
```
