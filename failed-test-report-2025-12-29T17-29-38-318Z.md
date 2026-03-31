# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:29:38
- **数据来源**: connectivity_results-20251229-172937.json
- **总测试数**: 494
- **失败测试数**: 2
- **成功测试数**: 492
- **失败率**: 0.40%
- **平均延迟**: 72.21ms
- **最小延迟**: 41ms
- **最大延迟**: 1612ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:29:38
- **IP地址**: 2a09:bac5:9f24:166e::23c:b7
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
| 36   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 374  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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
| 475  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 42   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 62   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 61   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 64   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 366  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 16   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 50   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 468  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 150  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 472  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 41   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 43   | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 85   | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 89   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 102  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 162  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 186  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 188  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 195  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 238  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 277  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 304  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 326  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 340  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 385  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 463  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 63   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 91   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 108  | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 159  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 191  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 225  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 245  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 382  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 433  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 45   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 60   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 72   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 81   | cf.877774.xyz                           | cf.877774.xyz                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 112  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 114  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 122  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 179  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 313  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 316  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 320  | 456.cloudflare.182682.xyz               | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 388  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 410  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 432  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 452  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 485  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 8    | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 11   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 30   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 82   | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 86   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 88   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 90   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 117  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 137  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 138  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 158  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 160  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 161  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 197  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 228  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 241  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 271  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 280  | palera.in                               | 172.67.157.122                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 299  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 330  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 339  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 365  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 411  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 419  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 19   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 48   | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 58   | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 87   | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 100  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 123  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 135  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 201  | bestcf.030101.xyz                       | 198.41.209.230                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 300  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 323  | 456.cloudflare.182682.xyz               | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 333  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 355  | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 389  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 46   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 67   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 109  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 111  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 127  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 154  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 164  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 189  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 194  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 200  | bestcf.030101.xyz                       | 104.19.147.41                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 203  | bestcf.030101.xyz                       | 2606:4700:0:69:bc02:15e0:153e:f2cf      | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 216  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 229  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 239  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 244  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 298  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 329  | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 367  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 377  | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 429  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 440  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 467  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 474  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 477  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 478  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 480  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 488  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 489  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 6    | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 17   | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 44   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 66   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 77   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 115  | www.hugedomains.com                     | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 156  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 196  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 207  | cf.zhetengsha.eu.org                    | 2a06:98c1:3108::6812:2a62               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 220  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 230  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 253  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 269  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 278  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 312  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 319  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 338  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 356  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 371  | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 372  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 373  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 420  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 442  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 447  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 458  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 7    | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 13   | cf.0sm.com                              | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 18   | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 39   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 92   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 93   | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 95   | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 153  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 190  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 193  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 208  | cf.zhetengsha.eu.org                    | 2a06:98c1:3101::ac40:919e               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 231  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 246  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 261  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 273  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 282  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 307  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 325  | 456.cloudflare.182682.xyz               | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 332  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 364  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 391  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 413  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 431  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 455  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 483  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 491  | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 14   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 65   | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 75   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 128  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 129  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 178  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 232  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 251  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 258  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 265  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 276  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 301  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 306  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 322  | 456.cloudflare.182682.xyz               | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 328  | 172.67.106.26                           | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 335  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 346  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 370  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 392  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 396  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 405  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 426  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 428  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 451  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 473  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 479  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 493  | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 35   | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 47   | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 36 条记录
- **快 (50-100ms)**: 164 条记录
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
