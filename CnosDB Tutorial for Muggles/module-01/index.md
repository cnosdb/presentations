# 相关术语和CLI

## 摘要

- CnosQL vs traditional SQL

- 使用cnosdb-cli快速开始

- 索引

- 查询
  - SELECT statement
  - FROM, WHERE, INTO, 和 GROUP BY clauses
  - CnosQL function
  - Subqueies

### CnosQL vs traditional SQL

- 时间序列数据在聚合场景中最有用

- CnosDB 没有 JSON

- CnosDB 中的`measurement`类似于一个 SQL 中的`table`

- CnosDB 中的`tag`就像 SQL 中的一个带索引的列

- CnosDB 中的`field`就像 SQL 中的没有索引的列

- CnosDB`points`类似于 SQL 中的行

- CnosDB 中不需要预定义`schema`

### Schema 入门

#### TSM

TSM(Time structured Merge Tree)和 TSM 文件

TSM文件包含只读的已排序和以压缩压缩的时间序列数据，TSM存在于Shard中。

LSM(Log-Structured Merge)是许多现代数据库存储中使用的一项非常重要的技术，LSM tree 是一种数据结构，旨在为长时间内经历大量写入率的文件提供底层本的索引。

CnosDB存储引擎看起来像一个LSM tree，他有一个write head log和一组只读的数据文件，这些文件在概念上类似于LSM中的SSTables。

TSM是CnosDB专门构建的数据存储格式，与现有的B+或LSM相比，TSM允许更大的压缩率和更高的读写吞吐。

一个TSM文件由四个部分组成：header，blocks，index，footer。

```
+--------+------------------------------------+-------------+--------------+
| Header |               Blocks               |    Index    |    Footer    |
|5 bytes |              N bytes               |   N bytes   |   4 bytes    |
+--------+------------------------------------+-------------+--------------+
```
CnosDB会为每个时间块创建一个分片，分片包含实际的编码和压缩数据，并由磁盘上的TSM文件表示，每个分片都属于一个且只有一个分片组，每个分片组中可能存在多个分片，每个分片都包含一组特定的`series`，落在指定分片组和指定`series`上的所有`point`都将存储在磁盘上的同一个分片中。

#### WAL 文件
#### Cache

### Indexes

- TSI(Time series index)使用操作系统的页面缓存将热数据拉入内存，让冷数据留在磁盘上
- 内存映射
- 该索引通过简单的时间戳为用户提供大量信息

#### TSI结构

TSI由以下部分组成：
- index
- Partition
- LogFile
- IndexFile

#### 写入

#### 合并

#### 读取

#### 日志文件结构

#### Index文件结构

#### Manifest

### CnosQL和语法

#### 单引号

#### 特殊字符

#### 逗号

#### 等号

#### 空格

#### 双引号

### CnosDB CLI
todo 描述

#### 启动cnosdb-cli

### 探索架构

### 数据库管理

### 保留策略管理

### 使用CnosQL探索数据

### CnosQL函数

### FILL 函数

### 常用表达式

### Cast 操作

### 子查询

1. 什么是 TSM 文件？
2. WAL文件的用途是什么？
3. CnosDB 自动创建的保留策略是什么？
4. 您如何找到数据库的保留策略？
5. DELETE 和 DROP 有什么区别？
6. GROUP BY 子句有什么作用？

7. 在本地 vagrant 环境中，创建一个名为“mydb”的数据库。
8. 创建一个 1 小时的保留策略，复制为 1，并设置其为默认保留策略
9. 使用“telegraf”数据库，创建一个显示可用内存和开发环境中可用内存百分比的查询。
