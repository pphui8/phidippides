# phidippides
> backend of huihuiblog

### basic usage
> phidippides [option]
1. help      : this help
2. version   : phidippides version
3. filepath  : configuration file path



todo list
- [x] read config file
- [ ] open default page
- [ ] build the frame


---

#### ```GET``` / &emsp;&emsp;>> API test
> ##### expected return
```json
{
    "status": 200
}
```

#### ```GET``` / index &emsp;&emsp;>> get index of blog
> ##### expected return
```json
[
    {
        "id": 1,
        "name": "pphui8",
        "descript": "hello world",
        "tag": "tag",
    },
    ......
]
```

#### ```GET``` / blog / { id } &emsp;&emsp;>> get blog
> ##### expected return
```json
{
    "status": 200,
    "blog_root": "https://xx.xxx.xxx",
}
```

#### ```POST``` / addblog  &emsp;&emsp;>> add blog
> ##### request body format
```json
{
    "blog_name": "blog_name",
    "desc": "blog description",
    "value": "pphui8",
    "token": "xxxx",
    "tag": "tag"
}
```

> ##### expected return
```json
# success
{
    "status": "success"
}
# failed
{
    "status": "failed",
    "error": "failed to add blog"
}
```
> ```
> tag:
> 1.test  
> 2.note  
> 3.blog  
> 4.code  
> ```

#### ```GET``` / delblog / <blog_name> / <token>  &emsp;&emsp;>> delete blog
> ##### expected return
```json
// success
{
    "status": "success"
}
// failed
{
    "status": "failed",
    "error": "wrong token"
}GET
```

#### ```GET``` / comment &emsp;&emsp;>> get comment
> ##### expected return
```json
[
    {
        "id":1,
        "username":"pphui8",
        "url":"pphui8.com",
        "value":"test",
        "time": "xxxx/xx/xx",
    },
    .....
]
```

#### ```POST``` / addcomment  &emsp;&emsp;>> add comment
> ##### request body format
```json
{
    "username": "blog_name",
    "url": "blog description",
    "value": "pphui8",
    "token": "xxx",
    "time": "time",
}
```
> ##### expected return
```json
# success
{
    "status": "success"
}
# failed
{
    "status": "failed",
    "error": "failed to add blog"
}
```

#### ```GET``` / delcomment / <comment_id> / <token>  &emsp;&emsp;>> delete comment
> ##### expected return
```json
// success
{
    "status": "success"
}
// failed
{
    "status": "failed",
    "error": "wrong token"
}GET
```

#### ```GET``` / filing  &emsp;&emsp;>> get filing
> ##### expected return
```json
{ 
    "test":0,
    "note":0,
    "blog":0,
    "code":0,
}
```
