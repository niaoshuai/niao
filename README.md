# niao
a tool


## 配置文件

```yaml
git:
  - gitlab_niaoshuai:
      name: [NAME]
      email: [EMAIL]
      type: gitlab
      keyType: rsa
      keyPath: ~/.ssh/[KEY_NAME]
```
## git 相关命令

```bash
# 初始化相关key （初始化完成之后 分别配置到第三方的平台配置sshkey）
$ niao git init  
# 校验配置是否正确
$ niao git validate 
# 切换key author等 （第一次拉取到项目执行）
$ niao git switch --key [KEY]
# 备份
$ niao git backup
# 恢复
$ niao git restore
```
