## 描述
+ 读取将数据库中的表结构，生成对应的实体类和相关的数据库增删改查操作
+ 通过rust宏来简化一些代码生成的重复工作
+ 支持mysql和sqlite

## 模块
* mysql_codegen 支持mysql属性宏库
* mysql_model  
mysql_gen.rs 用来读取mysql库，生成rust源代码
* sqlite_codegen 支持sqlite属性宏库
* sqlite_model  
sqlite_gen.rs 用来读取sqlite库，生成rust源代码
