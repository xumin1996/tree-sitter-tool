# 交叉编译wasm32-wasip1
`wasm32-wasip1`是最新标准，并且是配型较强。
交叉编译时需要安装clang llvm wasi-libc
```bash
sudo pacman -S llvm clang wasi-libc
```
并且设置`WASI_SYSROOT`用于找到stdio.h等头
```bash
export WASI_SYSROOT="/usr/share/wasi-sysroot"   
```
