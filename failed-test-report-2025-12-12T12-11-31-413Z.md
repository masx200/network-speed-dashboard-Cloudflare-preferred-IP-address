# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:11:31
- **数据来源**: connectivity_results-20251212-121131.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 114.93ms
- **最小延迟**: 66ms
- **最大延迟**: 563ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:11:31
- **IP地址**: 52.165.251.160
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

- **网络不可达: 网络不可达**: 164 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名                             | 目标IP                              | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | ------------------------------------- | ----------------------------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 1    | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable           |
| 6    | comicabc.com                          | 2606:4700:3030::ac43:ae15           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable           |
| 7    | comicabc.com                          | 2606:4700:3036::6815:400a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable           |
| 10   | www.ipget.net                         | 2606:4700:3036::6815:fd4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable            |
| 11   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable           |
| 15   | time.is                               | 2606:4700:20::ac43:449d             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable             |
| 16   | time.is                               | 2606:4700:20::681a:c36              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable              |
| 17   | time.is                               | 2606:4700:20::681a:d36              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable              |
| 21   | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable             |
| 22   | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable             |
| 23   | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable             |
| 27   | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable             |
| 28   | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable             |
| 29   | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable             |
| 33   | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable             |
| 34   | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable             |
| 35   | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable             |
| 39   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable             |
| 40   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable             |
| 41   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable             |
| 44   | www.pcmag.com                         | 2606:4700::6810:1576                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                |
| 45   | www.pcmag.com                         | 2606:4700::6810:1476                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                |
| 48   | cf.0sm.com                            | 2606:4700:3032::6815:785            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable            |
| 49   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable           |
| 52   | ipinfo.in                             | 2606:4700:3031::6815:1581           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable           |
| 53   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable           |
| 56   | steamdb.info                          | 2606:4700:10::6814:22d4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable             |
| 57   | steamdb.info                          | 2606:4700:10::ac42:affa             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable             |
| 60   | www.gov.ua                            | 2606:4700:3033::ac43:d17f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable           |
| 61   | www.gov.ua                            | 2606:4700:3031::6815:1748           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable           |
| 72   | iplocation.io                         | 2606:4700:20::681a:bde              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable              |
| 73   | iplocation.io                         | 2606:4700:20::681a:ade              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable              |
| 74   | iplocation.io                         | 2606:4700:20::ac43:4664             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable             |
| 75   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable            |
| 91   | icook.hk                              | 2606:4700:3031::6815:5ad2           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable           |
| 92   | icook.hk                              | 2606:4700:3037::ac43:a168           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable           |
| 96   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable             |
| 97   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable             |
| 98   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable             |
| 101  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable           |
| 105  | www.hugedomains.com                   | 2606:4700:20::681a:725              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable              |
| 106  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable             |
| 107  | www.hugedomains.com                   | 2606:4700:20::681a:625              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable              |
| 111  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable             |
| 112  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable             |
| 113  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable             |
| 116  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable           |
| 117  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable           |
| 126  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable           |
| 127  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable           |
| 141  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable           |
| 142  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable           |
| 147  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:fb:e00f:f2d6:8f63  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:fb:e00f:f2d6:8f63]:443: connect: network is unreachable  |
| 151  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable             |
| 152  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable             |
| 153  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable             |
| 186  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable             |
| 187  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable             |
| 188  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable             |
| 192  | toy-people.com                        | 2606:4700:20::ac43:4812             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable             |
| 193  | toy-people.com                        | 2606:4700:20::681a:324              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable              |
| 194  | toy-people.com                        | 2606:4700:20::681a:224              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable              |
| 195  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable           |
| 199  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable             |
| 200  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable             |
| 201  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable             |
| 205  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable             |
| 206  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable             |
| 207  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable             |
| 211  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable             |
| 212  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable             |
| 213  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable             |
| 218  | www.whatismyip.com                    | 2606:4700:20::681a:c17              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable              |
| 219  | www.whatismyip.com                    | 2606:4700:20::681a:d17              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable              |
| 220  | www.whatismyip.com                    | 2606:4700:20::ac43:4581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable             |
| 223  | zread.ai                              | 2606:4700:3033::6815:4cf0           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable           |
| 224  | zread.ai                              | 2606:4700:3032::ac43:ca4e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable           |
| 231  | bestcf.030101.xyz                     | 2606:4700:0:26e6:9bb0:8293:779:dc27 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:26e6:9bb0:8293:779:dc27]:443: connect: network is unreachable |
| 232  | bestcf.030101.xyz                     | 2606:4700:0:b21c:66ba:fef:2ef4:2658 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:b21c:66ba:fef:2ef4:2658]:443: connect: network is unreachable |
| 235  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable           |
| 236  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable           |
| 239  | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable           |
| 240  | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable           |
| 243  | fbi.gov                               | 2606:4700::6810:95f4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                |
| 244  | fbi.gov                               | 2606:4700::6810:94f4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                |
| 247  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable           |
| 248  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable           |
| 255  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable             |
| 256  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable             |
| 257  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable             |
| 261  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable             |
| 262  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable             |
| 263  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable             |
| 267  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable             |
| 268  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable             |
| 269  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable             |
| 278  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable             |
| 279  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable             |
| 280  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable             |
| 284  | ip.sb                                 | 2606:4700:20::681a:c1f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable              |
| 285  | ip.sb                                 | 2606:4700:20::681a:d1f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable              |
| 286  | ip.sb                                 | 2606:4700:20::ac43:4bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable             |
| 289  | palera.in                             | 2606:4700:3032::ac43:9d7a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable           |
| 290  | palera.in                             | 2606:4700:3035::6815:3a48           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable           |
| 295  | ip.gs                                 | 2606:4700:3036::6815:eb0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable            |
| 296  | ip.gs                                 | 2606:4700:3035::ac43:a01c           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable           |
| 300  | singapore.com                         | 2606:4700:20::681a:c8c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable              |
| 301  | singapore.com                         | 2606:4700:20::681a:d8c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable              |
| 302  | singapore.com                         | 2606:4700:20::ac43:4bc2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable             |
| 306  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable              |
| 307  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable              |
| 308  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable             |
| 312  | whatismyipaddress.com                 | 2606:4700::6813:de4f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                |
| 313  | whatismyipaddress.com                 | 2606:4700::6813:df4f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                |
| 317  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable             |
| 318  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable             |
| 319  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable             |
| 321  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable           |
| 325  | dnschecker.org                        | 2606:4700:20::ac43:49d8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable             |
| 326  | dnschecker.org                        | 2606:4700:20::681a:659              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable              |
| 327  | dnschecker.org                        | 2606:4700:20::681a:759              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable              |
| 331  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable             |
| 332  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable             |
| 333  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable             |
| 336  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable           |
| 337  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable           |
| 338  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable           |
| 343  | icook.tw                              | 2606:4700:10::6814:1c4a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable             |
| 344  | icook.tw                              | 2606:4700:10::ac42:9e73             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable             |
| 345  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable           |
| 349  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable             |
| 350  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable             |
| 351  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable             |
| 353  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable           |
| 357  | www.digitalocean.com                  | 2606:4700::6813:ae44                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                |
| 358  | www.digitalocean.com                  | 2606:4700::6813:ad44                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                |
| 366  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable             |
| 367  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable             |
| 368  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable             |
| 374  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable             |
| 375  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable             |
| 376  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable             |
| 379  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable           |
| 382  | tasteatlas.com                        | 2606:4700::6811:2469                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                |
| 383  | tasteatlas.com                        | 2606:4700::6811:2569                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                |
| 388  | www.udemy.com                         | 2606:4700::6810:8fed                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                |
| 389  | www.udemy.com                         | 2606:4700::6810:8eed                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                |
| 392  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable           |
| 393  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable            |
| 394  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable              |
| 401  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable            |
| 407  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable             |
| 408  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable             |
| 409  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable             |
| 414  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable             |
| 415  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable             |
| 416  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable             |
| 421  | japan.com                             | 2606:4700:20::ac43:465c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable             |
| 422  | japan.com                             | 2606:4700:20::681a:43c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable              |
| 423  | japan.com                             | 2606:4700:20::681a:53c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable              |
| 426  | ifconfig.co                           | 2606:4700:3030::ac43:a86a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable           |
| 427  | ifconfig.co                           | 2606:4700:3037::6815:365b           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable           |
| 431  | www.wto.org                           | 2606:4700:4406::ac40:9242           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable           |
| 432  | www.wto.org                           | 2a06:98c1:3102::6812:29be           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable           |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 440  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 442  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 443  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 164 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 167 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 164 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：time.is (3次), trevor.ns.cloudflare.com
(3次), moura.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 210  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 390  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 398  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 37   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 229  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 354  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 391  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 139  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 171  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 292  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 384  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 82   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 411  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 81   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 298  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 335  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 77   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 346  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 50   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 252  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 14   | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 58   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 71   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 167  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 226  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 369  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 410  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 129  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 140  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 381  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 78   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 234  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 146  | freeyx.cloudflare88.eu.org            | 141.101.120.244 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 227  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 242  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 329  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 274  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 282  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 419  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 38   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 123  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 175  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 355  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 9    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 121  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 277  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 334  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 79   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 364  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 417  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 62   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 119  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 132  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 216  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 371  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 64   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 80   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 94   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 293  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 315  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 348  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 66   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 67   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 70   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 114  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 118  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 238  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 250  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 270  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 287  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 273  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 283  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 203  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 241  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 320  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 438  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 198  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 230  | bestcf.030101.xyz                     | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 314  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 51   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 135  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 168  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 297  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 328  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 115  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 130  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 174  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 217  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 266  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 143  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 190  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 83   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 103  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 164 次

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
