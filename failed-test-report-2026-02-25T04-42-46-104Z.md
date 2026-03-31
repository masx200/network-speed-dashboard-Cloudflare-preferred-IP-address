# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 04:42:46
- **数据来源**: connectivity_results-20260225-044245.json
- **总测试数**: 863
- **失败测试数**: 243
- **成功测试数**: 620
- **失败率**: 28.16%
- **平均延迟**: 77.81ms
- **最小延迟**: 31ms
- **最大延迟**: 5612ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 04:42:46
- **IP地址**: 20.62.207.246
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 38.7095, -78.1539
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 239 次 (98.4%)
- **连接超时: I/O超时**: 4 次 (1.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (239 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 847  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 848  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 852  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 862  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 239 次 (98.4%)
- **连接超时**: 4 次 (1.6%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 243 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 239 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
iplocation.io (3次), huxley.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 620 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 725  | 104.26.1.88                           | 104.26.1.88     | IPv4   | h3   | ✅ 成功 | 31       | cloudflare |
| 484  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 32       | cloudflare |
| 616  | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 32       | cloudflare |
| 568  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 674  | 172.64.32.77                          | 172.64.32.77    | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 773  | 162.159.0.41                          | 162.159.0.41    | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 178  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 423  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 506  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 550  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 626  | 104.18.40.216                         | 104.18.40.216   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 639  | 104.17.215.66                         | 104.17.215.66   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 682  | 172.67.67.152                         | 172.67.67.152   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 722  | 162.159.36.205                        | 162.159.36.205  | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 47   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 86   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 498  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 295  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 373  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 455  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 491  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 492  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 494  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 628  | 104.20.21.147                         | 104.20.21.147   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 649  | 162.159.21.222                        | 162.159.21.222  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 672  | 104.18.37.177                         | 104.18.37.177   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 802  | 162.159.43.50                         | 162.159.43.50   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 392  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 496  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 567  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 603  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 713  | 104.18.42.106                         | 104.18.42.106   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 714  | 104.18.40.202                         | 104.18.40.202   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 730  | 104.26.0.210                          | 104.26.0.210    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 789  | 104.25.244.36                         | 104.25.244.36   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 185  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 311  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 588  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 739  | 104.17.193.113                        | 104.17.193.113  | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 749  | 162.159.58.17                         | 162.159.58.17   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 750  | 172.64.53.202                         | 172.64.53.202   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 776  | 104.20.31.132                         | 104.20.31.132   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 777  | 172.67.70.56                          | 172.67.70.56    | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 861  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 461  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 495  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 502  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 630  | 104.26.1.181                          | 104.26.1.181    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 699  | 162.159.36.52                         | 162.159.36.52   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 775  | 162.159.20.46                         | 162.159.20.46   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 788  | 104.25.247.78                         | 104.25.247.78   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 791  | 104.25.241.85                         | 104.25.241.85   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 78   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 130  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 465  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 751  | 162.159.1.39                          | 162.159.1.39    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 779  | 104.20.22.91                          | 104.20.22.91    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 154  | freeyx.cloudflare88.eu.org            | 141.101.121.66  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 214  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 268  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 389  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 429  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 543  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 584  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 648  | 162.159.42.146                        | 162.159.42.146  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 245  | cf.zhetengsha.eu.org                  | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 299  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 355  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 395  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 686  | 104.19.144.159                        | 104.19.144.159  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 734  | 104.20.20.192                         | 104.20.20.192   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 818  | 104.26.5.53                           | 104.26.5.53     | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 246  | cf.zhetengsha.eu.org                  | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 404  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 453  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 768  | 172.64.52.224                         | 172.64.52.224   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 824  | 172.67.77.185                         | 172.67.77.185   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 853  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 77   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 286  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 347  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 490  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 687  | 104.17.53.129                         | 104.17.53.129   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 782  | 172.67.72.212                         | 172.67.72.212   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 31   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 153  | freeyx.cloudflare88.eu.org            | 141.101.120.232 | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 165  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 203  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 374  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 393  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 397  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 401  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 414  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 679  | 104.20.25.161                         | 104.20.25.161   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 692  | 104.16.251.254                        | 104.16.251.254  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 716  | 172.64.53.165                         | 172.64.53.165   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 126  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 158  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 197  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 376  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 458  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 459  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 462  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 493  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 537  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 545  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 618  | 162.159.18.240                        | 162.159.18.240  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 728  | 104.20.28.239                         | 104.20.28.239   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 784  | 104.20.19.180                         | 104.20.19.180   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 857  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 129  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 266  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 344  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 382  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 504  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 625  | 104.18.42.61                          | 104.18.42.61    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 636  | 104.17.170.137                        | 104.17.170.137  | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 696  | 104.17.187.186                        | 104.17.187.186  | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 737  | 104.25.241.198                        | 104.25.241.198  | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 816  | 104.20.24.239                         | 104.20.24.239   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 27   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 218  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 281  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 289  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 442  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 443  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 519  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 585  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 736  | 104.17.97.228                         | 104.17.97.228   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 786  | 104.25.245.215                        | 104.25.245.215  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 820  | 172.67.75.212                         | 172.67.75.212   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 825  | 104.25.244.87                         | 104.25.244.87   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 841  | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 18   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 65   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 102  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 147  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 229  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 320  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 361  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 522  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 540  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 546  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 742  | 104.17.111.150                        | 104.17.111.150  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 743  | 104.18.166.232                        | 104.18.166.232  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 854  | cfip.xxxxxxxx.tk                      | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 41   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 70   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 111  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 145  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 251  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 263  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 276  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 325  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 450  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 507  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 566  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 644  | 104.17.189.30                         | 104.17.189.30   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 738  | 104.25.244.239                        | 104.25.244.239  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 741  | 104.18.172.20                         | 104.18.172.20   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 81   | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 82   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 84   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 88   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 103  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 105  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 124  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 157  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 342  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 409  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 471  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 508  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 517  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 539  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 552  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 582  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 619  | 172.64.145.119                        | 172.64.145.119  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 650  | 172.64.53.40                          | 172.64.53.40    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 697  | 104.26.3.117                          | 104.26.3.117    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 797  | 172.64.153.183                        | 172.64.153.183  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 801  | 172.64.151.253                        | 172.64.151.253  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 817  | 172.67.79.249                         | 172.67.79.249   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 823  | 172.67.65.81                          | 172.67.65.81    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 828  | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 838  | 104.16.245.121                        | 104.16.245.121  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 106  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 121  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 123  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 269  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 523  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 542  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 549  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 551  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 571  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 573  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 576  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 612  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 665  | 172.64.154.113                        | 172.64.154.113  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 691  | 104.17.25.87                          | 104.17.25.87    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 729  | 104.26.14.117                         | 104.26.14.117   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 46   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 69   | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 127  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 189  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 207  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 213  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 237  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 267  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 515  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 536  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 559  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 592  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 629  | 104.26.2.2                            | 104.26.2.2      | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 684  | 104.20.17.51                          | 104.20.17.51    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 9    | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 53   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 68   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 76   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 79   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 140  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 309  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 448  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 541  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 578  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 579  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 583  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 621  | 162.159.1.111                         | 162.159.1.111   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 623  | 172.64.153.141                        | 172.64.153.141  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 647  | 172.64.52.67                          | 172.64.52.67    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 669  | 172.64.152.215                        | 172.64.152.215  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 670  | 162.159.36.223                        | 162.159.36.223  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 676  | 104.26.11.160                         | 104.26.11.160   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 683  | 172.67.79.150                         | 172.67.79.150   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 695  | 104.17.214.136                        | 104.17.214.136  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 723  | 104.18.42.16                          | 104.18.42.16    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 855  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 66   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 97   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 204  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 316  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 343  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 362  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 563  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 698  | 162.159.1.145                         | 162.159.1.145   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 731  | 104.26.6.247                          | 104.26.6.247    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 770  | 108.162.198.198                       | 108.162.198.198 | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 780  | 104.20.24.244                         | 104.20.24.244   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 90   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 206  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 209  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 331  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 466  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 481  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 486  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 521  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 558  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 746  | 162.159.11.128                        | 162.159.11.128  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 772  | 172.64.229.156                        | 172.64.229.156  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 815  | 172.64.229.15                         | 172.64.229.15   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 835  | 104.25.240.227                        | 104.25.240.227  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 24   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 35   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 83   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 160  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 194  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 297  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 305  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 408  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 431  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 487  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 501  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 556  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 561  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 849  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 32   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 56   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 75   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 134  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 250  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 333  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 357  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 372  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 415  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 440  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 554  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 799  | 172.64.144.132                        | 172.64.144.132  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 860  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 235  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 464  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 548  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 560  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 581  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 617  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 701  | 104.18.47.46                          | 104.18.47.46    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 735  | 104.26.8.171                          | 104.26.8.171    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 796  | 104.25.250.205                        | 104.25.250.205  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 13   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 95   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 226  | bestcf.030101.xyz                     | 104.17.206.188  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 304  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 367  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 452  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 499  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 565  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 570  | 104.18.36.1                           | 104.18.36.1     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 607  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 675  | 104.18.41.101                         | 104.18.41.101   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 744  | 104.25.245.233                        | 104.25.245.233  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 747  | 162.159.3.128                         | 162.159.3.128   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 766  | 172.64.53.41                          | 172.64.53.41    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 827  | 104.17.56.177                         | 104.17.56.177   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 858  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 67   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 146  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 279  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 547  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 666  | 162.159.6.44                          | 162.159.6.44    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 685  | 104.26.4.4                            | 104.26.4.4      | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 778  | 172.67.67.0                           | 172.67.67.0     | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 792  | 104.25.240.123                        | 104.25.240.123  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 822  | 104.20.26.221                         | 104.20.26.221   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 850  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 26   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 30   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 36   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 177  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 190  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 418  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 463  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 513  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 631  | 104.26.3.120                          | 104.26.3.120    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 642  | 104.19.153.47                         | 104.19.153.47   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 651  | 108.162.198.148                       | 108.162.198.148 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 732  | 104.20.25.181                         | 104.20.25.181   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 763  | 198.41.222.191                        | 198.41.222.191  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 783  | 104.26.4.135                          | 104.26.4.135    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 793  | 104.25.254.14                         | 104.25.254.14   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 843  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 1    | 104.18.32.174                         | 104.18.32.174   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 63   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 71   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 161  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 219  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 264  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 298  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 321  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 417  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 433  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 447  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 451  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 509  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 640  | 104.19.34.231                         | 104.19.34.231   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 671  | 172.64.41.216                         | 172.64.41.216   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 690  | 104.17.110.226                        | 104.17.110.226  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 72   | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 128  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 241  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 354  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 445  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 472  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 681  | 172.67.78.67                          | 172.67.78.67    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 715  | 172.64.154.226                        | 172.64.154.226  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 40   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 55   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 172  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 221  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 277  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 341  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 428  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 454  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 589  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 615  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 781  | 104.26.11.33                          | 104.26.11.33    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 859  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 863  | cfip.xxxxxxxx.tk                      | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 10   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 144  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 199  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 350  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 432  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 444  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 449  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 505  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 544  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 575  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 798  | 162.159.33.191                        | 162.159.33.191  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 821  | 172.67.73.196                         | 172.67.73.196   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 485  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 503  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 574  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 646  | 104.17.25.241                         | 104.17.25.241   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 837  | 104.25.242.249                        | 104.25.242.249  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 856  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 5    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 64   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 231  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 337  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 422  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 569  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 98   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 115  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 148  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 457  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 315  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 446  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 733  | 172.67.65.150                         | 172.67.65.150   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 308  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 468  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 470  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 694  | 104.18.44.25                          | 104.18.44.25    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 727  | 172.64.229.134                        | 172.64.229.134  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 89   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 371  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 620  | 162.159.3.89                          | 162.159.3.89    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 22   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 92   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 205  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 217  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 512  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 577  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 678  | 104.26.1.194                          | 104.26.1.194    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 57   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 370  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 624  | 162.159.22.29                         | 162.159.22.29   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 627  | 172.64.146.67                         | 172.64.146.67   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 51   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 93   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 430  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 555  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 14   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 39   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 91   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 303  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 326  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 339  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 388  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 469  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 510  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 643  | 104.17.209.79                         | 104.17.209.79   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 45   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 135  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 138  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 380  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 611  | 172.67.75.11                          | 172.67.75.11    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 829  | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 23   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 28   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 80   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 85   | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 489  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 587  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 635  | 172.67.72.36                          | 172.67.72.36    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 673  | 172.64.145.108                        | 172.64.145.108  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 109  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 242  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 366  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 427  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 511  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 557  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 622  | 172.64.42.158                         | 172.64.42.158   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 700  | 172.64.40.196                         | 172.64.40.196   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 720  | 172.64.52.189                         | 172.64.52.189   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 785  | 104.26.5.194                          | 104.26.5.194    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 839  | 104.17.143.82                         | 104.17.143.82   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 840  | 104.18.160.38                         | 104.18.160.38   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 42   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 73   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 87   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 256  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 261  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 435  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 680  | 104.26.6.171                          | 104.26.6.171    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 693  | 104.16.155.230                        | 104.16.155.230  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 752  | 162.159.12.120                        | 162.159.12.120  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 29   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 125  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 319  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 402  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 500  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 717  | 108.162.198.85                        | 108.162.198.85  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 94   | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 232  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 249  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 278  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 613  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 689  | 104.17.153.151                        | 104.17.153.151  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 740  | 104.26.13.110                         | 104.26.13.110   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 745  | 172.64.153.140                        | 172.64.153.140  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 836  | 104.25.246.24                         | 104.25.246.24   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 173  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 403  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 488  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 25   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 271  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 473  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 688  | 104.26.7.7                            | 104.26.7.7      | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 851  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 257  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 410  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 553  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 634  | 104.20.25.82                          | 104.20.25.82    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 677  | 162.159.16.136                        | 162.159.16.136  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 748  | 104.25.250.174                        | 104.25.250.174  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 2    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 272  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 310  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 439  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 534  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 580  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 790  | 104.17.56.208                         | 104.17.56.208   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 122  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 637  | 104.26.10.239                         | 104.26.10.239   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 54   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 291  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 424  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 514  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 632  | 104.26.5.101                          | 104.26.5.101    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 265  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 467  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 645  | 104.19.35.242                         | 104.19.35.242   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 664  | 162.159.61.106                        | 162.159.61.106  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 787  | 104.18.148.235                        | 104.18.148.235  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 795  | 104.25.249.225                        | 104.25.249.225  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 74   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 210  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 572  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 591  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 614  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 774  | 162.159.44.202                        | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 834  | 104.25.254.89                         | 104.25.254.89   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 285  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 117  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 633  | 172.67.73.120                         | 172.67.73.120   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 17   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 166  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 460  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 564  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 586  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 638  | 104.17.211.218                        | 104.17.211.218  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 771  | 162.159.45.67                         | 162.159.45.67   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 814  | 172.64.52.181                         | 172.64.52.181   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 133  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 171  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 236  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 441  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 497  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 641  | 104.17.115.224                        | 104.17.115.224  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 819  | 162.159.38.83                         | 162.159.38.83   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 724  | 162.159.39.146                        | 162.159.39.146  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 167  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 349  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 520  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 456  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 605  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 813  | 162.159.45.150                        | 162.159.45.150  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 826  | 104.20.22.141                         | 104.20.22.141   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 184  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 290  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 360  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 416  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 535  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 800  | 162.159.39.74                         | 162.159.39.74   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 6    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 225  | bestcf.030101.xyz                     | 104.19.156.188  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 421  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 767  | 108.162.198.70                        | 108.162.198.70  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 193  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 270  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 280  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 356  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 769  | 162.159.38.45                         | 162.159.38.45   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 262  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 610  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 652  | 162.159.44.133                        | 162.159.44.133  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 406  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 606  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 220  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 375  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 719  | 162.159.38.226                        | 162.159.38.226  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 116  | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 149  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 663  | 162.159.38.67                         | 162.159.38.67   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 332  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 538  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 3    | 104.17.62.194                         | 104.17.62.194   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 139  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 327  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 396  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 604  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 516  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 765  | 162.159.39.180                        | 162.159.39.180  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 381  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 609  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 518  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 842  | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 608  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 712  | 162.159.39.20                         | 162.159.39.20   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 726  | 162.159.44.101                        | 162.159.44.101  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 255  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 764  | 162.159.38.134                        | 162.159.38.134  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 4    | 172.64.49.146                         | 172.64.49.146   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 21   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 110  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 198  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 96   | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 590  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 830  | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 721  | 162.159.45.65                         | 162.159.45.65   | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 179  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 434  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 383  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 668  | 162.159.45.165                        | 162.159.45.165  | IPv4   | h3   | ✅ 成功 | 159      | cloudflare |
| 183  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 667  | 162.159.39.136                        | 162.159.39.136  | IPv4   | h3   | ✅ 成功 | 196      | cloudflare |
| 159  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 562  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 376      | cloudflare |
| 296  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 505      | cloudflare |
| 353  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 518      | cloudflare |
| 794  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 714      | cloudflare |
| 718  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 5612     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 146 条记录
- **快 (50-100ms)**: 410 条记录
- **正常 (100-200ms)**: 59 条记录
- **慢 (200-500ms)**: 1 条记录
- **很慢 (>500ms)**: 4 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 239 次

### 按协议统计

- **none**: 243 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
