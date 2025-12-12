# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 07:02:34
- **数据来源**: connectivity_results-20251212-070234.json
- **总测试数**: 457
- **失败测试数**: 176
- **成功测试数**: 281
- **失败率**: 38.51%
- **平均延迟**: 81.40ms
- **最小延迟**: 37ms
- **最大延迟**: 742ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 07:02:34
- **IP地址**: 135.232.225.18
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

- **网络不可达: 网络不可达**: 173 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名                                                             | 目标IP                                | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                              |
| ---- | --------------------------------------------------------------------- | ------------------------------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------- |
| 4    | www.ipget.net                                                         | 2606:4700:3036::6815:fd4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable              |
| 5    | www.ipget.net                                                         | 2606:4700:3031::ac43:cf1a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable             |
| 8    | comicabc.com                                                          | 2606:4700:3030::ac43:ae15             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable             |
| 9    | comicabc.com                                                          | 2606:4700:3036::6815:400a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable             |
| 12   | bhr01erx45.inzjddnkdz.de5.net                                         | 2606:4700:3034::ac43:97cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::ac43:97cf]:443: connect: network is unreachable             |
| 13   | bhr01erx45.inzjddnkdz.de5.net                                         | 2606:4700:3031::6815:49fa             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:49fa]:443: connect: network is unreachable             |
| 17   | time.is                                                               | 2606:4700:20::681a:c36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                |
| 18   | time.is                                                               | 2606:4700:20::681a:d36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                |
| 19   | time.is                                                               | 2606:4700:20::ac43:449d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable               |
| 26   | wilson.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c6e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable               |
| 27   | wilson.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c36e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable               |
| 28   | wilson.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:236e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable               |
| 32   | rustam.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c94               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable               |
| 33   | rustam.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable               |
| 34   | rustam.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:2394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable               |
| 38   | moura.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cd9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable               |
| 39   | moura.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable               |
| 40   | moura.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable               |
| 44   | trevor.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c9a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable               |
| 45   | trevor.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c39a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable               |
| 46   | trevor.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:239a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable               |
| 50   | palera.in                                                             | 2606:4700:3035::6815:3a48             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable             |
| 51   | palera.in                                                             | 2606:4700:3032::ac43:9d7a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable             |
| 58   | ip.sb                                                                 | 2606:4700:20::681a:c1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                |
| 59   | ip.sb                                                                 | 2606:4700:20::681a:d1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                |
| 60   | ip.sb                                                                 | 2606:4700:20::ac43:4bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable               |
| 64   | singapore.com                                                         | 2606:4700:20::681a:c8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                |
| 65   | singapore.com                                                         | 2606:4700:20::681a:d8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                |
| 66   | singapore.com                                                         | 2606:4700:20::ac43:4bc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable               |
| 71   | benedict.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ccd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable               |
| 72   | benedict.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable               |
| 73   | benedict.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable               |
| 76   | ip.gs                                                                 | 2606:4700:3035::ac43:a01c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable             |
| 77   | ip.gs                                                                 | 2606:4700:3036::6815:eb0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable              |
| 81   | silkbook.com                                                          | 2606:4700:20::681a:9a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                |
| 82   | silkbook.com                                                          | 2606:4700:20::ac43:4bd0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable               |
| 83   | silkbook.com                                                          | 2606:4700:20::681a:8a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                |
| 86   | whatismyipaddress.com                                                 | 2606:4700::6813:de4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                  |
| 87   | whatismyipaddress.com                                                 | 2606:4700::6813:df4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                  |
| 90   | ae8a9c24-83de.masx200.ddns-ip.net                                     | 2606:4700:3030::6815:e29              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable              |
| 91   | ae8a9c24-83de.masx200.ddns-ip.net                                     | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 95   | dnschecker.org                                                        | 2606:4700:20::681a:659                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                |
| 96   | dnschecker.org                                                        | 2606:4700:20::681a:759                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                |
| 97   | dnschecker.org                                                        | 2606:4700:20::ac43:49d8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable               |
| 98   | [2606:4700:964f::6e2c:588e]                                           | 2606:4700:964f::6e2c:588e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable             |
| 103  | yx-auto.pages.dev                                                     | 2606:4700:310c::ac42:2c90             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable             |
| 104  | yx-auto.pages.dev                                                     | 2606:4700:310c::ac42:2f70             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable             |
| 108  | ashton.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable               |
| 109  | ashton.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable               |
| 110  | ashton.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable               |
| 115  | local-aria2-webui.masx200.ddns-ip.net                                 | 2606:4700:3030::6815:e29              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable              |
| 116  | local-aria2-webui.masx200.ddns-ip.net                                 | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 117  | [2606:4700:440b::3e6e:5f06]                                           | 2606:4700:440b::3e6e:5f06             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable             |
| 121  | julio.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cd1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable               |
| 122  | julio.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable               |
| 123  | julio.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable               |
| 126  | [2606:4700:4403::7357:544f]                                           | 2606:4700:4403::7357:544f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable             |
| 130  | uriah.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable               |
| 131  | uriah.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable               |
| 132  | uriah.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable               |
| 135  | www.udemy.com                                                         | 2606:4700::6810:8eed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                  |
| 136  | www.udemy.com                                                         | 2606:4700::6810:8fed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                  |
| 137  | [2606:4700:4408::18c5:3304]                                           | 2606:4700:4408::18c5:3304             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable             |
| 142  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx201.dpdns.org                | 2606:4700:3034::6815:9e6              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable              |
| 143  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx201.dpdns.org                | 2606:4700:3033::ac43:a162             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable             |
| 146  | tasteatlas.com                                                        | 2606:4700::6811:2569                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                  |
| 147  | tasteatlas.com                                                        | 2606:4700::6811:2469                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                  |
| 152  | www.digitalocean.com                                                  | 2606:4700::6813:ad44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                  |
| 153  | www.digitalocean.com                                                  | 2606:4700::6813:ae44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                  |
| 156  | icook.tw                                                              | 2606:4700:10::ac42:9e73               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable               |
| 157  | icook.tw                                                              | 2606:4700:10::6814:1c4a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable               |
| 160  | [2606:4700:83be::11:74f]                                              | 2606:4700:83be::11:74f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                |
| 166  | otto.ns.cloudflare.com                                                | 2606:4700:58::a29f:2c87               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable               |
| 167  | otto.ns.cloudflare.com                                                | 2803:f800:50::6ca2:c387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable               |
| 168  | otto.ns.cloudflare.com                                                | 2a06:98c1:50::ac40:2387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable               |
| 169  | [2606:4700:83bd::7d8:2b47]                                            | 2606:4700:83bd::7d8:2b47              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable              |
| 175  | damien.ns.cloudflare.com                                              | 2606:4700:58::a29f:2ca8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable               |
| 176  | damien.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable               |
| 177  | damien.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable               |
| 180  | ifconfig.co                                                           | 2606:4700:3030::ac43:a86a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable             |
| 181  | ifconfig.co                                                           | 2606:4700:3037::6815:365b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable             |
| 185  | japan.com                                                             | 2606:4700:20::681a:53c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                |
| 186  | japan.com                                                             | 2606:4700:20::681a:43c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                |
| 187  | japan.com                                                             | 2606:4700:20::ac43:465c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable               |
| 191  | lewis.ns.cloudflare.com                                               | 2606:4700:58::a29f:2c9f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable               |
| 192  | lewis.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c39f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable               |
| 193  | lewis.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:239f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable               |
| 197  | stock.hostmonit.com                                                   | 2606:4700:3033::ac43:bbfb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable             |
| 198  | stock.hostmonit.com                                                   | 2606:4700:3037::6815:7c1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable              |
| 201  | www.wto.org                                                           | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 202  | www.wto.org                                                           | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 210  | abdullah.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ccb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable               |
| 211  | abdullah.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable               |
| 212  | abdullah.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable               |
| 216  | cf.877774.xyz                                                         | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 217  | cf.877774.xyz                                                         | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 222  | www.pcmag.com                                                         | 2606:4700::6810:1576                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                  |
| 223  | www.pcmag.com                                                         | 2606:4700::6810:1476                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                  |
| 227  | ipinfo.in                                                             | 2606:4700:3031::6815:1581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable             |
| 228  | ipinfo.in                                                             | 2606:4700:3037::ac43:c6cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable             |
| 232  | steamdb.info                                                          | 2606:4700:10::6814:22d4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable               |
| 233  | steamdb.info                                                          | 2606:4700:10::ac42:affa               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable               |
| 247  | www.gov.ua                                                            | 2606:4700:3031::6815:1748             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable             |
| 248  | www.gov.ua                                                            | 2606:4700:3033::ac43:d17f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable             |
| 254  | iplocation.io                                                         | 2606:4700:20::681a:ade                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                |
| 255  | iplocation.io                                                         | 2606:4700:20::681a:bde                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                |
| 256  | iplocation.io                                                         | 2606:4700:20::ac43:4664               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable               |
| 263  | huxley.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cbc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable               |
| 264  | huxley.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable               |
| 265  | huxley.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable               |
| 268  | cf.0sm.com                                                            | 2606:4700:3032::6815:785              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable              |
| 269  | cf.0sm.com                                                            | 2606:4700:3037::ac43:bb91             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable             |
| 271  | [2606:4700:9add::880:52fc]                                            | 2606:4700:9add::880:52fc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable              |
| 275  | cf.877771.xyz                                                         | 2606:4700:3033::6815:50b4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable             |
| 276  | cf.877771.xyz                                                         | 2606:4700:3033::ac43:98b7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable             |
| 279  | icook.hk                                                              | 2606:4700:3031::6815:5ad2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable             |
| 280  | icook.hk                                                              | 2606:4700:3037::ac43:a168             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable             |
| 288  | www.hugedomains.com                                                   | 2606:4700:20::ac43:46bf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable               |
| 289  | www.hugedomains.com                                                   | 2606:4700:20::681a:725                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                |
| 290  | www.hugedomains.com                                                   | 2606:4700:20::681a:625                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                |
| 320  | sullivan.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ca1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable               |
| 321  | sullivan.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable               |
| 322  | sullivan.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable               |
| 323  | [2606:4700:8de6::5fa2:799e]                                           | 2606:4700:8de6::5fa2:799e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable             |
| 332  | zread.ai                                                              | 2606:4700:3033::6815:4cf0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable             |
| 333  | zread.ai                                                              | 2606:4700:3032::ac43:ca4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable             |
| 338  | craig.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cc0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable               |
| 339  | craig.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable               |
| 340  | craig.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable               |
| 346  | pranab.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cc7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable               |
| 347  | pranab.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable               |
| 348  | pranab.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable               |
| 354  | 6a507fac-b8e4-45be-b2f0-9310f72c7eca.b.3.b.f.0.7.4.0.1.0.0.2.ip6.arpa | 2606:4700:3031::ac43:868b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable             |
| 355  | 6a507fac-b8e4-45be-b2f0-9310f72c7eca.b.3.b.f.0.7.4.0.1.0.0.2.ip6.arpa | 2606:4700:3030::6815:63c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable              |
| 368  | cris.ns.cloudflare.com                                                | 2606:4700:58::a29f:2cca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable               |
| 369  | cris.ns.cloudflare.com                                                | 2803:f800:50::6ca2:c3ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable               |
| 370  | cris.ns.cloudflare.com                                                | 2a06:98c1:50::ac40:23ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable               |
| 374  | toy-people.com                                                        | 2606:4700:20::681a:224                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                |
| 375  | toy-people.com                                                        | 2606:4700:20::681a:324                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                |
| 376  | toy-people.com                                                        | 2606:4700:20::ac43:4812               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable               |
| 383  | decker.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c9b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable               |
| 384  | decker.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c39b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable               |
| 385  | decker.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:239b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable               |
| 389  | kyree.ns.cloudflare.com                                               | 2606:4700:58::a29f:2ccf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable               |
| 390  | kyree.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable               |
| 391  | kyree.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable               |
| 395  | www.whatismyip.com                                                    | 2606:4700:20::ac43:4581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable               |
| 396  | www.whatismyip.com                                                    | 2606:4700:20::681a:d17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                |
| 397  | www.whatismyip.com                                                    | 2606:4700:20::681a:c17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                |
| 401  | dylan.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cbb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable               |
| 402  | dylan.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable               |
| 403  | dylan.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable               |
| 407  | braden.ns.cloudflare.com                                              | 2606:4700:58::a29f:2ca9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable               |
| 408  | braden.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable               |
| 409  | braden.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable               |
| 412  | bestcf.030101.xyz                                                     | 2606:4700:0:11bf:374c:580d:a8a6:8e5b  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:11bf:374c:580d:a8a6:8e5b]:443: connect: network is unreachable  |
| 413  | bestcf.030101.xyz                                                     | 2606:4700:58:7e7b:edf2:ee1f:c9fe:41e3 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58:7e7b:edf2:ee1f:c9fe:41e3]:443: connect: network is unreachable |
| 415  | freeyx.cloudflare88.eu.org                                            | 2606:4700:3010:0:f5e8:7af2:12d8:5d82  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:f5e8:7af2:12d8:5d82]:443: connect: network is unreachable  |
| 422  | [2606:4700:4409::5b5b:7758]                                           | 2606:4700:4409::5b5b:7758             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable             |
| 426  | bowen.ns.cloudflare.com                                               | 2606:4700:58::a29f:2c53               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable               |
| 427  | bowen.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable               |
| 428  | bowen.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:2353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable               |
| 431  | fbi.gov                                                               | 2606:4700::6810:94f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                  |
| 432  | fbi.gov                                                               | 2606:4700::6810:95f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                  |
| 435  | xn--b6gac.eu.org                                                      | 2606:4700:3037::ac43:99fd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable             |
| 436  | xn--b6gac.eu.org                                                      | 2606:4700:3035::6815:5a4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable             |
| 438  | [2606:4700:440f::53aa:4126]                                           | 2606:4700:440f::53aa:4126             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable             |
| 441  | cloudflare-ip.mofashi.ltd                                             | 2606:4700:3037::6815:48e9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable             |
| 442  | cloudflare-ip.mofashi.ltd                                             | 2606:4700:3037::ac43:9bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable             |
| 448  | cf.zhetengsha.eu.org                                                  | 2a06:98c1:3101::ac40:919e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable             |
| 449  | cf.zhetengsha.eu.org                                                  | 2a06:98c1:3108::6812:2a62             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable             |
| 453  | cf.090227.xyz                                                         | 2606:4700:4407::ac40:9052             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable             |
| 454  | cf.090227.xyz                                                         | 2a06:98c1:310d::6812:2bae             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable             |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 455  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 456  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 457  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 176 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：time.is (3次), wilson.ns.cloudflare.com
(3次), rustam.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                                                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 277  | icook.hk                                                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 184  | japan.com                                                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 204  | eur.877774.xyz                                                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 207  | abdullah.ns.cloudflare.com                                            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 350  | www.visa.cn                                                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 165  | otto.ns.cloudflare.com                                                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 250  | 104.16.45.84                                                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 356  | cu.877774.xyz                                                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 423  | bowen.ns.cloudflare.com                                               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 138  | gamer.com.tw                                                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 257  | shopify.com                                                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 361  | cu.877774.xyz                                                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 220  | www.pcmag.com                                                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 392  | www.whatismyip.com                                                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 430  | fbi.gov                                                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 343  | pranab.ns.cloudflare.com                                              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 304  | cmcc.877774.xyz                                                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 416  | 104.16.223.179                                                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 444  | saas.sin.fan                                                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 327  | www.okcupid.com                                                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 158  | 104.18.37.40                                                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 246  | www.gov.ua                                                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 251  | iplocation.io                                                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 360  | cu.877774.xyz                                                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 94   | dnschecker.org                                                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 278  | icook.hk                                                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 302  | cmcc.877774.xyz                                                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 88   | ae8a9c24-83de.masx200.ddns-ip.net                                     | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 241  | ct.877774.xyz                                                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 316  | cmcc.877774.xyz                                                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 373  | toy-people.com                                                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 183  | japan.com                                                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 245  | www.gov.ua                                                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 252  | iplocation.io                                                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 3    | www.ipget.net                                                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 55   | ip.sb                                                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 56   | ip.sb                                                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 85   | whatismyipaddress.com                                                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 148  | 104.17.142.12                                                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 221  | www.pcmag.com                                                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 293  | cmcc.877774.xyz                                                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 353  | 6a507fac-b8e4-45be-b2f0-9310f72c7eca.b.3.b.f.0.7.4.0.1.0.0.2.ip6.arpa | 104.21.6.60     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 101  | yx-auto.pages.dev                                                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 235  | ipv4.ip.sb                                                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 240  | ct.877774.xyz                                                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 285  | www.hugedomains.com                                                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 306  | cmcc.877774.xyz                                                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 324  | www.okcupid.com                                                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 341  | www.visa.com.sg                                                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 342  | www.visa.com.sg                                                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 351  | www.visa.cn                                                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 421  | asia.877774.xyz                                                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 425  | bowen.ns.cloudflare.com                                               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 141  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx201.dpdns.org                | 172.67.161.98   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 237  | ct.877774.xyz                                                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 239  | ct.877774.xyz                                                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 249  | 104.18.39.196                                                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 305  | cmcc.877774.xyz                                                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 386  | kyree.ns.cloudflare.com                                               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 388  | kyree.ns.cloudflare.com                                               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 15   | time.is                                                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 112  | 172.67.106.26                                                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 134  | www.udemy.com                                                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 209  | abdullah.ns.cloudflare.com                                            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 296  | cmcc.877774.xyz                                                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 334  | 172.67.120.0                                                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 47   | 104.16.61.163                                                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 149  | 172.67.79.211                                                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 174  | damien.ns.cloudflare.com                                              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 215  | cf.877774.xyz                                                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 225  | ipinfo.in                                                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 266  | cf.0sm.com                                                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 308  | cmcc.877774.xyz                                                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 310  | cmcc.877774.xyz                                                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 314  | cmcc.877774.xyz                                                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 326  | www.okcupid.com                                                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 380  | decker.ns.cloudflare.com                                              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 440  | cloudflare-ip.mofashi.ltd                                             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 111  | 104.18.42.26                                                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 231  | steamdb.info                                                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 258  | 172.67.75.172                                                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 298  | cmcc.877774.xyz                                                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 23   | wilson.ns.cloudflare.com                                              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 114  | local-aria2-webui.masx200.ddns-ip.net                                 | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 159  | 198.62.62.4                                                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 345  | pranab.ns.cloudflare.com                                              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 394  | www.whatismyip.com                                                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 414  | freeyx.cloudflare88.eu.org                                            | 141.101.120.15  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 57   | ip.sb                                                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 63   | singapore.com                                                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 139  | gamer.com.tw                                                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 162  | 104.26.13.31                                                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 163  | otto.ns.cloudflare.com                                                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 267  | cf.0sm.com                                                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 35   | moura.ns.cloudflare.com                                               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 80   | silkbook.com                                                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 102  | yx-auto.pages.dev                                                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 145  | tasteatlas.com                                                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 150  | www.digitalocean.com                                                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 182  | japan.com                                                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 20 条记录
- **快 (50-100ms)**: 80 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 176 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
