# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:27:24
- **数据来源**: connectivity_results-20251229-172723.json
- **总测试数**: 495
- **失败测试数**: 2
- **成功测试数**: 493
- **失败率**: 0.40%
- **平均延迟**: 82.74ms
- **最小延迟**: 50ms
- **最大延迟**: 603ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:27:24
- **IP地址**: 2a09:bac1:76c1:5e78::272:a5
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 36.4766, -78.1847
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 35   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 171  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 153  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 46   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 51   | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 77   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 177  | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 203  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 300  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 424  | palera.in                               | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 219  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 88   | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 351  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 385  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 64   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 118  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 172  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 302  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 360  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 427  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 460  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 12   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 36   | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 43   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 52   | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 53   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 187  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 278  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 412  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 478  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 482  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 71   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 156  | freeyx.cloudflare88.eu.org              | 141.101.121.109                         | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 262  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 317  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 392  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 416  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 451  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 494  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 16   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 21   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 41   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 44   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 70   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 72   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 74   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 80   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 99   | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 129  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 147  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 196  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 217  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 220  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 233  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 235  | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 303  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 344  | bestcf.030101.xyz                       | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a    | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 415  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 456  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 475  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 22   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 56   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 73   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 91   | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 116  | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 131  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 253  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 284  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 297  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 304  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 335  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 364  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 405  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 446  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 457  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 459  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 480  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 10   | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 14   | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 37   | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 89   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 154  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 168  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 170  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 192  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 216  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 279  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 323  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 324  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 336  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 342  | bestcf.030101.xyz                       | 104.19.47.227                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 436  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 452  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 104  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 114  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 126  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 134  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 169  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 191  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 207  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 264  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 275  | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 292  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 346  | cf.zhetengsha.eu.org                    | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 353  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 15   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 20   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 28   | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 93   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 98   | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 100  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 105  | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 202  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 208  | eur.877774.xyz                          | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 273  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 294  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 296  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 298  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 313  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 328  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 330  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 374  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 381  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 387  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 397  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 404  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 430  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 447  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 462  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 469  | 456.cloudflare.182682.xyz               | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 476  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 5    | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 11   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 62   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 63   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 67   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 96   | cu.877774.xyz                           | 172.64.145.202                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 125  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 132  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 155  | freeyx.cloudflare88.eu.org              | 141.101.121.146                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 167  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 184  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 255  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 339  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 372  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 377  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 422  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 432  | benedict.ns.cloudflare.com              | 162.159.44.205                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 458  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 481  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 57   | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 94   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 111  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 122  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 152  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 163  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 164  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 175  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 183  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 205  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 209  | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 211  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 214  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 229  | cmcc.877774.xyz                         | 104.16.149.6                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 246  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 263  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 286  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 299  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 311  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 322  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 331  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 366  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 435  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 443  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 448  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 38   | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 84   | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 150  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 204  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 283  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 314  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 444  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 463  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 488  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 76   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 90   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 101  | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 135  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 186  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 256  | www.7749tv.com                          | 104.17.196.161                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 281  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 318  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 334  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 349  | cf.zhetengsha.eu.org                    | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 354  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 361  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 368  | kyree.ns.cloudflare.com                 | 162.159.44.207                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 382  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 394  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 200 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
