# target

页面可以通过前端 yew 形式,也可以直接通过模板引擎(askama tera) 等。
目前是 yew 实现了业务逻辑。

返回给 `update_engine` 的请求是拼接的字符串。考虑生成 xml 模板写入，然后转成字符数组返回。
