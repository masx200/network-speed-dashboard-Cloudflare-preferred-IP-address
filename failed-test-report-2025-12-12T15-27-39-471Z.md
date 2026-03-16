# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:27:39
- **数据来源**: connectivity_results-20251212-152739.json
- **总测试数**: 451
- **失败测试数**: 167
- **成功测试数**: 284
- **失败率**: 37.03%
- **平均延迟**: 121.82ms
- **最小延迟**: 41ms
- **最大延迟**: 598ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:27:39
- **IP地址**: 40.76.238.182
- **国家/地区**: United States (US)
- **ASN**: AS8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **数据源**: ipinfo.io

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 163 次 (97.6%)
- **连接超时: I/O超时**: 4 次 (2.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (163 次测试)

| 序号 | 主机/域名                             | 目标IP                                | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                              |
| ---- | ------------------------------------- | ------------------------------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------- |
| 5    | comicabc.com                          | 2606:4700:3036::6815:400a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable             |
| 6    | comicabc.com                          | 2606:4700:3030::ac43:ae15             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable             |
| 10   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable               |
| 11   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable               |
| 12   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable               |
| 15   | www.pcmag.com                         | 2606:4700::6810:1476                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                  |
| 16   | www.pcmag.com                         | 2606:4700::6810:1576                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                  |
| 20   | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable               |
| 21   | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable               |
| 22   | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable               |
| 25   | www.ipget.net                         | 2606:4700:3036::6815:fd4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable              |
| 26   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable             |
| 29   | cf.0sm.com                            | 2606:4700:3032::6815:785              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable              |
| 30   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable             |
| 33   | ipinfo.in                             | 2606:4700:3031::6815:1581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable             |
| 34   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable             |
| 37   | www.gov.ua                            | 2606:4700:3031::6815:1748             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable             |
| 38   | www.gov.ua                            | 2606:4700:3033::ac43:d17f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable             |
| 41   | steamdb.info                          | 2606:4700:10::ac42:affa               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable               |
| 42   | steamdb.info                          | 2606:4700:10::6814:22d4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable               |
| 52   | iplocation.io                         | 2606:4700:20::681a:ade                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                |
| 53   | iplocation.io                         | 2606:4700:20::ac43:4664               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable               |
| 54   | iplocation.io                         | 2606:4700:20::681a:bde                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                |
| 57   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable              |
| 72   | icook.hk                              | 2606:4700:3031::6815:5ad2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable             |
| 73   | icook.hk                              | 2606:4700:3037::ac43:a168             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable             |
| 77   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable               |
| 78   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable               |
| 79   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable               |
| 85   | www.hugedomains.com                   | 2606:4700:20::ac43:46bf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable               |
| 86   | www.hugedomains.com                   | 2606:4700:20::681a:625                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                |
| 87   | www.hugedomains.com                   | 2606:4700:20::681a:725                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                |
| 88   | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable             |
| 92   | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable               |
| 93   | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable               |
| 94   | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable               |
| 102  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable             |
| 103  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable             |
| 117  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable               |
| 118  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable               |
| 119  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable               |
| 126  | time.is                               | 2606:4700:20::ac43:449d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable               |
| 127  | time.is                               | 2606:4700:20::681a:d36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                |
| 128  | time.is                               | 2606:4700:20::681a:c36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                |
| 131  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable             |
| 132  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable             |
| 138  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable               |
| 139  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable               |
| 140  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable               |
| 142  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:bf:5dba:14ae:9501:48f7 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:bf:5dba:14ae:9501:48f7]:443: connect: network is unreachable |
| 149  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable               |
| 150  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable               |
| 151  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable               |
| 155  | toy-people.com                        | 2606:4700:20::681a:324                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                |
| 156  | toy-people.com                        | 2606:4700:20::ac43:4812               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable               |
| 157  | toy-people.com                        | 2606:4700:20::681a:224                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                |
| 187  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable               |
| 188  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable               |
| 189  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable               |
| 193  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable               |
| 194  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable               |
| 195  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable               |
| 199  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable               |
| 200  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable               |
| 201  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable               |
| 206  | bestcf.030101.xyz                     | 2606:4700:0:b684:c5c1:5d02:e603:436c  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:b684:c5c1:5d02:e603:436c]:443: connect: network is unreachable  |
| 210  | www.whatismyip.com                    | 2606:4700:20::681a:d17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                |
| 211  | www.whatismyip.com                    | 2606:4700:20::ac43:4581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable               |
| 212  | www.whatismyip.com                    | 2606:4700:20::681a:c17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                |
| 216  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable             |
| 219  | zread.ai                              | 2606:4700:3033::6815:4cf0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable             |
| 220  | zread.ai                              | 2606:4700:3032::ac43:ca4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable             |
| 223  | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable             |
| 224  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable             |
| 228  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable               |
| 229  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable               |
| 230  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable               |
| 234  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable               |
| 235  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable               |
| 236  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable               |
| 238  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable             |
| 241  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable             |
| 242  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable             |
| 246  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable             |
| 247  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable             |
| 252  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable               |
| 253  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable               |
| 254  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable               |
| 259  | cf.090227.xyz                         | 2606:4700:4407::ac40:9052             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable             |
| 260  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable             |
| 263  | icook.tw                              | 2606:4700:10::ac42:9e73               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable               |
| 264  | icook.tw                              | 2606:4700:10::6814:1c4a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable               |
| 268  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable               |
| 269  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable               |
| 270  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable               |
| 275  | palera.in                             | 2606:4700:3035::6815:3a48             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable             |
| 276  | palera.in                             | 2606:4700:3032::ac43:9d7a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable             |
| 279  | fbi.gov                               | 2606:4700::6810:94f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                  |
| 280  | fbi.gov                               | 2606:4700::6810:95f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                  |
| 285  | singapore.com                         | 2606:4700:20::681a:d8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                |
| 286  | singapore.com                         | 2606:4700:20::681a:c8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                |
| 287  | singapore.com                         | 2606:4700:20::ac43:4bc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable               |
| 290  | ip.gs                                 | 2606:4700:3035::ac43:a01c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable             |
| 291  | ip.gs                                 | 2606:4700:3036::6815:eb0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable              |
| 296  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                |
| 297  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable               |
| 298  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                |
| 303  | ip.sb                                 | 2606:4700:20::681a:c1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                |
| 304  | ip.sb                                 | 2606:4700:20::681a:d1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                |
| 305  | ip.sb                                 | 2606:4700:20::ac43:4bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable               |
| 309  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable               |
| 310  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable               |
| 311  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable               |
| 314  | whatismyipaddress.com                 | 2606:4700::6813:df4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                  |
| 315  | whatismyipaddress.com                 | 2606:4700::6813:de4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                  |
| 317  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 321  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable               |
| 322  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable               |
| 323  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable               |
| 327  | dnschecker.org                        | 2606:4700:20::ac43:49d8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable               |
| 328  | dnschecker.org                        | 2606:4700:20::681a:659                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                |
| 329  | dnschecker.org                        | 2606:4700:20::681a:759                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                |
| 330  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable             |
| 339  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable               |
| 340  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable               |
| 341  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable               |
| 343  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable             |
| 348  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 349  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 353  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable               |
| 354  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable               |
| 355  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable               |
| 356  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable             |
| 359  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable             |
| 364  | www.udemy.com                         | 2606:4700::6810:8fed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                  |
| 365  | www.udemy.com                         | 2606:4700::6810:8eed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                  |
| 368  | tasteatlas.com                        | 2606:4700::6811:2469                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                  |
| 369  | tasteatlas.com                        | 2606:4700::6811:2569                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                  |
| 374  | www.digitalocean.com                  | 2606:4700::6813:ae44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                  |
| 375  | www.digitalocean.com                  | 2606:4700::6813:ad44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                  |
| 379  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                |
| 383  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable               |
| 384  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable               |
| 385  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable               |
| 390  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable               |
| 391  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable               |
| 392  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable               |
| 393  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable              |
| 401  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable               |
| 402  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable               |
| 403  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable               |
| 407  | ifconfig.co                           | 2606:4700:3030::ac43:a86a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable             |
| 408  | ifconfig.co                           | 2606:4700:3037::6815:365b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable             |
| 412  | japan.com                             | 2606:4700:20::ac43:465c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable               |
| 413  | japan.com                             | 2606:4700:20::681a:43c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                |
| 414  | japan.com                             | 2606:4700:20::681a:53c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                |
| 417  | www.wto.org                           | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 418  | www.wto.org                           | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 421  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable             |
| 422  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable              |
| 429  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable               |
| 430  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable               |
| 431  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable               |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 436  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 437  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 443  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 451  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 163 次 (97.6%)
- **连接超时**: 4 次 (2.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 167 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 163 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 400  | lewis.ns.cloudflare.com    | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 362  | www.udemy.com              | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 295  | 456.cloudflare.182682.xyz               | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 338  | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 97   | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 361  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 239  | cloudflare-ip.mofashi.ltd  | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 333  | www.ipchicken.com          | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 360  | gamer.com.tw               | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 405  | ifconfig.co                | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 167  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 256  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 319  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 39   | steamdb.info               | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 56   | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 335  | 104.18.14.76               | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 96   | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 95   | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 58   | 172.67.110.232             | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 60   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 274  | palera.in                  | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 294  | 456.cloudflare.182682.xyz               | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 428  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 144  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 180  | cmcc.877774.xyz            | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 277  | fbi.gov                    | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 49   | iplocation.io              | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 411  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 129  | yx-auto.pages.dev          | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 18   | trevor.ns.cloudflare.com   | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 145  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 326  | dnschecker.org             | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 366  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 373  | www.digitalocean.com       | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 91   | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 13   | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 23   | www.ipget.net              | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 331  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 396  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 162  | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 48   | shopify.com                | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 370  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 426  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 265  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 445  | cfip.xxxxxxxx.tk           | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 109  | cu.877774.xyz              | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 284  | singapore.com              | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 377  | 104.18.37.40               | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 63   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 107  | cu.877774.xyz              | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 415  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 302  | ip.sb                      | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 35   | www.gov.ua                 | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 125  | time.is                    | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 137  | pranab.ns.cloudflare.com   | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 158  | cmcc.877774.xyz            | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 135  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 386  | 104.26.13.31               | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 80   | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 233  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 112  | cu.877774.xyz              | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 153  | toy-people.com             | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 8    | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 17   | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 105  | cu.877774.xyz              | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 183  | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 191  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 301  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 399  | lewis.ns.cloudflare.com    | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 217  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 404  | 104.19.223.58              | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 46   | ipv4.ip.sb                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 81   | www.4chan.org              | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 98   | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 100  | cf.877771.xyz              | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 133  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 175  | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 307  | benedict.ns.cloudflare.com | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 27   | cf.0sm.com                 | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 232  | braden.ns.cloudflare.com   | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 262  | icook.tw                   | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 289  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 358  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 186  | kyree.ns.cloudflare.com    | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 240  | cloudflare-ip.mofashi.ltd  | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 261  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 281  | 104.16.61.163              | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 376  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 416  | www.wto.org                | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 66   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 169  | cmcc.877774.xyz            | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 244  | xn--b6gac.eu.org           | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 423  | 104.19.175.123             | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 3    | comicabc.com               | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 36   | www.gov.ua                 | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 111  | cu.877774.xyz              | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 178  | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 258  | cf.090227.xyz              | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 173  | cmcc.877774.xyz            | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 227  | bowen.ns.cloudflare.com    | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 96 条记录
- **正常 (100-200ms)**: 2 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 163 次

### 按协议统计

- **none**: 167 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
