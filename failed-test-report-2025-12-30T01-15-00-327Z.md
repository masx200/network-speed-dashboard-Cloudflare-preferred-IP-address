# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:15:00
- **数据来源**: connectivity_results-20251230-011459.json
- **总测试数**: 507
- **失败测试数**: 2
- **成功测试数**: 505
- **失败率**: 0.39%
- **平均延迟**: 80.18ms
- **最小延迟**: 45ms
- **最大延迟**: 1092ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:15:00
- **IP地址**: 2a09:bac1:76e0:c8::e:2c7
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8874, -87.6318
- **时区**: America/Chicago
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
| 27   | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 268  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1
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
| 273  | cfip.xxxxxxxx.tk                        | 188.114.97.144                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 150  | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 241  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 384  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 471  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 39   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 240  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 253  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 456  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 14   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 68   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 79   | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 115  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 141  | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 161  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 274  | cfip.xxxxxxxx.tk                        | 104.18.228.35                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 343  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 437  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 485  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 13   | www.ipget.net                           | 104.21.15.212                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 23   | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 53   | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 60   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 86   | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 91   | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 109  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 116  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 133  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 148  | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 159  | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 182  | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 249  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 354  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 358  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 369  | cf.zhetengsha.eu.org                    | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 383  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 462  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 494  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 26   | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 30   | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 89   | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 127  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 134  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 136  | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 137  | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 201  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 279  | cfip.xxxxxxxx.tk                        | 104.21.91.19                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 330  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 390  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 394  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 403  | braden.ns.cloudflare.com                | 172.64.35.169                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 412  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 416  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 431  | time.is                                 | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 480  | 456.cloudflare.182682.xyz               | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 22   | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 36   | wilson.ns.cloudflare.com                | 172.64.35.110                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 50   | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 108  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 176  | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 296  | freeyx.cloudflare88.eu.org              | 141.101.120.156                         | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 314  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 371  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 376  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 432  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 459  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 44   | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 69   | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 138  | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 140  | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 171  | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 236  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 264  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 266  | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 355  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 362  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 381  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 413  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 454  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 470  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 484  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 9    | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 18   | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 61   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 119  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 129  | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 188  | icook.hk                                | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 276  | cfip.xxxxxxxx.tk                        | 190.93.246.67                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 295  | freeyx.cloudflare88.eu.org              | 141.101.120.246                         | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 318  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 323  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 357  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 372  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 398  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 477  | 456.cloudflare.182682.xyz               | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 482  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 31   | trevor.ns.cloudflare.com                | 172.64.35.154                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 52   | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 59   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 85   | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 144  | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 170  | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 190  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 263  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 328  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 420  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 450  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 501  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 54   | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 158  | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 221  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 250  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 351  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 385  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 433  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 465  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 47   | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 48   | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 65   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 77   | otto.ns.cloudflare.com                  | 162.159.44.135                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 92   | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 105  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 152  | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 157  | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 164  | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 174  | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 180  | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 210  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 228  | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 230  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 232  | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 245  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 275  | cfip.xxxxxxxx.tk                        | 198.41.214.141                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 333  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 360  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 447  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 473  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 483  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 74   | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 100  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 104  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 106  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 128  | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 130  | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 211  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 234  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 239  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 244  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 252  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 272  | cfip.xxxxxxxx.tk                        | 104.27.21.118                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 283  | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 300  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 361  | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 389  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 434  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 486  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 500  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 503  | uriah.ns.cloudflare.com                 | 172.64.35.194                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 43   | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 56   | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 66   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 72   | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 88   | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 124  | abdullah.ns.cloudflare.com              | 172.64.35.203                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 131  | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 145  | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 178  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 186  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 202  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 205  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 231  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 277  | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 286  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 294  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 317  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 342  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 349  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 387  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 400  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 408  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 451  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 493  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 16   | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 17   | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 19   | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 71   | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 120  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 147  | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 198  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 251  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 259  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 281  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 316  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 322  | decker.ns.cloudflare.com                | 108.162.195.155                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 334  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 352  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 353  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 199 条记录
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
