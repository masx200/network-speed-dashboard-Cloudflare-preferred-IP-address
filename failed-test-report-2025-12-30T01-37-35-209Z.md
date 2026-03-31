# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:37:35
- **数据来源**: connectivity_results-20251230-013734.json
- **总测试数**: 504
- **失败测试数**: 182
- **成功测试数**: 322
- **失败率**: 36.11%
- **平均延迟**: 121.34ms
- **最小延迟**: 46ms
- **最大延迟**: 647ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:37:35
- **IP地址**: 172.172.87.83
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 178 次 (97.8%)
- **连接超时: I/O超时**: 4 次 (2.2%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (178 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 488  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 489  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 492  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 497  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (97.8%)
- **连接超时**: 4 次 (2.2%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 182 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 322 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 382  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 276  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 239  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 485  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 448  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 350  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 360  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 434  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 477  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 504  | cfip.xxxxxxxx.tk                      | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 145  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 315  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 203  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 314  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 351  | 456.cloudflare.182682.xyz             | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 473  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 423  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 140  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 104  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 136  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 296  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 361  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 464  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 213  | cloudflare-ip.mofashi.ltd             | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 501  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 36   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 320  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 476  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 229  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 436  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 453  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 47   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 456  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 502  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 97   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 197  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 308  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 39   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 57   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 86   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 260  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 478  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 90   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 222  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 311  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 415  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 93   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 94   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 129  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 131  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 265  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 471  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 188  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 432  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 83   | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 124  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 356  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 414  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 383  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 55   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 68   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 240  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 372  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 25   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 63   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 162  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 440  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 88   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 96   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 159  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 284  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 326  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 496  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 48   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 54   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 110  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 313  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 331  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 407  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 458  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 494  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 15   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 26   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 180  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 221  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 479  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 324  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 75   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 170  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 248  | cf.zhetengsha.eu.org                  | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 310  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 377  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 425  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 499  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 95   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 105  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 318  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 449  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 457  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 10   | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 127  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 192  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 208  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 219  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 400  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 101  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 102  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 139  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 160  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 186  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 207  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 323  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 227  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 256  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 317  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 474  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 9    | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 89   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 218  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 228  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 257  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 297  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 368  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 373  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 405  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 13   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 69   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 70   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 182  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 126  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 230  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 253  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 277  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 357  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 20   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 64   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 346  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 376  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 387  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 421  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 40   | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 43   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 281  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 282  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 363  | freeyx.cloudflare88.eu.org            | 141.101.121.205 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 452  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 44   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 67   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 99   | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 307  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 352  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 475  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 235  | bestcf.030101.xyz                     | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 386  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 399  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 460  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 490  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 56   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 98   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 138  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 193  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 325  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 412  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 133  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 196  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 303  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 349  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 52   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 53   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 155  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 223  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 330  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 417  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 482  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 35   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 181  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 195  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 268  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 294  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 306  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 332  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 404  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 410  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 6    | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 31   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 117  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 164  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 309  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 336  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 424  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 500  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 217  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 243  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 288  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 380  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 135  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 249  | cf.zhetengsha.eu.org                  | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 290  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 316  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 388  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 393  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 71   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 91   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 103  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 113  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 51   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 100  | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 236  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 362  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 163  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 174  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 369  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 392  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 430  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 459  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 112  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 132  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 384  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 472  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 493  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 341  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 484  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 111  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 134  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 264  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 431  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 441  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 125  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 156  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 483  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 498  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 32   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 118  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 123  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 168  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 319  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 503  | cfip.xxxxxxxx.tk                      | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 214  | cloudflare-ip.mofashi.ltd             | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 394  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 398  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 298  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 252  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 259  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 295  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 435  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 5    | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 161  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 187  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 299  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 406  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 202  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 442  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 447  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 169  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 322  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 79   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 137  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 194  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 301  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 495  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 82   | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 261  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 312  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 429  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 481  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 270  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 14   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 175  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 179  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 342  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 178  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 76   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 244  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 2    | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 209  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 269  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 3    | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 340  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 119  | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 411  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 300  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 491  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 242  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 289  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 128  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 81   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 321  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 275  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 347  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 375  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 21   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 92   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 201  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 87   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 130  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 180      | cloudflare |
| 241  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 182      | cloudflare |
| 4    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 416  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 283  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 205      | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 209      | cloudflare |
| 337  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 225      | cloudflare |
| 374  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 308      | cloudflare |
| 302  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 325      | cloudflare |
| 486  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 400      | cloudflare |
| 443  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 520      | cloudflare |
| 487  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 532      | cloudflare |
| 233  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 545      | cloudflare |
| 274  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 565      | cloudflare |
| 27   | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 647      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 89 条记录
- **正常 (100-200ms)**: 221 条记录
- **慢 (200-500ms)**: 6 条记录
- **很慢 (>500ms)**: 5 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 178 次

### 按协议统计

- **none**: 182 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
