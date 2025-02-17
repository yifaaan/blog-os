# 变量定义
TARGET = x86_64-blog_os
KERNEL = target/$(TARGET)/debug/bootimage-blog-os.bin
CARGO = cargo

# 默认目标
.PHONY: all
all: run

# 检查并安装必要的工具
.PHONY: check-tools
check-tools:
	@which cargo >/dev/null || (echo "请安装 Rust 工具链" && exit 1)
	@rustup component list | grep -q "llvm-tools-preview" || rustup component add llvm-tools-preview
	@which bootimage >/dev/null || cargo install bootimage

# 构建内核
.PHONY: build
build: check-tools
	$(CARGO) bootimage

# 运行 QEMU（适用于 WSL2）
.PHONY: run
run: build
	qemu-system-x86_64  -drive format=raw,file=$(KERNEL)

# 清理构建文件
.PHONY: clean
clean:
	$(CARGO) clean

# 帮助信息
.PHONY: help
help:
	@echo "可用的命令："
	@echo "  make          - 构建并运行内核"
	@echo "  make build    - 仅构建内核"
	@echo "  make run      - 运行已构建的内核"
	@echo "  make clean    - 清理构建文件"
	@echo "  make help     - 显示此帮助信息"