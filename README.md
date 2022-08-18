### 将 fn(1 | 2 | 3, xxx) 转换为对函数的多次调用。

例如：

```rust
saint_he!(powerCon(1 | 2 | 6 | 7 | 11 | 52 | 57 | 58 | 65, 10))
```

会被转化为：

```rust
    {
        let _ = powerCon(1, 10);
        let _ = powerCon(2, 10);
        let _ = powerCon(6, 10);
        let _ = powerCon(7, 10);
        let _ = powerCon(11, 10);
        let _ = powerCon(52, 10);
        let _ = powerCon(57, 10);
        let _ = powerCon(58, 10);
        let _ = powerCon(65, 10);
    }
```