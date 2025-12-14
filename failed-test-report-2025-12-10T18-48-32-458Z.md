# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:48:32
- **数据来源**: connectivity_results-20251210-184832.json
- **总测试数**: 481
- **失败测试数**: 178
- **成功测试数**: 303
- **失败率**: 37.01%
- **平均延迟**: 142.28ms
- **最小延迟**: 75ms
- **最大延迟**: 702ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (96.1%)
- **连接超时: I/O超时**: 7 次 (3.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名                                                             | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | [2606:4700:440b::3e6e:5f06]                                           | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 2    | [2606:4700:83be::11:74f]                                              | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 10   | www.ipget.net                                                         | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 11   | www.ipget.net                                                         | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 14   | www.wto.org                                                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 15   | www.wto.org                                                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 21   | ashton.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 22   | ashton.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 23   | ashton.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 27   | ip.sb                                                                 | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 28   | ip.sb                                                                 | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 29   | ip.sb                                                                 | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 33   | local-aria2-webui.masx200.ddns-ip.net                                 | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 34   | local-aria2-webui.masx200.ddns-ip.net                                 | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 40   | braden.ns.cloudflare.com                                              | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 41   | braden.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 42   | braden.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 46   | dylan.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 47   | dylan.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 48   | dylan.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 51   | [2606:4700:440f::53aa:4126]                                           | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 55   | abdullah.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 56   | abdullah.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 57   | abdullah.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 61   | time.is                                                               | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 62   | time.is                                                               | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 63   | time.is                                                               | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 66   | xn--b6gac.eu.org                                                      | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 67   | xn--b6gac.eu.org                                                      | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 71   | [2606:4700:8de6::5fa2:799e]                                           | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 74   | fbi.gov                                                               | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 75   | fbi.gov                                                               | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 80   | sullivan.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 81   | sullivan.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 82   | sullivan.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 88   | www.gov.ua                                                            | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 89   | www.gov.ua                                                            | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 93   | benedict.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 94   | benedict.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 95   | benedict.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 99   | cf.877771.xyz                                                         | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 100  | cf.877771.xyz                                                         | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 107  | singapore.com                                                         | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 108  | singapore.com                                                         | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 109  | singapore.com                                                         | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 125  | craig.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 126  | craig.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 127  | craig.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 132  | toy-people.com                                                        | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 133  | toy-people.com                                                        | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 134  | toy-people.com                                                        | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 139  | decker.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 140  | decker.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 141  | decker.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 147  | uriah.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 148  | uriah.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 149  | uriah.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 160  | moura.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 161  | moura.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 162  | moura.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 166  | julio.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 167  | julio.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 168  | julio.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 172  | kyree.ns.cloudflare.com                                               | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 173  | kyree.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 174  | kyree.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 179  | rustam.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 180  | rustam.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 181  | rustam.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 187  | wilson.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 188  | wilson.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 189  | wilson.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 193  | pranab.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 194  | pranab.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 195  | pranab.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 198  | icook.hk                                                              | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 199  | icook.hk                                                              | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 202  | palera.in                                                             | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 203  | palera.in                                                             | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 207  | cris.ns.cloudflare.com                                                | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 208  | cris.ns.cloudflare.com                                                | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 209  | cris.ns.cloudflare.com                                                | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 218  | otto.ns.cloudflare.com                                                | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 219  | otto.ns.cloudflare.com                                                | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 220  | otto.ns.cloudflare.com                                                | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 224  | trevor.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 225  | trevor.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 226  | trevor.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 229  | whatismyipaddress.com                                                 | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 230  | whatismyipaddress.com                                                 | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 233  | yx-auto.pages.dev                | 2606:4700:3033::ac43:a162               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable               |
| 234  | yx-auto.pages.dev                | 2606:4700:3034::6815:9e6                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable                |
| 238  | icook.tw                                                              | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 239  | icook.tw                                                              | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 244  | damien.ns.cloudflare.com                                              | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 245  | damien.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 246  | damien.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 251  | bestcf.030101.xyz                                                     | 2606:4700:0:c55a:62a8:85ff:cd2c:c647    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c55a:62a8:85ff:cd2c:c647]:443: connect: network is unreachable    |
| 252  | bestcf.030101.xyz                                                     | 2606:4700:0:cd:9b2d:c8ba:6717:5f84      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:cd:9b2d:c8ba:6717:5f84]:443: connect: network is unreachable      |
| 256  | silkbook.com                                                          | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 257  | silkbook.com                                                          | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 258  | silkbook.com                                                          | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 263  | www.pcmag.com                                                         | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 264  | www.pcmag.com                                                         | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 268  | dnschecker.org                                                        | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 269  | dnschecker.org                                                        | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 270  | dnschecker.org                                                        | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 274  | [2606:4700:4409::5b5b:7758]                                           | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 275  | [2606:4700:4408::18c5:3304]                                           | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 282  | cf.877774.xyz                                                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 283  | cf.877774.xyz                                                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 288  | ip.gs                                                                 | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 289  | ip.gs                                                                 | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 292  | [2606:4700:4403::7357:544f]                                           | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 299  | cf.090227.xyz                                                         | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 300  | cf.090227.xyz                                                         | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 308  | cloudflare-ip.mofashi.ltd                                             | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 309  | cloudflare-ip.mofashi.ltd                                             | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 312  | tasteatlas.com                                                        | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 313  | tasteatlas.com                                                        | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 317  | freeyx.cloudflare88.eu.org                                            | 2606:4700:3009:aa59:4b67:100d:c792:9d91 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:100d:c792:9d91]:443: connect: network is unreachable |
| 318  | [2606:4700:83bd::7d8:2b47]                                            | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 321  | www.udemy.com                                                         | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 322  | www.udemy.com                                                         | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 325  | zread.ai                                                              | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 326  | zread.ai                                                              | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 332  | ipinfo.in                                                             | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 333  | ipinfo.in                                                             | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 337  | stock.hostmonit.com                                                   | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 338  | stock.hostmonit.com                                                   | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 350  | cf.zhetengsha.eu.org                                                  | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 351  | cf.zhetengsha.eu.org                                                  | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 355  | steamdb.info                                                          | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 356  | steamdb.info                                                          | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 359  | ae8a9c24-83de.masx200.ddns-ip.net                                     | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 360  | ae8a9c24-83de.masx200.ddns-ip.net                                     | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 363  | cf.0sm.com                                                            | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 364  | cf.0sm.com                                                            | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 370  | iplocation.io                                                         | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 371  | iplocation.io                                                         | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 372  | iplocation.io                                                         | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 375  | [2606:4700:9add::880:52fc]                                            | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 380  | lewis.ns.cloudflare.com                                               | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 381  | lewis.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 382  | lewis.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 388  | ifconfig.co                                                           | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 389  | ifconfig.co                                                           | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 396  | huxley.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 397  | huxley.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 398  | huxley.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 401  | [2606:4700:964f::6e2c:588e]                                           | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 405  | www.hugedomains.com                                                   | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 406  | www.hugedomains.com                                                   | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 407  | www.hugedomains.com                                                   | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 410  | yx-auto.pages.dev | 2606:4700:3031::ac43:868b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable               |
| 411  | yx-auto.pages.dev | 2606:4700:3030::6815:63c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable                |
| 415  | bowen.ns.cloudflare.com                                               | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 416  | bowen.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 417  | bowen.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 420  | www.digitalocean.com                                                  | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 421  | www.digitalocean.com                                                  | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 425  | www.whatismyip.com                                                    | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 426  | www.whatismyip.com                                                    | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 427  | www.whatismyip.com                                                    | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 430  | comicabc.com                                                          | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 431  | comicabc.com                                                          | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 435  | japan.com                                                             | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 436  | japan.com                                                             | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 437  | japan.com                                                             | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 440  | yx-auto.pages.dev                                                     | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 441  | yx-auto.pages.dev                                                     | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 334  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 470  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 472  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 473  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 474  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 475  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 481  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (96.1%)
- **连接超时**: 7 次 (3.9%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 111.171（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 178 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：ashton.ns.cloudflare.com (3次), ip.sb
(3次), braden.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 87 |
www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare | | 331 |
ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 366 |
104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 428 |
comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 186 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare
| | 241 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 80
| cloudflare | | 284 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 248 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 457 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 206 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 262 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 266 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 422 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 |
✅ 成功 | 84 | cloudflare | | 461 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 |
✅ 成功 | 85 | cloudflare | | 235 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3
| ✅ 成功 | 89 | cloudflare | | 249 | bestcf.030101.xyz | 104.17.179.12 | IPv4 |
h3 | ✅ 成功 | 89 | cloudflare | | 365 | 172.64.154.18 | 172.64.154.18 | IPv4 |
h3 | ✅ 成功 | 90 | cloudflare | | 377 | lewis.ns.cloudflare.com |
108.162.195.159 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare | | 395 |
huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare
| | 143 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 446 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 95
| cloudflare | | 419 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅
成功 | 96 | cloudflare | | 307 | cloudflare-ip.mofashi.ltd | 104.21.72.233 |
IPv4 | h3 | ✅ 成功 | 97 | cloudflare | | 261 | www.pcmag.com | 104.16.20.118 |
IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 361 | cf.0sm.com | 104.21.7.133 |
IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 362 | cf.0sm.com | 172.67.187.145 |
IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 144 | uriah.ns.cloudflare.com |
108.162.195.194 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 373 | www.4chan.org
| 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 136 |
decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 103 |
cloudflare | | 413 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅
成功 | 104 | cloudflare | | 414 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4
| h3 | ✅ 成功 | 104 | cloudflare | | 255 | silkbook.com | 104.26.8.160 | IPv4 |
h3 | ✅ 成功 | 105 | cloudflare | | 122 | craig.ns.cloudflare.com |
108.162.195.192 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 150 |
eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 335
| stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 343 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare
| | 124 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 171 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅
成功 | 107 | cloudflare | | 190 | pranab.ns.cloudflare.com | 108.162.195.199 |
IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 286 | ip.gs | 104.21.14.176 | IPv4 |
h3 | ✅ 成功 | 107 | cloudflare | | 316 | freeyx.cloudflare88.eu.org |
141.101.120.120 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 340 | ct.877774.xyz
| 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 83 | ipv4.ip.sb |
104.26.12.31 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 142 |
www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | |
197 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 204
| cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 108 |
cloudflare | | 243 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 434 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功
| 108 | cloudflare | | 111 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 131 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 176 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 |
h3 | ✅ 成功 | 109 | cloudflare | | 352 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3
| ✅ 成功 | 109 | cloudflare | | 358 | ae8a9c24-83de.masx200.ddns-ip.net |
104.21.14.41 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 392 | 104.18.37.40 |
104.18.37.40 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 466 | cmcc.877774.xyz
| 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 76 | 172.64.146.16 |
172.64.146.16 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 196 | icook.hk |
104.21.90.210 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 259 | www.visa.cn |
162.159.153.2 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 464 | cmcc.877774.xyz
| 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 45 |
dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare
| | 178 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 111 |
cloudflare | | 214 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 |
111 | cloudflare | | 408 |
yx-auto.pages.dev |
104.21.6.60 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 145 |
uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 154 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 330 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 347 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 112
| cloudflare | | 8 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 113 |
cloudflare | | 155 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 113
| cloudflare | | 156 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 |
113 | cloudflare | | 277 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 279 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 281 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 293 | 172.64.38.15 | 172.64.38.15 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 346 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 |
✅ 成功 | 113 | cloudflare | | 357 | ae8a9c24-83de.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 386 | ifconfig.co |
104.21.54.91 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 49 | 104.17.142.12 |
104.17.142.12 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare | | 104 | singapore.com |
104.26.12.140 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare | | 32 |
local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 115
| cloudflare | | 183 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 115
| cloudflare | | 349 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅
成功 | 115 | cloudflare | | 73 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功
| 116 | cloudflare | | 90 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 240 | 104.18.78.214 | 104.18.78.214 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 280 | cf.877774.xyz | 104.18.41.190 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 304 | asia.877774.xyz | 104.17.142.146 |
IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 480 | cfip.xxxxxxxx.tk |
190.93.244.201 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 117 | cu.877774.xyz
| 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare | | 123 |
craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 117 |
cloudflare | | 336 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功
| 117 | cloudflare | | 342 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功
| 117 | cloudflare | | 459 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅
成功 | 117 | cloudflare | | 59 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 |
118 | cloudflare | | 215 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3
| ✅ 成功 | 118 | cloudflare | | 439 | yx-auto.pages.dev | 172.66.47.112 | IPv4
| h3 | ✅ 成功 | 118 | cloudflare | | 105 | singapore.com | 104.26.13.140 | IPv4
| h3 | ✅ 成功 | 119 | cloudflare | | 128 | 172.64.159.6 | 172.64.159.6 | IPv4 |
h3 | ✅ 成功 | 119 | cloudflare | | 424 | www.whatismyip.com | 104.26.12.23 |
IPv4 | h3 | ✅ 成功 | 119 | cloudflare | | 455 | cmcc.877774.xyz | 104.16.149.8
| IPv4 | h3 | ✅ 成功 | 119 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 23 条记录
- **正常 (100-200ms)**: 77 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 178 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
