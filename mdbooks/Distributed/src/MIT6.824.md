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

分布式系统的背景介绍：
+ Why is hard
+ 性能<->分片
+ 故障/容错<->复制
+ 一致性<->性能

强一致性：表现为向一台服务器，
但是会带来不可避免的复杂性上升。


GFS设计目标：
+ 高速
+ 大容量
+ 容错
+ 大文件顺序访问优化
+ 允许一定的不一致性来换取一定的性能提升


GFS的两个表单：
+ 文件名到Chunk ID或者Chunk Handle数组的对应
+ Chunk ID到Chunk数据的对应关系
    + 每个Chunk存储在哪些服务器上，所以这部分是Chunk服务器的列表
    + 每个Chunk当前的版本号，所以Master节点必须记住每个Chunk对应的版本号。
    + 所有对于Chunk的写操作都必须在主Chunk（Primary Chunk）上顺序处理，主Chunk是Chunk的多个副本之一。所以，Master节点必须记住哪个Chunk服务器持有主Chunk。
    + 并且，主Chunk只能在特定的租约时间内担任主Chunk，所以，Master节点要记住主Chunk的租约过期时间。

这两个表单会被同时备份至硬盘。（Log，checkpoint）

Read:
+ name,offset->Master
+ master发送Chunk ID（Handle）和服务器列表
+ client选择服务器读取数据
+ chunk服务器返回数据

Write:
+ 找primary chunk（已经指定或数据最新者）
    + chunk的版本号到了多少是master存储，不是从chunk serve中获取最大值
+ primary负责实际的更新
+ client先向primary和从节点发送数据到临时区。然后收到所有成功后，向primary告知追加。primary检查后，追加并通知从追加。从成功后返回信息给primary，全部从成功后，回复客服端成功。

接受部分成功追加导致的read时的不一致。
primary的版本号只在重新指定Primary时改变。

不能发现primary挂了就指定新的，这受到租约限制，同时避免脑裂问题。

对于新文件。随机指定Chunk ID，版本号为1，primary+2secondary。

如果你想要将GFS升级成强一致系统，我可以为你列举一些你需要考虑的事情：
+ 你可能需要让Primary来探测重复的请求，这样第二个写入数据B的请求到达时，Primary就知道，我们之前看到过这个请求，可能执行了也可能没执行成功。Primay要尝试确保B不会在文件中出现两次。所以首先需要的是探测重复的能力。
+ 对于Secondary来说，如果Primay要求Secondary执行一个操作，Secondary必须要执行而不是只返回一个错误给Primary。对于一个严格一致的系统来说，是不允许Secondary忽略Primary的请求而没有任何补偿措施的。所以我认为，Secondary需要接受请求并执行它们。如果Secondary有一些永久性故障，例如磁盘被错误的拔出了，你需要有一种机制将Secondary从系统中移除，这样Primary可以与剩下的Secondary继续工作。但是GFS没有做到这一点，或者说至少没有做对。
+ 当Primary要求Secondary追加数据时，直到Primary确信所有的Secondary都能执行数据追加之前，Secondary必须小心不要将数据暴露给读请求。所以对于写请求，你或许需要多个阶段。在第一个阶段，Primary向Secondary发请求，要求其执行某个操作，并等待Secondary回复说能否完成该操作，这时Secondary并不实际执行操作。在第二个阶段，如果所有Secondary都回复说可以执行该操作，这时Primary才会说，好的，所有Secondary执行刚刚你们回复可以执行的那个操作。这是现实世界中很多强一致系统的工作方式，这被称为两阶段提交（Two-phase commit）。
+ 另一个问题是，当Primary崩溃时，可能有一组操作由Primary发送给Secondary，Primary在确认所有的Secondary收到了请求之前就崩溃了。当一个Primary崩溃了，一个Secondary会接任成为新的Primary，但是这时，新Primary和剩下的Secondary会在最后几个操作有分歧，因为部分副本并没有收到前一个Primary崩溃前发出的请求。所以，新的Primary上任时，需要显式的与Secondary进行同步，以确保操作历史的结尾是相同的。
+ 最后，时不时的，Secondary之间可能会有差异，或者客户端从Master节点获取的是稍微过时的Secondary。系统要么需要将所有的读请求都发送给Primary，因为只有Primary知道哪些操作实际发生了，要么对于Secondary需要一个租约系统，就像Primary一样，这样就知道Secondary在哪些时间可以合法的响应客户端。


缺点：
+ 单master，RAM有限，处理能力有限
+ master的故障需要人工