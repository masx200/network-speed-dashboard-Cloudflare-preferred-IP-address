# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:25:59
- **数据来源**: connectivity_results-20251229-172558.json
- **总测试数**: 494
- **失败测试数**: 6
- **成功测试数**: 488
- **失败率**: 1.21%
- **平均延迟**: 75.88ms
- **最小延迟**: 39ms
- **最大延迟**: 2870ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:25:59
- **IP地址**: 2a09:bac5:9f20:1791::259:e7
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

- **连接超时: I/O超时**: 6 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 32   | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 131  | craig.ns.cloudflare.com  | 108.162.195.192 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.192:443: i/o timeout |
| 189  | dylan.ns.cloudflare.com  | 172.64.35.187   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.187:443: i/o timeout   |
| 263  | bowen.ns.cloudflare.com  | 162.159.44.83   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.83:443: i/o timeout   |
| 361  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 419  | damien.ns.cloudflare.com | 108.162.195.168 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.168:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 6 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 108.162（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 6 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 465  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 231  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 128  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 437  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 184  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 111  | www.hugedomains.com                     | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 240  | cf.090227.xyz                           | 104.18.35.15                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 303  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 100  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 438  | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 156  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 191  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 297  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 305  | singapore.com                           | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 371  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 381  | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 455  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 87   | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 227  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 265  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 304  | singapore.com                           | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 329  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 332  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 365  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 414  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 59   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 97   | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 174  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 181  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 194  | bestcf.030101.xyz                       | 198.41.209.230                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 203  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 229  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 236  | fbi.gov                                 | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 254  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 301  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 343  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 395  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 439  | japan.com                               | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 464  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 473  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 26   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 119  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 162  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 171  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 172  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 208  | cf.zhetengsha.eu.org                    | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 237  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 270  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 309  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 341  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 356  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 402  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 443  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 456  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 458  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 477  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 44   | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 109  | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 198  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 241  | cf.090227.xyz                           | 172.64.152.241                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 283  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 289  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 310  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 321  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 330  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 331  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 362  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 427  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 441  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 462  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 470  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 474  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 108  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 150  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 173  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 183  | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 243  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 272  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 293  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 316  | 456.cloudflare.182682.xyz               | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 342  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 370  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 385  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 406  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 422  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 457  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 471  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 484  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 6    | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 12   | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 88   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 102  | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 117  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 185  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 242  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 273  | palera.in                               | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 276  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 290  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 403  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 468  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 63   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 76   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 103  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 147  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 219  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 224  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 232  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 295  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 368  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 404  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 463  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 467  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 475  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 482  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 3    | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 22   | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 31   | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 51   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 57   | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 86   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 101  | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 114  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 127  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:bf:5dba:fabf:8068:e072   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 152  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 154  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 197  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 207  | cf.zhetengsha.eu.org                    | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 213  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 225  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 271  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 355  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 424  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 431  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 11   | www.ipget.net                           | 104.21.15.212                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 40   | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 53   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 56   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 64   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 82   | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 130  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 158  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 163  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 186  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 195  | bestcf.030101.xyz                       | 2606:4700:0:c5:4803:8845:8bde:1897      | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 258  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 302  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 318  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 327  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 336  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 360  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 394  | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 448  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 472  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 5    | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 20   | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 73   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 77   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 105  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 110  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 113  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 118  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 129  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 134  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 164  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 199  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 234  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 308  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 344  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 359  | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 364  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 383  | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 388  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 389  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 390  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 405  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 411  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 415  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 421  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 434  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 445  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 452  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 453  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 454  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 486  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 27   | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 48   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 60   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 155  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 175  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 177  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 192  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 206  | cf.zhetengsha.eu.org                    | 104.18.35.15                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 235  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 238  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 248  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 250  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 25 条记录
- **快 (50-100ms)**: 175 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 6 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
