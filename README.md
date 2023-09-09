# MIPS编译器

本程序需要与Logisim电路设计软件结合使用，其本主要功能就是将32位MIPS汇编指令转换成能够被Logisim内存识别的机器代码。

## 使用方法

下载本程序的压缩包后，将其解压到任意位置即可。程序主要的文件由

* `mipsc.exe`: 程序主体
* `config.json`: 管理程序的一些关键配置信息
* `cmds.json`: 存储本程序支持的指令信息

### 运行程序

本程序提供的压缩包里面已经提供好了一些默认的配置信息和指令信息。同时，为了方便在任意文件夹内都可以使用本程序，请在系统设置中配置好环境变量。

> 配置系统环境变量：[教程](https://pengpengyang94.github.io/2020/05/win10%E8%AE%BE%E7%BD%AE%E7%8E%AF%E5%A2%83%E5%8F%98%E9%87%8F%E5%9B%BE%E6%96%87%E6%95%99%E7%A8%8B/)

再配置好环境变量后，本程序的使用方法类似于c语言的编译gcc。

```bat
mipsc source -o output -f format
```

* `source`: 需要编译的源代码文件名，本程序建议使用 `.mips`结尾

* `output`: 机器指令输出文件名

  不输入时默认为源代码文件名+`.dat`后缀

* `format`: 输出格式[`logisim/plain`]

  logisim则按照该软件规定的读入格式输出，详情请见[此处](./docs/logisim_output_format.md)

  plain则以16进制明文格式输出

### 配置文件

本程序使用Json文件管理配置信息，而默认的配置信息包括一下字段。

```json
{
    "language": "CN",
    "cmds_path": "cmds.json",
    "default_save_format": "Logisim",
    "default_save_extension": "dat"
}
```

* `language`: 程序语言 [`CN/EN` ]（尚未时装）

* `cmds_path`: 存储指令信息文件的路径，可以使用相对路径和绝对路径

  注意：相对路径是以exe程序所在目录计算的

* `default_save_format`: 默认保存格式[`Logisim/Plain`]

* `default_save_extension`: 默认保存后缀（在填写时注意文件名不要冲突）

### 指令信息

本程序使用Json文件管理配置信息，且目前仅支持MIPS架构中的I型指令和R型指令。本文件存储路径需要由上述配置文件指定。

```json
[
    {
        "type": "R",
        "name": "syscall",
        "funct": 12,
        "operands": []
    },
        {
        "type": "I",
        "name": "beq",
        "op": 4,
        "operands": ["RS", "RT", "IMM"]
    },
]
```

* `type`: 指令类型[`R/I`]

* `operands`: 操作数类型和顺序，[`RS/RT/RD/SHAMT/IMM`]

  注意：目前本程序还没有实现指令信息文件的校验，请在编写本文件的时候注意各类型指令支持的操作数。

## 关于开发

本程序使用Rust语言，如果你对本程序感兴趣，笔者也提供了相关的[开发文档](./docs/README.md)。

> 开发文档还在完善当中...

## 其他

本程序主要用于教学使用，如果你发现了bug或者有改进的地方，可以提交PR、提出issue、或者联系以下邮箱：

> huch37@mail2.sysu.edu.cn