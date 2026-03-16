# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:20:08
- **数据来源**: connectivity_results-20251229-172007.json
- **总测试数**: 494
- **失败测试数**: 181
- **成功测试数**: 313
- **失败率**: 36.64%
- **平均延迟**: 104.90ms
- **最小延迟**: 43ms
- **最大延迟**: 543ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:20:08
- **IP地址**: 64.236.137.134
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8835, -87.6305
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 178 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (178 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 487  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 492  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 494  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 462  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 123  | icook.hk                              | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 76   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 231  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 381  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 40   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 321  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 310  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 130  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 135  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 345  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 400  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 127  | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 262  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 353  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 383  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 180  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 260  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 394  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 390  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 461  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 27   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 248  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 261  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 357  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 456  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 100  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 214  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 445  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 9    | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 110  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 322  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 483  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 6    | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 21   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 338  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 84   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 490  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 101  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 170  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 269  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 369  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 376  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 79   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 128  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 283  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 417  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 431  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 43   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 93   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 218  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 446  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 351  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 361  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 450  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 427  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 371  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 491  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 247  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 276  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 334  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 360  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 401  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 432  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 471  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 243  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 265  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 326  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 459  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 39   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 339  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 421  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 437  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 467  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 19   | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 33   | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 136  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 24   | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 172  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 65   | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 242  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 488  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 35   | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 200  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 31   | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 169  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 168  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 194  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 199  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 178  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 384  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 16   | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 114  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 289  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 290  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 426  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 94   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 272  | xn--b6gac.eu.org                      | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 341  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 246  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 346  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 413  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 489  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 52   | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 298  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 347  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 449  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 473  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 12   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 181  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 280  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 468  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 477  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 11   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 115  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 237  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 457  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 113  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 177  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 320  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 403  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 15   | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 317  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 408  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 34   | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 41   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 106  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 122  | icook.hk                              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 186  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 198  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 377  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 25   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 232  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 249  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 7    | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 32   | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 111  | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 251  | freeyx.cloudflare88.eu.org            | 141.101.120.176 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 370  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 387  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 68   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 309  | cf.090227.xyz                         | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 476  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 120  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 176  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 414  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 475  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 92   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 279  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 38   | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 42   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 206  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 230  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 332  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 395  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 359  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 439  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 458  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 80   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 109  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 137  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 327  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 412  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 91   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 126  | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 185  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 365  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 470  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 112  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 163  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 182  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 415  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 435  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 460  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 36   | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 119  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 454  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 469  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 167  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 388  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 201  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 285  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 74   | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 75   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 7 条记录
- **快 (50-100ms)**: 152 条记录
- **正常 (100-200ms)**: 41 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 178 次

### 按协议统计

- **none**: 181 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
