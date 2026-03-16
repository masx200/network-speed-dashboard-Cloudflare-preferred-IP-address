# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:41:56
- **数据来源**: connectivity_results-20251214-144156.json
- **总测试数**: 441
- **失败测试数**: 166
- **成功测试数**: 275
- **失败率**: 37.64%
- **平均延迟**: 77.18ms
- **最小延迟**: 35ms
- **最大延迟**: 679ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:41:56
- **IP地址**: 52.225.73.163
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 37.3388, -121.8916
- **时区**: America/Los_Angeles
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 163 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (163 次测试)

| 序号 | 主机/域名                             | 目标IP                                | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                              |
| ---- | ------------------------------------- | ------------------------------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------- |
| 5    | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable               |
| 6    | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable               |
| 7    | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable               |
| 11   | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable               |
| 12   | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable               |
| 13   | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable               |
| 16   | www.ipget.net                         | 2606:4700:3036::6815:fd4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable              |
| 17   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable             |
| 21   | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable               |
| 22   | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable               |
| 23   | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable               |
| 27   | toy-people.com                        | 2606:4700:20::681a:224                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                |
| 28   | toy-people.com                        | 2606:4700:20::681a:324                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                |
| 29   | toy-people.com                        | 2606:4700:20::ac43:4812               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable               |
| 32   | freeyx.cloudflare88.eu.org            | 2606:4700:3010:bf:5dba:fa1d:5993:9cf8 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:bf:5dba:fa1d:5993:9cf8]:443: connect: network is unreachable |
| 33   | freeyx.cloudflare88.eu.org            | 2606:4700:3009:0:72:9d2c:ac0d:3727    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:0:72:9d2c:ac0d:3727]:443: connect: network is unreachable    |
| 38   | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable               |
| 39   | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable               |
| 40   | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable               |
| 47   | zread.ai                              | 2a06:98c1:3120::3                     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                     |
| 48   | zread.ai                              | 2a06:98c1:3121::3                     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                     |
| 49   | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable             |
| 52   | comicabc.com                          | 2606:4700:3030::ac43:ae15             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable             |
| 53   | comicabc.com                          | 2606:4700:3036::6815:400a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable             |
| 56   | bestcf.030101.xyz                     | 2606:4700:0:c8c9:724e:bf54:61f9:838d  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c8c9:724e:bf54:61f9:838d]:443: connect: network is unreachable  |
| 57   | bestcf.030101.xyz                     | 2606:4700::13:7aa5:8275:4599          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::13:7aa5:8275:4599]:443: connect: network is unreachable          |
| 61   | www.whatismyip.com                    | 2606:4700:20::ac43:4581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable               |
| 62   | www.whatismyip.com                    | 2606:4700:20::681a:c17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                |
| 63   | www.whatismyip.com                    | 2606:4700:20::681a:d17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                |
| 66   | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable             |
| 67   | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable             |
| 69   | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable             |
| 72   | fbi.gov                               | 2606:4700::6810:95f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                  |
| 73   | fbi.gov                               | 2606:4700::6810:94f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                  |
| 77   | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable               |
| 78   | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable               |
| 79   | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable               |
| 86   | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable               |
| 87   | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable               |
| 88   | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable               |
| 98   | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable               |
| 99   | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable               |
| 100  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable               |
| 104  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable               |
| 105  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable               |
| 106  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable               |
| 109  | cf.zhetengsha.eu.org                  | 2a06:98c1:3105::6812:230f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable             |
| 110  | cf.zhetengsha.eu.org                  | 2606:4700:440a::ac40:98f1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable             |
| 116  | ip.sb                                 | 2606:4700:20::681a:d1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                |
| 117  | ip.sb                                 | 2606:4700:20::ac43:4bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable               |
| 118  | ip.sb                                 | 2606:4700:20::681a:c1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                |
| 124  | singapore.com                         | 2606:4700:20::681a:c8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                |
| 125  | singapore.com                         | 2606:4700:20::ac43:4bc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable               |
| 126  | singapore.com                         | 2606:4700:20::681a:d8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                |
| 130  | palera.in                             | 2606:4700:3035::6815:3a48             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable             |
| 131  | palera.in                             | 2606:4700:3032::ac43:9d7a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable             |
| 135  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                |
| 136  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                |
| 137  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable               |
| 140  | ip.gs                                 | 2606:4700:3035::ac43:a01c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable             |
| 141  | ip.gs                                 | 2606:4700:3036::6815:eb0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable              |
| 144  | whatismyipaddress.com                 | 2606:4700::6813:df4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                  |
| 145  | whatismyipaddress.com                 | 2606:4700::6813:de4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                  |
| 149  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable               |
| 150  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable               |
| 151  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable               |
| 153  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 157  | dnschecker.org                        | 2606:4700:20::ac43:49d8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable               |
| 158  | dnschecker.org                        | 2606:4700:20::681a:659                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                |
| 159  | dnschecker.org                        | 2606:4700:20::681a:759                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                |
| 161  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable             |
| 165  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable               |
| 166  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable               |
| 167  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable               |
| 171  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable               |
| 172  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable               |
| 173  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable               |
| 177  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable             |
| 182  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable               |
| 183  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable               |
| 184  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable               |
| 187  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable             |
| 188  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable             |
| 189  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable             |
| 192  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 193  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 196  | www.udemy.com                         | 2606:4700::6810:8eed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                  |
| 197  | www.udemy.com                         | 2606:4700::6810:8fed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                  |
| 198  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable             |
| 201  | tasteatlas.com                        | 2606:4700::6811:2569                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                  |
| 202  | tasteatlas.com                        | 2606:4700::6811:2469                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                  |
| 206  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable             |
| 207  | cf.090227.xyz                         | 2606:4700:4407::ac40:9052             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable             |
| 216  | icook.tw                              | 2606:4700:10::ac42:9e73               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable               |
| 217  | icook.tw                              | 2606:4700:10::6814:1c4a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable               |
| 218  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                |
| 223  | time.is                               | 2606:4700:20::681a:d36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                |
| 224  | time.is                               | 2606:4700:20::ac43:449d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable               |
| 225  | time.is                               | 2606:4700:20::681a:c36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                |
| 227  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable              |
| 230  | www.digitalocean.com                  | 2606:4700::6813:ae44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                  |
| 231  | www.digitalocean.com                  | 2606:4700::6813:ad44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                  |
| 238  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable               |
| 239  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable               |
| 240  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable               |
| 244  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable               |
| 245  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable               |
| 246  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable               |
| 250  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable               |
| 251  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable               |
| 252  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable               |
| 256  | ifconfig.co                           | 2606:4700:3037::6815:365b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable             |
| 257  | ifconfig.co                           | 2606:4700:3030::ac43:a86a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable             |
| 262  | japan.com                             | 2606:4700:20::681a:53c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                |
| 263  | japan.com                             | 2606:4700:20::ac43:465c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable               |
| 264  | japan.com                             | 2606:4700:20::681a:43c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                |
| 273  | www.wto.org                           | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 274  | www.wto.org                           | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 278  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable               |
| 279  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable               |
| 280  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable               |
| 285  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable             |
| 286  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable              |
| 290  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable               |
| 291  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable               |
| 292  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable               |
| 296  | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable               |
| 297  | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable               |
| 298  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable               |
| 309  | www.pcmag.com                         | 2606:4700::6810:1476                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                  |
| 310  | www.pcmag.com                         | 2606:4700::6810:1576                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                  |
| 313  | ipinfo.in                             | 2606:4700:3031::6815:1581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable             |
| 314  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable             |
| 317  | steamdb.info                          | 2606:4700:10::6814:22d4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable               |
| 318  | steamdb.info                          | 2606:4700:10::ac42:affa               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable               |
| 327  | www.gov.ua                            | 2606:4700:3031::6815:1748             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable             |
| 328  | www.gov.ua                            | 2606:4700:3033::ac43:d17f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable             |
| 333  | iplocation.io                         | 2606:4700:20::681a:bde                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                |
| 334  | iplocation.io                         | 2606:4700:20::681a:ade                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                |
| 335  | iplocation.io                         | 2606:4700:20::ac43:4664               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable               |
| 337  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable             |
| 338  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable              |
| 346  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable               |
| 347  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable               |
| 348  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable               |
| 352  | icook.hk                              | 2606:4700:3037::ac43:a168             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable             |
| 353  | icook.hk                              | 2606:4700:3031::6815:5ad2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable             |
| 386  | www.hugedomains.com                   | 2606:4700:20::681a:725                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                |
| 387  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable               |
| 388  | www.hugedomains.com                   | 2606:4700:20::681a:625                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                |
| 405  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable             |
| 406  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable             |
| 411  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable               |
| 412  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable               |
| 413  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable               |
| 417  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable             |
| 418  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable             |
| 422  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable               |
| 423  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable               |
| 424  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable               |
| 430  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable               |
| 431  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable               |
| 432  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable               |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 435  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 440  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 441  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 163 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 166 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 163 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：decker.ns.cloudflare.com (3次),
cris.ns.cloudflare.com (3次), dylan.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 45   | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 82   | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 112  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 147  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 329  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 233  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 289  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 324  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 362  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 427  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 35   | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 243  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 425  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 437  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 215  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 393  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 403  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 399  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 114  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 391  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 51   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 282  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 316  | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 368  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 228  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 308  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 300  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 396  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 320  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 429  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 303  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 404  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 281  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 54   | bestcf.030101.xyz                     | 104.19.61.113   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 354  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 4    | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 94   | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 129  | palera.in                             | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 111  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 284  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 31   | freeyx.cloudflare88.eu.org            | 141.101.121.70  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 95   | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 146  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 378  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 108  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 185  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 242  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 24   | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 97   | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 219  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 220  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 383  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 58   | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 162  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 213  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 302  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 394  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 139  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 160  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 195  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 222  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 343  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 365  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 376  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 397  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 212  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 221  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 261  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 270  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 271  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 276  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 357  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 371  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 46   | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 55   | bestcf.030101.xyz                     | 104.19.42.208   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 65   | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 70   | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 143  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 174  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 186  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 293  | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 304  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 321  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 330  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 336  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 364  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 366  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 408  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 414  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 83   | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 120  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 190  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 214  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 265  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 355  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 374  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 377  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 400  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 407  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 416  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 18   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 19   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 25   | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 36   | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 64   | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 90   | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 91   | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 93   | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 113  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 209  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 226  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 259  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 379  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 384  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 438  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 439  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 34   | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 152  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 253  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 319  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 351  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 361  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 44   | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 68   | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 74   | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 80   | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 115  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 154  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 155  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 176  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 232  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 255  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 260  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 267  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 311  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 359  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 367  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 41   | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 71   | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 170  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 203  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 205  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 305  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 415  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 81   | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 210  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 288  | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 306  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 369  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 389  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 42   | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 30   | freeyx.cloudflare88.eu.org            | 141.101.121.80  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 92   | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 229  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 247  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 322  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 323  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 358  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 15   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 191  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 235  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 339  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 345  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 401  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 107  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 121  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 133  | 456.cloudflare.182682.xyz                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 194  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 204  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 312  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 370  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 381  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 390  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 395  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 156  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 211  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 249  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 409  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 89   | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 134  | 456.cloudflare.182682.xyz                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 269  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 344  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 85   | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 102  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 234  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 299  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 301  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 419  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 76   | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 208  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 332  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 349  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 380  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 392  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 26   | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 237  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 340  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 341  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 372  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 385  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 31 条记录
- **快 (50-100ms)**: 169 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 163 次

### 按协议统计

- **none**: 166 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
