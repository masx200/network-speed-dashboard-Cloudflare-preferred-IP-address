# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:21:08
- **数据来源**: connectivity_results-20251229-172107.json
- **总测试数**: 498
- **失败测试数**: 181
- **成功测试数**: 317
- **失败率**: 36.35%
- **平均延迟**: 114.42ms
- **最小延迟**: 63ms
- **最大延迟**: 1490ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:21:08
- **IP地址**: 52.173.182.162
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6015, -93.6127
- **时区**: America/Chicago
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
| 1    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 7    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 8    | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 11   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 12   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 16   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 17   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 18   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 21   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 22   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 26   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 27   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 28   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 32   | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 33   | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 34   | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 40   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 41   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 42   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 43   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 44   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 47   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 48   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 51   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 52   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 55   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 56   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 62   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 63   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 67   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 68   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 69   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 74   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 90   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 91   | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 95   | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 96   | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 97   | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 100  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 106  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 107  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 108  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 111  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 112  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 116  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 117  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 118  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 128  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 129  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 130  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 136  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 137  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 144  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:0:fb:e00f:f23d:42c6      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:fb:e00f:f23d:42c6]:443: connect: network is unreachable      |
| 148  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 149  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 150  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 154  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 155  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 156  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 160  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 161  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 162  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 192  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 193  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 194  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 198  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 199  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 200  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 204  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 205  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 206  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 210  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 211  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 215  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 216  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 217  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 221  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 222  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 223  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 229  | bestcf.030101.xyz                       | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:e7ac:854f:c15c:d3b1:fc6a]:443: connect: network is unreachable    |
| 230  | bestcf.030101.xyz                       | 2606:4700:0:c5:4803:8845:8bde:1897      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c5:4803:8845:8bde:1897]:443: connect: network is unreachable      |
| 233  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 234  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 237  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 238  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 241  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 242  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 243  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 248  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 249  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 250  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 255  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 256  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 260  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 261  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 262  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 266  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 267  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 268  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 272  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 273  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 274  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 283  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 284  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 285  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 289  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 290  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 291  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 295  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 296  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 297  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 302  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 303  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 309  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 310  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 311  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 314  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 315  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 317  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 321  | 456.cloudflare.182682.xyz                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 322  | 456.cloudflare.182682.xyz                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 323  | 456.cloudflare.182682.xyz                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 326  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 327  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 329  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 332  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 333  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 336  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 337  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 338  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 345  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 346  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 350  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 351  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 357  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 358  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 359  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 360  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 364  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 368  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 369  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 370  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 374  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 378  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 379  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 380  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 384  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 385  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 386  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 393  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 394  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 401  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 402  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 403  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 407  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 408  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 421  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 424  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 429  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 430  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 437  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 438  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 439  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 444  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 445  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 446  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 458  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 459  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 460  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 465  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 474  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 475  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 476  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 477  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 478  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 479  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 480  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 481  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 489  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 497  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 498  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 279  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 280  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 377  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 426  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 259  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 240  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 414  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 387  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 406  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 457  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 72   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 110  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 133  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 214  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 319  | 456.cloudflare.182682.xyz                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 334  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 65   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 103  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 258  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 313  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 463  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 466  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 30   | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 61   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 316  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 397  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 422  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 490  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 70   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 94   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 432  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 6    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 20   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 124  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 257  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 363  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 473  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 173  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 201  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 347  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 373  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 411  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 54   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 64   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 340  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 371  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 392  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 49   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 187  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 189  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 207  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 278  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 300  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 324  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 434  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 441  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 57   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 127  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 232  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 235  | xn--b6gac.eu.org                      | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 58   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 417  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 431  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 142  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 348  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 383  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 409  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 15   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 75   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 84   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 122  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 181  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 269  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 339  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 454  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 226  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 448  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 472  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 483  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 53   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 276  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 428  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 488  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 493  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 304  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 307  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 362  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 218  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 413  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 420  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 447  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 5    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 131  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 140  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 281  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 381  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 59   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 271  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 277  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 13   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 123  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 372  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 455  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 470  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 494  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 10   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 87   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 115  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 82   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 180  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 335  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 343  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 104  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 288  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 135  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 39   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 165  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 174  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 251  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 331  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 354  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 451  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 485  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 2    | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 151  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 179  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 231  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 254  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 423  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 81   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 170  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 171  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 188  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 282  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 89   | icook.hk                              | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 191  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 301  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 433  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 36   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 38   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 330  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 427  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 467  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 375  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 440  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 482  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 83   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 172  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 419  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 453  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 462  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 31   | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 105  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 183  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 308  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 197  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 312  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 320  | 456.cloudflare.182682.xyz                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 449  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 92   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 159  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 184  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 236  | xn--b6gac.eu.org                      | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 85   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 404  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 24   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 185  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 209  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 469  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 102  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 225  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 252  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 344  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 382  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 3    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 88   | icook.hk                              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 306  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 23   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 101  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 195  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 71   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 77   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 79   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 342  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 468  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 76   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 286  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 398  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 412  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 325  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 395  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 66   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 143  | freeyx.cloudflare88.eu.org            | 141.101.120.176 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 169  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 178  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 244  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 349  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 145 条记录
- **正常 (100-200ms)**: 55 条记录
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
