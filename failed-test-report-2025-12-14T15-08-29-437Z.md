# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 15:08:29
- **数据来源**: connectivity_results-20251214-150828.json
- **总测试数**: 447
- **失败测试数**: 168
- **成功测试数**: 279
- **失败率**: 37.58%
- **平均延迟**: 115.00ms
- **最小延迟**: 48ms
- **最大延迟**: 834ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 15:08:29
- **IP地址**: 52.150.28.32
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

- **网络不可达: 网络不可达**: 165 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (165 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 108  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 417  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 447  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 165 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 165 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 199  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 422  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 298  | 456.cloudflare.182682.xyz             | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 240  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 275  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 329  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 69   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 217  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 139  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 393  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 14   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 152  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 22   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 132  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 294  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 19   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 59   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 330  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 356  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 387  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 360  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 178  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 285  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 9    | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 10   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 11   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 16   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 42   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 305  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 412  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 120  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 234  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 291  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 443  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 111  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 287  | bestcf.030101.xyz                     | 104.17.211.46   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 80   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 204  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 371  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 407  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 117  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 212  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 222  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 251  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 265  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 306  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 350  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 424  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 136  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 214  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 418  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 421  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 27   | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 67   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 106  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 253  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 337  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 232  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 376  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 399  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 8    | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 357  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 414  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 77   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 103  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 112  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 173  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 434  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 31   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 37   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 88   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 247  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 268  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 425  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 13   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 28   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 442  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 32   | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 107  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 209  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 229  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 395  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 3    | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 46   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 66   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 75   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 85   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 130  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 210  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 366  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 419  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 184  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 361  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 428  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 295  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 7    | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 12   | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 86   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 116  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 20   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 35   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 133  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 324  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 401  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 95   | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 198  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 269  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 311  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 344  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 351  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 15   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 90   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 18   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 180  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 213  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 221  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 235  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 328  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 402  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 127  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 29   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 33   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 102  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 264  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 314  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 362  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 388  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 121  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 181  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 79   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 81   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 156  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 239  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 245  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 310  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 415  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 423  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 110  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 205  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 230  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 258  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 323  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 17   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 72   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 367  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 162  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 176  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 188  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 36   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 55   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 73   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 203  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 429  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 56   | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 369  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 30   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 146  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 182  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 286  | bestcf.030101.xyz                     | 104.17.212.134  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 315  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 331  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 416  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 26   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 259  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 343  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 151  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 363  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 378  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 400  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 405  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 76   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 194  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 299  | 456.cloudflare.182682.xyz             | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 24   | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 61   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 87   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 389  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 74   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 124  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 175  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 164  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 211  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 338  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 345  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 420  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 318  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 336  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 4    | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 6    | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 50   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 147  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 368  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 158  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 169  | freeyx.cloudflare88.eu.org            | 141.101.120.165 | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 354  | www.7749tv.com                        | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 177  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 276  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 21   | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 1    | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 25   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 118 条记录
- **正常 (100-200ms)**: 81 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 165 次

### 按协议统计

- **none**: 168 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
