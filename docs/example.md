## 1. 问题

考虑问题：逐个比较两个长度相同的数组元素是否相同

> (当长度不同时，认为是异常情况，而不是不相同)

## 2. 解决方案


当然，实际输入时可能会输入两个长度并不相等的数组。为了保证函数的鲁棒性，我们需要对异常情况进行处理。下面我们编写了两个函数；

1. 在开始比较前，先验证两个数组长度是否相等，如果不相等则抛出异常

2. 直接进行比较，如果出现数组越界，则说明两个数组长度不相等
此时再抛出异常

```python
LengthError = ValueError("The length of arr1 and arr2 are not equal")

# 方法1
def compare1(arr1: list[int], arr2: list[int]) -> bool:
    if len(arr1) != len(arr2):
        raise LengthError

    for i in range(len(arr1)):
        if arr1[i] != arr2[i]:
            return False
    
    return True

# 方法2
def compare2(arr1: list[int], arr2: list[int]) -> bool:
    for i in range(len(arr1)):
        try:
            if arr1[i] != arr2[i]:
                return False
        except IndexError:
            raise LengthError
    
    return True
```

## 3. 疑惑

两种方法比较：

1. 方法1能够快速响应错误，但是需要额外一次check。
2.  方法2不需要额外check，但是需要等待真正报错时才会返回错误。

> 当然，方法2只有当len(arr1)>len(arr2)的情况才会抛出异常。存在缺陷，但是暂时不考虑这个问题

疑问：如果采取方法2，如果能一开始就能检测出异常，何必要等到真正出错的时候才抛出异常。如果采取方法1，需要额外进行一次条件验证，影响性能？