# 指令文档

本程序在压缩包中也提供了对应的指令信息文件，也存储在了仓库的`data/test/cmds.json`中。包括的具体的指令类型如下。

## Register: 寄存器型

指令结构：OP(6) + rs(5) + rt(5) + rd(5) +shamt(5) + funct(6)

* 采用扩展操作码形式（OP字段值为0，具体指令功能由6位`funct`字段决定）

* `rs`、 `rt`、 `rd`为寄存器操作数字段，可访问32个通用寄存器
* `shamt`字段是移位变量，只用于移位指令，其他指令无效

| funct | 指令           | 功能描述                                                  |
| ----- | -------------- | --------------------------------------------------------- |
| 12    | `syscall`      | 系统调用，这里实现停机指令                                |
| 32    | `add rd rs rt` | 加法指令：R[rd] <- R[rs] + R[rt]                          |
| 42    | `slt rd rs rt` | 小于置位指令，有符号比较：R[rd] <- (R[rs])<R[rt]) ? 1 : 0 |

## Immediate: 立即数型

指令结构：OP(6) + rs(5) + rt(5) + Imm(16)

* `rs`、` rt`为寄存器操作数字段，可访问32个通用寄存器
* `Imm`字段为16位有符号立即数字段

| OP   | 指令              | 功能描述                                                     |
| ---- | ----------------- | ------------------------------------------------------------ |
| 04   | `beq rs rt imm `  | 相等跳转：if(R[rs] == R[rt]) PC ← PC + 4 + SignExt18b(imm<<2)； |
| 05   | `bne rs rt imm `  | 不等跳转：if(R[rs] != R[rt]) PC ← PC + 4 + SignExt18b(imm<<2)； |
| 08   | `addi rt rs imm ` | 立即数加：R[rt] ← R[rs]+SignExt16b(imm) ；                   |
| 35   | `lw rt imm rs `   | 取数指令：R[rt] ← Mem4B(R[rs] + SignExt16b(imm)) ；          |
| 43   | `sw rt imm rs) `  | 存数指令：Mem4B(R[rs]+SignExt16b(imm)) ← R[rt]；             |

