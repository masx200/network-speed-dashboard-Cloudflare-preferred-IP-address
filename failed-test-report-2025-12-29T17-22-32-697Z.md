# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:22:32
- **数据来源**: connectivity_results-20251229-172232.json
- **总测试数**: 495
- **失败测试数**: 181
- **成功测试数**: 314
- **失败率**: 36.57%
- **平均延迟**: 117.78ms
- **最小延迟**: 69ms
- **最大延迟**: 3491ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:22:32
- **IP地址**: 172.215.217.96
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1437, -104.8117
- **时区**: America/Denver
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 178 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (178 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 5    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 6    | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 9    | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 10   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 13   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 14   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 17   | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 18   | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 22   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 23   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 24   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 28   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 29   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 30   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 33   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 34   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 37   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 38   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 41   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 42   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 45   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 46   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 50   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 51   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 67   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 74   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 75   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 76   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 81   | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 82   | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 83   | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 93   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 94   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 95   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 96   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 97   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 102  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 103  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 104  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 107  | cf.zhetengsha.eu.org                    | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 108  | cf.zhetengsha.eu.org                    | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 114  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 115  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 116  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 118  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:bf:5dba:fabf:8068:e072   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:bf:5dba:fabf:8068:e072]:443: connect: network is unreachable   |
| 125  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 126  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 127  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 131  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 132  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 133  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 137  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 138  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 139  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 143  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 144  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 145  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 149  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 150  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 151  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 181  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 182  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 183  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 188  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 189  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 192  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 193  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 194  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 197  | bestcf.030101.xyz                       | 2606:4700:0:c5:4803:8845:8bde:1897      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c5:4803:8845:8bde:1897]:443: connect: network is unreachable      |
| 198  | bestcf.030101.xyz                       | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:e7ac:854f:c15c:d3b1:fc6a]:443: connect: network is unreachable    |
| 203  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 206  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 207  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 211  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 212  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 213  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 214  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 220  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 221  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 222  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 230  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 231  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 235  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 236  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 242  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 243  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 244  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 247  | cf.090227.xyz                           | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 248  | cf.090227.xyz                           | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 252  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 253  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 254  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 258  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 259  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 260  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 264  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 265  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 266  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 270  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 271  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 272  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 276  | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 280  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 281  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 282  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 288  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 289  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 290  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 293  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 294  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 300  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 301  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 302  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 306  | 456.cloudflare.182682.xyz                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 307  | 456.cloudflare.182682.xyz                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 308  | 456.cloudflare.182682.xyz                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 312  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 313  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 317  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 318  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 319  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 322  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 323  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 327  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 328  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 329  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 333  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 334  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 335  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 337  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 338  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 347  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 348  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 349  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 350  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 355  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 356  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 357  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 360  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 361  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 364  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 365  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 366  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 371  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 372  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 376  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 377  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 385  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 386  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 387  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 388  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 392  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 393  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 394  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 397  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 401  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 408  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 409  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 410  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 414  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 415  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 416  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 420  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 421  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 424  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 425  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 434  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 435  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 436  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 474  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 475  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 476  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 477  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 478  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 479  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 480  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 481  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 482  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 489  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 494  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 495  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 450  | 104.26.5.134               | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 87   | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 146  | dylan.ns.cloudflare.com    | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 298  | singapore.com              | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 442  | 172.64.91.69               | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 225  | www.okcupid.com            | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 309  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 369  | icook.tw                   | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 379  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 493  | cfip.xxxxxxxx.tk           | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 90   | cloudflare.182682.xyz      | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 237  | saas.sin.fan               | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 448  | 172.64.229.7               | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 187  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 340  | 172.67.106.26              | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 456  | 162.159.140.116            | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 71   | iplocation.io              | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 99   | www.hugedomains.com        | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 285  | benedict.ns.cloudflare.com | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 405  | lewis.ns.cloudflare.com    | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 451  | 162.159.137.204            | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 216  | www.visa.cn                | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 326  | dnschecker.org             | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | tasteatlas.com             | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 457  | 104.18.89.52               | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 35   | ipinfo.in                  | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 91   | cloudflare.182682.xyz      | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 165  | cmcc.877774.xyz            | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 174  | cmcc.877774.xyz            | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 201  | asia.877774.xyz            | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 395  | cfip.1323123.xyz           | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 437  | 198.41.208.15              | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 79   | huxley.ns.cloudflare.com   | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 153  | cmcc.877774.xyz            | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 210  | craig.ns.cloudflare.com    | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 277  | ip.sb                      | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 454  | 104.26.8.117               | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 473  | 104.18.189.153             | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 100  | www.hugedomains.com        | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 111  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 158  | cmcc.877774.xyz            | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 215  | www.visa.cn                | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 273  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 304  | 456.cloudflare.182682.xyz               | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 354  | tasteatlas.com             | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 367  | gamer.com.tw               | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 430  | www.csgo.com               | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 468  | 104.19.212.207             | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 44   | www.gov.ua                 | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 57   | 172.67.75.172              | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 58   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 156  | cmcc.877774.xyz            | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 184  | 104.17.79.11               | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 362  | www.udemy.com              | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 429  | www.csgo.com               | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 68   | 172.67.110.232             | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 77   | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 339  | 104.18.42.26               | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 469  | 104.17.69.244              | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 491  | cfip.xxxxxxxx.tk           | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 69   | shopify.com                | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 202  | 172.67.120.0               | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 344  | uriah.ns.cloudflare.com    | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 484  | 104.16.65.1                | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 162  | cmcc.877774.xyz            | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 171  | cmcc.877774.xyz            | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 219  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 316  | ashton.ns.cloudflare.com   | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 370  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 417  | 172.67.181.209             | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 438  | 198.41.194.162             | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 440  | 172.64.52.127              | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 467  | 104.18.255.167             | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 492  | cfip.xxxxxxxx.tk           | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 98   | 172.67.243.218             | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 155  | cmcc.877774.xyz            | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 157  | cmcc.877774.xyz            | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 190  | cloudflare-ip.mofashi.ltd  | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 275  | www.ipchicken.com          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 396  | 104.26.13.31               | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 88   | cloudflare.182682.xyz      | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 110  | www.visa.com.sg            | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 112  | pranab.ns.cloudflare.com   | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 147  | dylan.ns.cloudflare.com    | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 159  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 169  | cmcc.877774.xyz            | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 223  | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 229  | xn--b6gac.eu.org           | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 232  | 172.64.151.55              | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 249  | bowen.ns.cloudflare.com    | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 263  | time.is                    | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 358  | cf.877774.xyz              | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 470  | 104.31.16.158              | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 72   | iplocation.io              | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 186  | zread.ai                   | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 199  | asia.877774.xyz            | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 381  | 172.64.35.24               | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 441  | 162.159.61.183             | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 471  | 104.17.167.134             | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 472  | 104.18.223.253             | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 105  | cf.zhetengsha.eu.org       | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 122  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 170  | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 227  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 246  | cf.090227.xyz              | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 375  | www.digitalocean.com       | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 382  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 404  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 483  | 104.17.142.212             | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 129  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 257  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 274  | www.ipchicken.com          | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 466  | 104.19.154.200             | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 59   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 172  | cmcc.877774.xyz            | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 205  | cf.877771.xyz              | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 351  | 162.159.133.85             | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 32   | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 117  | freeyx.cloudflare88.eu.org | 141.101.120.176 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 255  | moura.ns.cloudflare.com    | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 261  | time.is                    | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 383  | otto.ns.cloudflare.com     | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 412  | japan.com                  | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 446  | 104.26.4.90                | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 78   | huxley.ns.cloudflare.com   | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 86   | www.4chan.org              | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 262  | time.is                    | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 283  | www.glassdoor.com          | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 411  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 458  | 104.18.81.19               | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 12   | comicabc.com               | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 109  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 269  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 325  | dnschecker.org             | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 373  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 390  | damien.ns.cloudflare.com   | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 398  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 465  | 104.19.220.22              | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 80   | huxley.ns.cloudflare.com   | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 314  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 363  | www.udemy.com              | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 423  | stock.hostmonit.com        | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 8    | icook.hk                   | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 154  | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 256  | moura.ns.cloudflare.com    | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 422  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 49   | cf.0sm.com                 | 172.67.66.56    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 121  | na.877774.xyz              | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 134  | decker.ns.cloudflare.com   | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 178  | www.whatismyip.com         | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 20   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 52   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 89   | cloudflare.182682.xyz      | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 140  | kyree.ns.cloudflare.com    | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 240  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 268  | rustam.ns.cloudflare.com   | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 460  | 198.41.208.224             | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 241  | braden.ns.cloudflare.com   | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 368  | gamer.com.tw               | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 433  | abdullah.ns.cloudflare.com | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 25   | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 36   | ipinfo.in                  | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 63   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 64   | ct.877774.xyz              | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 123  | cris.ns.cloudflare.com     | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 239  | braden.ns.cloudflare.com   | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 279  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 286  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 167  | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 176  | cmcc.877774.xyz            | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 374  | www.digitalocean.com       | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 428  | 172.64.82.114              | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 432  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 452  | 162.159.128.253            | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 120  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 175  | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 177  | cmcc.877774.xyz            | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 234  | fbi.gov                    | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 346  | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 19   | wilson.ns.cloudflare.com   | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 251  | bowen.ns.cloudflare.com    | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 462  | 104.18.151.172             | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 148  | dylan.ns.cloudflare.com    | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 180  | www.whatismyip.com         | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 345  | uriah.ns.cloudflare.com    | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 427  | 108.162.198.54             | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 92   | cloudflare.182682.xyz      | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 135  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 168  | cmcc.877774.xyz            | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 200  | asia.877774.xyz            | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 296  | 162.159.36.104             | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 330  | julio.ns.cloudflare.com    | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 402  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 1    | 172.64.154.18              | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 31   | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 228  | xn--b6gac.eu.org           | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 378  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 21   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 66   | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 384  | otto.ns.cloudflare.com     | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 157 条记录
- **正常 (100-200ms)**: 43 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 178 次

### 按协议统计

- **none**: 181 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
