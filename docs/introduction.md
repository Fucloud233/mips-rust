# Project: MIPS编译器

使用Rust语言实现简单地MIPS汇编指令到机器指令的编译器。

> 例如：对于MIPS架构32位R型指令的构成为（括号内的为对应操作数所占位数）
>
> OP(6) + rs(5) + rt(5) + rd(5) +shamt(5) + funct(6)
>
> 所以对于指令`add rd rs rt`，只需要通过移位运算符将对应的操作数值填充到对应位置上，就可以完成汇编指令向机器代码的转换。

## 数据结构

本程序中使用的数据结构

#### Operand 指令操作数

* 类型：枚举
* 属性：类型、位数、偏移量

#### CmdKind 指令类型

* 类型：结构体

* 属性：指令名称、id、操作数（集合）

  > 一条指令中的操作数肯定不只一个

#### CmdManager指令管理器

* 类型：结构体
* 属性：指令名称与指令类型的映射表

#### Cmd 指令

* 类型：结构体
* 属性：指令名称，操作数的值（集合）

---

**注意：**Cmd与CmdKind不同

* Cmd是解析后的中间结果，存放的是具体的数值
  `add rd rs rt`
* CmdKind是一种指令模板，存放的是形式化上的操作数类型
  `add 1 2 3 `

## 工作流程

整个程序的按照模块化的方式进行划分，包含以下4个流程

1. 读取：从文件中读取源代码文件，进行简单的预处理（去除空行、空白字符、注释等），以行为单位保存在字符串数组。
2. 解析：将字符串文本逐行解析，保存为中间数据结构Cmd，并进行检查
3. 转换：将中间数据结构转换为32为无符号整型
4. 输出：将机器指令输出到本地文件

由于读取和输出不是本程序的核心功能，因此不在此赘述。

### 解析

**为什么需要解析部分？**

从理论上来讲，程序是可以边解析边转换的。但是解析的开销是要比转换大的，设想一下，如果遇到较长的代码量，采用边解析边转换的机制，当出现解析错误时，将花费更多的时间。因此，将解析部分和转换部分解耦，先解析，再转换，能够更快地确定代码中的错误。

**解析部分是怎么工作的？**

1. 分解为字符串流：按照MIPS汇编格式，可以按空格对一行字符串进行划分
   由于空行被剔除了，划分后的字符串肯定是>=1的
2. 解析操作数：字符串流中第一个为指令名称，后面都是操作数
   所以需要对后面的字符串，需要转换为整型数据
3. 检验解析结果：主要以三个方面来检验
   * 指令名称：指令名称是否存在
   * 操作数数量：操作数数量是否相等
   * 操作数大小：任意操作大小是否在给定的长度之内

### 转换

转换的逻辑

1. 根据Cmd中名称从CmdManager中查找到对应的CmdKind
2. 使用CmdKind.to_code()将每个输入的操作数转换为32位整形
3. CmdKind.to_code()则是根据指令所拥有的操作数调用CmdOprand.to_code()

#### 具体实现

##### 步骤1： 编译逻辑层

先通过指令名称在CmdManager中找对对应指令类型，再调用其函数to_code进行转换

```rust
let mut codes: Vec<u32> = Vec::new();
for cmd in cmds {
    let cmd_kind = MANAGER.get(cmd.name()).unwrap();
    let code = cmd_kind.to_code(cmd.nums()).unwrap();
    codes.push(code);
}
```

##### 步骤2：指令类型层

由于Rust语言本身的特点，对于CmdKind这种枚举类型，只能使用match的方式处理。

其实现逻辑（对于R型指令）：

1. 嵌入操作数

2. 嵌入FUNCT

   > 在汇编指令中，每条R指令的FUCT 值是不会变的，类似于指令的唯一ID

前者我们封装了函数`embed_num`用于处理，后者直接使用Oprand.to_code函数处理即可。`embed_num`的实现，就是循环调用每个操作数的to_code函数，并将其累加进来。

为了保证接口的鲁棒性，我们还是对操作数的值的数量与操作数数量进行检验，否则直接使用运算符[]对数组取值，当数组越界时，就会出现不可恢复错误。

> 当然，这里也可以替换成使用使用get()函数
>
> 在Rust中，vector存在两种取值方式，使用运算符[]与get()函数。前者当出现越界时会产生不可恢复错误panic，后者则会返回Result，需要错误处理。当然，此处也可以使用get()函数，但笔者暂时还是这样处理。

为什么要check？

在编译逻辑层中，编译逻辑中已经经过验证步骤，因此我们可以自信地unwrap。但是在指令类型层中，对这个结构体对象中，任何输入的方式都是未知的，因此需要check。当然有些错误，例如type检查是在编译时 就可以通过Rust的编译器鉴别，我们只关心运行时所发生的错误，比如说此处的操作数数量是否匹配。

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
fn embed_oprands(nums: &Vec<usize>, parts: &Vec<Operand>) 
	-> Result<u32, CompileErrKind> {
    if parts.len() != nums.len() {
        return Err(CompileErrKind::OperandNumError{
            expected: nums.len(),
            found: nums.len()
        })
    }

    let mut res:u32 = 0;
    for i in 0..parts.len() {
        res += parts[i].to_code(nums[i]);
    }

    Ok(res)
}
```

##### 步骤3：操作数层

在操作数层的to_code中，我们采取的实现策略就有所不同。我们并没有显式进行异常处理，而是通过直接取模的方式的方式。

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

## 存在问题

在解析和转换中各自都需要以此验证，但验证目的不同：

* 解析部分：通过提前检查，降低报错的响应时间
* 转换部分：为了保证转换接口本身的鲁棒性



不采用check的方式，而是使用出错return的方式
