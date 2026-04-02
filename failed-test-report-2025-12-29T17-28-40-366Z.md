# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:28:40
- **数据来源**: connectivity_results-20251229-172839.json
- **总测试数**: 495
- **失败测试数**: 3
- **成功测试数**: 492
- **失败率**: 0.61%
- **平均延迟**: 56.67ms
- **最小延迟**: 40ms
- **最大延迟**: 658ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:28:40
- **IP地址**: 2a09:bac1:76a0:28::1d3:97
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

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 36   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 108  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 495  | 198.41.194.162   | 198.41.194.162 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.194.162:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 52   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 263  | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 9    | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 28   | trevor.ns.cloudflare.com                | 172.64.35.154                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 107  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 343  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 413  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 444  | julio.ns.cloudflare.com                 | 172.64.35.209                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 481  | uriah.ns.cloudflare.com                 | 172.64.35.194                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 27   | trevor.ns.cloudflare.com                | 4444.cloudflare.182682.xyz                         | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 72   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 141  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 197  | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 228  | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 270  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 346  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 349  | 162.159.24.131                          | 162.159.24.131                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 367  | 162.159.58.65                           | 162.159.58.65                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 386  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 429  | ip.gs                                   | 172.67.160.28                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 22   | wilson.ns.cloudflare.com                | 108.162.195.110                         | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 129  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 170  | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 183  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 196  | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 209  | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 226  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 247  | decker.ns.cloudflare.com                | 172.64.35.155                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 251  | kyree.ns.cloudflare.com                 | 108.162.195.207                         | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 274  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 301  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 303  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 313  | cf.090227.xyz                           | 2a06:98c1:3101::ac40:919e               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 347  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 350  | moura.ns.cloudflare.com                 | 162.159.44.217                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 356  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 375  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 383  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 397  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 415  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 430  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 436  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 20   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 48   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 49   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 64   | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 85   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 134  | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 161  | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 163  | craig.ns.cloudflare.com                 | 162.159.44.192                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 182  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 184  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 185  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 262  | dylan.ns.cloudflare.com                 | 108.162.195.187                         | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 311  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 317  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 342  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 352  | moura.ns.cloudflare.com                 | 108.162.195.217                         | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 357  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 370  | time.is                                 | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 394  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 414  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 427  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 443  | julio.ns.cloudflare.com                 | 162.159.44.209                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 456  | www.visa.com.hk                         | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 458  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 464  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 468  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 470  | singapore.com                           | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 473  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 475  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 13   | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 86   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 88   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 122  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 149  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 176  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 208  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 234  | toy-people.com                          | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 235  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 280  | bestcf.030101.xyz                       | 198.41.209.230                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 310  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 314  | japan.com                               | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 318  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 368  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 377  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 388  | palera.in                               | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 392  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 395  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 411  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 416  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 453  | 456.cloudflare.182682.xyz               | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 476  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 480  | uriah.ns.cloudflare.com                 | 162.159.44.194                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 491  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 39   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 46   | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 51   | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 54   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 57   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 75   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 97   | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 115  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 147  | 4444.cloudflare.182682.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 152  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 153  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 240  | lewis.ns.cloudflare.com                 | 108.162.195.159                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 278  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 285  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 293  | cf.zhetengsha.eu.org                    | 104.18.42.98                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 312  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 315  | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 316  | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 321  | braden.ns.cloudflare.com                | 162.159.44.169                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 338  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 348  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 372  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 373  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 393  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 407  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 418  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 423  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 431  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 437  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 449  | 456.cloudflare.182682.xyz               | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 451  | 456.cloudflare.182682.xyz               | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 452  | 456.cloudflare.182682.xyz               | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 460  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 467  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 8    | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 15   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 19   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 29   | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 42   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 43   | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 59   | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 60   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 61   | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 66   | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 78   | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 93   | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 104  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 119  | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 130  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 136  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 154  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 178  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 200  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 202  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 207  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 210  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 214  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 220  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 245  | decker.ns.cloudflare.com                | 108.162.195.155                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 254  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 260  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 268  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 287  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 299  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 307  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 345  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 366  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 376  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 382  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 391  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 408  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 409  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 410  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 412  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 420  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 422  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 424  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 454  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 455  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 474  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 478  | www.7749tv.com                          | 104.17.196.161                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 487  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 489  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 493  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 494  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 7    | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 38   | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 58   | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 63   | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 76   | cloudflare.182682.xyz                   | 6666.cloudflare.182682.xyz                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 98   | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 116  | cu.877774.xyz                           | 172.64.145.202                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 117  | cf.877774.xyz                           | cf.877774.xyz                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 118  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 120  | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 123  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 125  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 137  | www.4444.cloudflare.182682.xyz                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 151  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 189  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 238  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 200 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 3 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
