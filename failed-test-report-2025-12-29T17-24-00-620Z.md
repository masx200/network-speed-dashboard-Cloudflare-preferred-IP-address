# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:24:00
- **数据来源**: connectivity_results-20251229-172359.json
- **总测试数**: 498
- **失败测试数**: 2
- **成功测试数**: 496
- **失败率**: 0.40%
- **平均延迟**: 56.97ms
- **最小延迟**: 30ms
- **最大延迟**: 1513ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:24:00
- **IP地址**: 2a09:bac1:76a0:28::4:371
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 37.1835, -121.7714
- **时区**: America/Los_Angeles
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
| 93   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 378  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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
| 325  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 30       | cloudflare |
| 436  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 34       | cloudflare |
| 439  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 69   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 176  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 215  | xn--b6gac.eu.org                        | 2a06:98c1:3120::3                       | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 291  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 35       | cloudflare |
| 322  | singapore.com                           | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 352  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 398  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 15   | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 157  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 178  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 202  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 248  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 256  | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 259  | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 261  | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 388  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 5    | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 45   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 61   | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 80   | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 108  | icook.hk                                | 188.114.96.3                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 123  | 4444.cloudflare.182682.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 131  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 158  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 203  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 206  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 247  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 279  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 287  | time.is                                 | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 317  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 331  | 456.cloudflare.182682.xyz               | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 437  | www.wto.org                             | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 443  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 494  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 11   | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 50   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 51   | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 57   | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 89   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 100  | cfip.xxxxxxxx.tk                        | 104.27.21.118                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 103  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 121  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 167  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 213  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 267  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 290  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 295  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 296  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 316  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 324  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 354  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 370  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 404  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 420  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 455  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 460  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 462  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 466  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 6    | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 7    | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 9    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 23   | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 49   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 55   | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 67   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 71   | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 86   | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 88   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 91   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 109  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 111  | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 112  | www.4444.cloudflare.182682.xyz                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 113  | www.4444.cloudflare.182682.xyz                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 209  | cf.zhetengsha.eu.org                    | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 226  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 228  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 252  | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 269  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 278  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 308  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 319  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 327  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 330  | 456.cloudflare.182682.xyz               | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 333  | 456.cloudflare.182682.xyz               | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 358  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 383  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 385  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 390  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 395  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 413  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 418  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 451  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 467  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 477  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 478  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 479  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 480  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 486  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 498  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 4    | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 18   | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 24   | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 28   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 30   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 43   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 58   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 62   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 63   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 81   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 87   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 92   | cfip.xxxxxxxx.tk                        | 198.41.214.141                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 107  | icook.hk                                | 188.114.97.3                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 118  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 125  | 4444.cloudflare.182682.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 129  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 147  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 166  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 172  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 185  | bestcf.030101.xyz                       | 2606:4700::fffd:819d:acda               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 200  | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 205  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 224  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 230  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 260  | cloudflare.182682.xyz                   | 6666.cloudflare.182682.xyz                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 288  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 314  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 339  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 351  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 359  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 379  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 380  | 172.67.79.211                           | 172.67.79.211                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 381  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 387  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 393  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 396  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 411  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 425  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 428  | www.7749tv.com                          | 104.19.133.4                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 434  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 458  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 464  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 488  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 496  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 8    | www.ipget.net                           | 104.21.15.212                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 10   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 14   | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 17   | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 19   | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 41   | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 56   | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 65   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 68   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 85   | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 95   | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 96   | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 135  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 137  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 164  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 177  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 183  | bestcf.030101.xyz                       | 104.19.147.41                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 229  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 242  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 246  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 250  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 270  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 275  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 294  | palera.in                               | 172.67.157.122                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 302  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 323  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 338  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 356  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 357  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 376  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 377  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 391  | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 394  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 400  | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 406  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 429  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 445  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 454  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 461  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 468  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 482  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 485  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 491  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 495  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 13   | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 16   | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 47   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 76   | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 78   | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 84   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 104  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 110  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 124  | 4444.cloudflare.182682.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 200 条记录
- **快 (50-100ms)**: 0 条记录
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
