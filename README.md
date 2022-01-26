# target

页面可以通过前端 yew 形式,也可以直接通过模板引擎(askama tera) 等。
目前是 yew 实现了业务逻辑。

返回给 `update_engine` 的请求是拼接的字符串。考虑生成 xml 模板写入，然后转成字符数组返回。

## backend

todo 使用过程宏的方法， 构造初始化数据库函数。
todo tera 填充 update.xml 耗时约在 250 微秒 (不包含初始化， 二者初始化都是通过 `lazy_static!` 进行)
待测试 askama 的时间

两版代码数量对比
