# gglog
greengrassのログをいい感じに表示してくれるプログラム．
直近更新されたログファイルを閲覧できるなどなど．

## Install
Releaseページから最新のバイナリをダウンロードし，実行権限を付与して`/usr/local/bin/`に配置．
```bash
wget https://github.com/tskawada/gglog/releases/download/v0.01/gglog
chmod 755 gglog
sudo mv gglog /usr/local/bin/
```

## Usage
sudo権限が必要
```bash
sudo gglog [OPTIONS]
```
### Options
- `-c`  display a log file which updated latest
- `-h`  print help information
- `-l`  display a list of log files
- `-V`  print version information

