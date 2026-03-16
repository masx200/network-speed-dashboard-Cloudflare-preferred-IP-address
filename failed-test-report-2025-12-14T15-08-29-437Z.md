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

| 序号 | 主机/域名                             | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | ------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 39   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 40   | ipinfo.in                             | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 43   | www.ipget.net                         | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 44   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 47   | comicabc.com                          | 2a06:98c1:3120::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                       |
| 48   | comicabc.com                          | 2a06:98c1:3121::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                       |
| 52   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 53   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 54   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 57   | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 58   | www.pcmag.com                         | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 62   | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 63   | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 64   | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 70   | cf.0sm.com                            | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 71   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 78   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 82   | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 83   | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 84   | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 91   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 92   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 93   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 96   | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 97   | icook.hk                              | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 100  | steamdb.info                          | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 101  | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 104  | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 105  | www.gov.ua                            | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 109  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 113  | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 114  | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 115  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 118  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 119  | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 122  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 123  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 134  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 137  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 138  | fbi.gov                               | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 142  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 143  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 144  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 148  | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 149  | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 150  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 153  | cf.090227.xyz                         | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 154  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 155  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 159  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 160  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 161  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 165  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 166  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 167  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 170  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa:66d2:154e:f17c:9632   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa:66d2:154e:f17c:9632]:443: connect: network is unreachable   |
| 171  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa73:2243:9add:8c5a:8a20 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa73:2243:9add:8c5a:8a20]:443: connect: network is unreachable |
| 185  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 186  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 187  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 191  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 192  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 193  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 196  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 197  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 200  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 201  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 206  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 207  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 208  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 218  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 219  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 220  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 224  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 225  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 226  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 236  | toy-people.com                        | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 237  | toy-people.com                        | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 238  | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 242  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 243  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 244  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 248  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 249  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 250  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 254  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 255  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 256  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 260  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 261  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 262  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 266  | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 267  | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 271  | ip.sb                                 | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 272  | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 273  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 277  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 278  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 282  | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 283  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 284  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 288  | bestcf.030101.xyz                     | 2606:4700::8d:f082:8938:66d8            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::8d:f082:8938:66d8]:443: connect: network is unreachable            |
| 289  | bestcf.030101.xyz                     | 2606:4700::fffd:819d:acda               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::fffd:819d:acda]:443: connect: network is unreachable               |
| 296  | palera.in                             | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 297  | palera.in                             | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 301  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 302  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 303  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 307  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 308  | singapore.com                         | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 309  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 312  | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 313  | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 316  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 317  | whatismyipaddress.com                 | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 319  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 321  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 325  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 326  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 327  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 333  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 334  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 335  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 339  | dnschecker.org                        | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 340  | dnschecker.org                        | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 341  | dnschecker.org                        | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 342  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 347  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 348  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 349  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 352  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 355  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 358  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 359  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 364  | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 365  | tasteatlas.com                        | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 372  | www.udemy.com                         | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 373  | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 375  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 379  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 380  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 383  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 384  | www.digitalocean.com                  | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 385  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 390  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 391  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 392  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 396  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 397  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 398  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 403  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 404  | ifconfig.co                           | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 408  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 409  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 410  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 426  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 427  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 431  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 432  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 433  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 439  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 440  | japan.com                             | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 441  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 444  | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 445  | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 108  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 417  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 447  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
| 298  | 456.cloudflare.182682.xyz                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
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
| 299  | 456.cloudflare.182682.xyz                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
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
