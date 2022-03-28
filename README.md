# niao
a tool


## 配置文件

```yaml
git:
  - gitlab_niaoshuai:
      name: 任帅鹏
      email: renshuaipeng@jiaoyu361.com
      type: gitlab
      keyType: rsa
      keyPath: ~/.ssh/gitlab_niaoshuai_rsa
  - github_niaoshuai:
      name: niaoshuai
      email: niao.shuai123@163.com
      keyType: ed25519
      keyPath: ~/.ssh/github_niaoshuai_ed25519
```
## git 相关命令

```bash
# 初始化相关key （初始化完成之后 分别配置到第三方的平台配置sshkey）
$ niao git init  
# 校验配置是否正确
$ niao git validate 
# 切换key author等 （第一次拉取到项目执行）
$ niao git switch --key [KEY]
```
