# github-change-name

リモートリポジトリ名前を変更する(HTTPS形式)


```zsh
github-change-name <old-name> <new-name>
```
```zsh
% git remote -v
origin\thttps://github.com/<old-name>/<repo-name>.git (fetch)
```
↓ 
```zsh
% git remote -v
origin\thttps://github.com/<new-name>/<repo-name>.git (fetch)
```
