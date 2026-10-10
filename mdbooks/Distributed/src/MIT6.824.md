# Introduction

分布式系统的核心是通过网络来协调，共同完成一致任务的一些计算机。

能不分布式，就不分布式。

使用分布式的理由：
+ 并行
+ 故障容忍
+ 地理、物理限制
+ 安全/隔离

挑战：
+ 并行化
+ 组件的奇怪故障
+ 性能需取舍

 
基础架构的类型主要是存储，通信（网络）和计算。

设计的分布式系统尽可能做到对调用方方便。

抽象主题：
+ 实现
    + RPC
    + 进程/线程
    + 锁机制
+ 性能
    + 可扩展性（Scalability）
+ 容错
    + 可用性（Availability）
    + 可恢复性（Recoverability）
    + 非易失存储
    + 复制（replication）
+ 一致性（consistency）
    + 定义不是特别统一
    + 性能和一致性之前的权衡，一致性越强性能越差

## MapReduce

让程序员编写Map和Reduce函数，剩下的由整个分布式系统处理。

整个MapReduce计算被称为Job。
每一次对map或Reduce的一次调用被称为一个Task。

MapReduce可以前后拼接协同工作。
但是后来的Spark优化了流程协同。

2004年时，网线的网速限制了系统，2020年，交换机限制了系统。

# RPC & Thread

使用go作为课程语言，所需工具便于调用，不必手动管理内存，简单易用。

线程用途：
+ I/O并发
+ 并行计算
+ 后台任务，定时任务等功能的实现

异步编程/事件驱动风格：
可以参考I/O多路复用select/poll/epoll的知识

异步稍微编程麻烦。

协调（coordination）：
+ channal
+ sync.Cond
+ wait Group

死锁（Deadlock）

竞争问题的解决一般依赖于分析工具。
go过于简单，有时会出现不必要的麻烦。

# GFS
