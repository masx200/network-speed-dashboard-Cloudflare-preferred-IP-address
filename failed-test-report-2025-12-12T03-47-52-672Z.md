# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 03:47:52
- **数据来源**: connectivity_results-20251212-034752.json
- **总测试数**: 471
- **失败测试数**: 177
- **成功测试数**: 294
- **失败率**: 37.58%
- **平均延迟**: 118.26ms
- **最小延迟**: 49ms
- **最大延迟**: 728ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 173 次 (97.7%)
- **连接超时: I/O超时**: 4 次 (2.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名                                                             | 目标IP                                | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                              |
| ---- | --------------------------------------------------------------------- | ------------------------------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------- |
| 4    | comicabc.com                                                          | 2606:4700:3030::ac43:ae15             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable             |
| 5    | comicabc.com                                                          | 2606:4700:3036::6815:400a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable             |
| 8    | www.ipget.net                                                         | 2606:4700:3031::ac43:cf1a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable             |
| 9    | www.ipget.net                                                         | 2606:4700:3036::6815:fd4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable              |
| 12   | www.pcmag.com                                                         | 2606:4700::6810:1476                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                  |
| 13   | www.pcmag.com                                                         | 2606:4700::6810:1576                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                  |
| 17   | www.whatismyip.com                                                    | 2606:4700:20::681a:c17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                |
| 18   | www.whatismyip.com                                                    | 2606:4700:20::ac43:4581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable               |
| 19   | www.whatismyip.com                                                    | 2606:4700:20::681a:d17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                |
| 22   | yx-auto.pages.dev                                         | 2606:4700:3034::ac43:97cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::ac43:97cf]:443: connect: network is unreachable             |
| 23   | yx-auto.pages.dev                                         | 2606:4700:3031::6815:49fa             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:49fa]:443: connect: network is unreachable             |
| 27   | trevor.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c9a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable               |
| 28   | trevor.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c39a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable               |
| 29   | trevor.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:239a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable               |
| 33   | wilson.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c6e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable               |
| 34   | wilson.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c36e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable               |
| 35   | wilson.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:236e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable               |
| 38   | steamdb.info                                                          | 2606:4700:10::6814:22d4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable               |
| 39   | steamdb.info                                                          | 2606:4700:10::ac42:affa               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable               |
| 42   | ipinfo.in                                                             | 2606:4700:3031::6815:1581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable             |
| 43   | ipinfo.in                                                             | 2606:4700:3037::ac43:c6cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable             |
| 46   | www.gov.ua                                                            | 2606:4700:3031::6815:1748             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable             |
| 47   | www.gov.ua                                                            | 2606:4700:3033::ac43:d17f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable             |
| 57   | iplocation.io                                                         | 2606:4700:20::681a:ade                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                |
| 58   | iplocation.io                                                         | 2606:4700:20::681a:bde                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                |
| 59   | iplocation.io                                                         | 2606:4700:20::ac43:4664               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable               |
| 62   | [2606:4700:9add::880:52fc]                                            | 2606:4700:9add::880:52fc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable              |
| 70   | huxley.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cbc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable               |
| 71   | huxley.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable               |
| 72   | huxley.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable               |
| 75   | icook.hk                                                              | 2606:4700:3037::ac43:a168             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable             |
| 76   | icook.hk                                                              | 2606:4700:3031::6815:5ad2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable             |
| 79   | cf.0sm.com                                                            | 2606:4700:3032::6815:785              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable              |
| 80   | cf.0sm.com                                                            | 2606:4700:3037::ac43:bb91             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable             |
| 91   | [2606:4700:8de6::5fa2:799e]                                           | 2606:4700:8de6::5fa2:799e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable             |
| 95   | www.hugedomains.com                                                   | 2606:4700:20::681a:725                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                |
| 96   | www.hugedomains.com                                                   | 2606:4700:20::ac43:46bf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable               |
| 97   | www.hugedomains.com                                                   | 2606:4700:20::681a:625                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                |
| 101  | sullivan.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ca1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable               |
| 102  | sullivan.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable               |
| 103  | sullivan.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable               |
| 113  | craig.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cc0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable               |
| 114  | craig.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable               |
| 115  | craig.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable               |
| 131  | yx-auto.pages.dev | 2606:4700:3031::ac43:868b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable             |
| 132  | yx-auto.pages.dev | 2606:4700:3030::6815:63c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable              |
| 136  | pranab.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cc7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable               |
| 137  | pranab.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable               |
| 138  | pranab.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable               |
| 146  | toy-people.com                                                        | 2606:4700:20::681a:324                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                |
| 147  | toy-people.com                                                        | 2606:4700:20::681a:224                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                |
| 148  | toy-people.com                                                        | 2606:4700:20::ac43:4812               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable               |
| 152  | cris.ns.cloudflare.com                                                | 2606:4700:58::a29f:2cca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable               |
| 153  | cris.ns.cloudflare.com                                                | 2803:f800:50::6ca2:c3ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable               |
| 154  | cris.ns.cloudflare.com                                                | 2a06:98c1:50::ac40:23ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable               |
| 183  | cf.877771.xyz                                                         | 2606:4700:3033::ac43:98b7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable             |
| 184  | cf.877771.xyz                                                         | 2606:4700:3033::6815:50b4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable             |
| 188  | kyree.ns.cloudflare.com                                               | 2606:4700:58::a29f:2ccf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable               |
| 189  | kyree.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable               |
| 190  | kyree.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable               |
| 194  | decker.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c9b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable               |
| 195  | decker.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c39b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable               |
| 196  | decker.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:239b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable               |
| 200  | dylan.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cbb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable               |
| 201  | dylan.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable               |
| 202  | dylan.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable               |
| 205  | zread.ai                                                              | 2606:4700:3033::6815:4cf0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable             |
| 206  | zread.ai                                                              | 2606:4700:3032::ac43:ca4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable             |
| 209  | bestcf.030101.xyz                                                     | 2606:4700::55f3:1b80:3ac2:86a1        | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::55f3:1b80:3ac2:86a1]:443: connect: network is unreachable        |
| 210  | bestcf.030101.xyz                                                     | 2606:4700:0:9d:c8da:8ab5:e2e1:1e34    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:9d:c8da:8ab5:e2e1:1e34]:443: connect: network is unreachable    |
| 214  | cf.877774.xyz                                                         | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 215  | cf.877774.xyz                                                         | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 220  | [2606:4700:4409::5b5b:7758]                                           | 2606:4700:4409::5b5b:7758             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable             |
| 223  | fbi.gov                                                               | 2606:4700::6810:94f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                  |
| 224  | fbi.gov                                                               | 2606:4700::6810:95f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                  |
| 228  | braden.ns.cloudflare.com                                              | 2606:4700:58::a29f:2ca9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable               |
| 229  | braden.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable               |
| 230  | braden.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable               |
| 233  | xn--b6gac.eu.org                                                      | 2606:4700:3035::6815:5a4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable             |
| 234  | xn--b6gac.eu.org                                                      | 2606:4700:3037::ac43:99fd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable             |
| 237  | freeyx.cloudflare88.eu.org                                            | 2606:4700:3010:bf:5dba:fabf:8068:e072 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:bf:5dba:fabf:8068:e072]:443: connect: network is unreachable |
| 239  | [2606:4700:440f::53aa:4126]                                           | 2606:4700:440f::53aa:4126             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable             |
| 243  | bowen.ns.cloudflare.com                                               | 2606:4700:58::a29f:2c53               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable               |
| 244  | bowen.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable               |
| 245  | bowen.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:2353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable               |
| 249  | moura.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cd9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable               |
| 250  | moura.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable               |
| 251  | moura.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable               |
| 255  | time.is                                                               | 2606:4700:20::681a:d36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                |
| 256  | time.is                                                               | 2606:4700:20::681a:c36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                |
| 257  | time.is                                                               | 2606:4700:20::ac43:449d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable               |
| 260  | cloudflare-ip.mofashi.ltd                                             | 2606:4700:3037::6815:48e9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable             |
| 261  | cloudflare-ip.mofashi.ltd                                             | 2606:4700:3037::ac43:9bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable             |
| 264  | cf.zhetengsha.eu.org                                                  | 2606:4700:440a::ac40:98f1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable             |
| 265  | cf.zhetengsha.eu.org                                                  | 2a06:98c1:3105::6812:230f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable             |
| 269  | rustam.ns.cloudflare.com                                              | 2606:4700:58::a29f:2c94               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable               |
| 270  | rustam.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable               |
| 271  | rustam.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:2394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable               |
| 281  | ip.sb                                                                 | 2606:4700:20::681a:d1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                |
| 282  | ip.sb                                                                 | 2606:4700:20::ac43:4bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable               |
| 283  | ip.sb                                                                 | 2606:4700:20::681a:c1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                |
| 288  | palera.in                                                             | 2606:4700:3035::6815:3a48             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable             |
| 289  | palera.in                                                             | 2606:4700:3032::ac43:9d7a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable             |
| 293  | benedict.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ccd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable               |
| 294  | benedict.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable               |
| 295  | benedict.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable               |
| 302  | singapore.com                                                         | 2606:4700:20::ac43:4bc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable               |
| 303  | singapore.com                                                         | 2606:4700:20::681a:d8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                |
| 304  | singapore.com                                                         | 2606:4700:20::681a:c8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                |
| 307  | ip.gs                                                                 | 2606:4700:3035::ac43:a01c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable             |
| 308  | ip.gs                                                                 | 2606:4700:3036::6815:eb0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable              |
| 312  | silkbook.com                                                          | 2606:4700:20::681a:9a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                |
| 313  | silkbook.com                                                          | 2606:4700:20::ac43:4bd0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable               |
| 314  | silkbook.com                                                          | 2606:4700:20::681a:8a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                |
| 318  | whatismyipaddress.com                                                 | 2606:4700::6813:de4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                  |
| 319  | whatismyipaddress.com                                                 | 2606:4700::6813:df4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                  |
| 322  | cf.090227.xyz                                                         | 2606:4700:4407::ac40:9052             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable             |
| 323  | cf.090227.xyz                                                         | 2a06:98c1:310d::6812:2bae             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable             |
| 327  | dnschecker.org                                                        | 2606:4700:20::681a:659                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                |
| 328  | dnschecker.org                                                        | 2606:4700:20::681a:759                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                |
| 329  | dnschecker.org                                                        | 2606:4700:20::ac43:49d8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable               |
| 333  | ashton.ns.cloudflare.com                                              | 2606:4700:58::a29f:2cad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable               |
| 334  | ashton.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable               |
| 335  | ashton.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable               |
| 338  | local-aria2-webui.masx200.ddns-ip.net                                 | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 339  | local-aria2-webui.masx200.ddns-ip.net                                 | 2606:4700:3030::6815:e29              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable              |
| 343  | julio.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cd1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable               |
| 344  | julio.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable               |
| 345  | julio.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable               |
| 346  | [2606:4700:964f::6e2c:588e]                                           | 2606:4700:964f::6e2c:588e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable             |
| 351  | ae8a9c24-83de.masx200.ddns-ip.net                                     | 2606:4700:3030::6815:e29              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable              |
| 352  | ae8a9c24-83de.masx200.ddns-ip.net                                     | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 358  | uriah.ns.cloudflare.com                                               | 2606:4700:58::a29f:2cc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable               |
| 359  | uriah.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c3c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable               |
| 360  | uriah.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:23c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable               |
| 361  | [2606:4700:440b::3e6e:5f06]                                           | 2606:4700:440b::3e6e:5f06             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable             |
| 366  | yx-auto.pages.dev                                                     | 2606:4700:310c::ac42:2c90             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable             |
| 367  | yx-auto.pages.dev                                                     | 2606:4700:310c::ac42:2f70             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable             |
| 369  | [2606:4700:4403::7357:544f]                                           | 2606:4700:4403::7357:544f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable             |
| 372  | yx-auto.pages.dev                | 2606:4700:3033::ac43:a162             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable             |
| 373  | yx-auto.pages.dev                | 2606:4700:3034::6815:9e6              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable              |
| 376  | www.udemy.com                                                         | 2606:4700::6810:8eed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                  |
| 377  | www.udemy.com                                                         | 2606:4700::6810:8fed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                  |
| 378  | [2606:4700:4408::18c5:3304]                                           | 2606:4700:4408::18c5:3304             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable             |
| 381  | icook.tw                                                              | 2606:4700:10::ac42:9e73               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable               |
| 382  | icook.tw                                                              | 2606:4700:10::6814:1c4a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable               |
| 385  | tasteatlas.com                                                        | 2606:4700::6811:2469                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                  |
| 386  | tasteatlas.com                                                        | 2606:4700::6811:2569                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                  |
| 391  | www.digitalocean.com                                                  | 2606:4700::6813:ae44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                  |
| 392  | www.digitalocean.com                                                  | 2606:4700::6813:ad44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                  |
| 400  | otto.ns.cloudflare.com                                                | 2606:4700:58::a29f:2c87               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable               |
| 401  | otto.ns.cloudflare.com                                                | 2803:f800:50::6ca2:c387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable               |
| 402  | otto.ns.cloudflare.com                                                | 2a06:98c1:50::ac40:2387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable               |
| 403  | [2606:4700:83be::11:74f]                                              | 2606:4700:83be::11:74f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                |
| 406  | ifconfig.co                                                           | 2606:4700:3037::6815:365b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable             |
| 407  | ifconfig.co                                                           | 2606:4700:3030::ac43:a86a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable             |
| 412  | damien.ns.cloudflare.com                                              | 2606:4700:58::a29f:2ca8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable               |
| 413  | damien.ns.cloudflare.com                                              | 2803:f800:50::6ca2:c3a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable               |
| 414  | damien.ns.cloudflare.com                                              | 2a06:98c1:50::ac40:23a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable               |
| 415  | [2606:4700:83bd::7d8:2b47]                                            | 2606:4700:83bd::7d8:2b47              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable              |
| 420  | lewis.ns.cloudflare.com                                               | 2606:4700:58::a29f:2c9f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable               |
| 421  | lewis.ns.cloudflare.com                                               | 2803:f800:50::6ca2:c39f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable               |
| 422  | lewis.ns.cloudflare.com                                               | 2a06:98c1:50::ac40:239f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable               |
| 427  | japan.com                                                             | 2606:4700:20::681a:43c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                |
| 428  | japan.com                                                             | 2606:4700:20::681a:53c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                |
| 429  | japan.com                                                             | 2606:4700:20::ac43:465c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable               |
| 433  | abdullah.ns.cloudflare.com                                            | 2606:4700:58::a29f:2ccb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable               |
| 434  | abdullah.ns.cloudflare.com                                            | 2803:f800:50::6ca2:c3cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable               |
| 435  | abdullah.ns.cloudflare.com                                            | 2a06:98c1:50::ac40:23cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable               |
| 438  | stock.hostmonit.com                                                   | 2606:4700:3033::ac43:bbfb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable             |
| 439  | stock.hostmonit.com                                                   | 2606:4700:3037::6815:7c1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable              |
| 442  | www.wto.org                                                           | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 443  | www.wto.org                                                           | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 456  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 457  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 458  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 465  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (97.7%)
- **连接超时**: 4 次 (2.3%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 177 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：www.whatismyip.com (3次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 430 |
abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 49 |
cloudflare | | 325 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 161 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 309 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 55 |
cloudflare | | 444 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 56
| cloudflare | | 468 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 |
58 | cloudflare | | 109 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 59
| cloudflare | | 169 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 |
64 | cloudflare | | 142 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 |
65 | cloudflare | | 375 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 |
65 | cloudflare | | 340 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3
| ✅ 成功 | 69 | cloudflare | | 26 | trevor.ns.cloudflare.com | 172.64.35.154 |
IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 83 | ct.877774.xyz | 172.64.229.185 |
IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 321 | cf.090227.xyz | 172.64.144.82 |
IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 389 | www.digitalocean.com |
104.19.174.68 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 182 | cf.877771.xyz |
104.21.80.180 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare | | 432 |
abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 71 |
cloudflare | | 168 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 72
| cloudflare | | 371 | yx-auto.pages.dev |
172.67.161.98 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare | | 44 | www.gov.ua |
104.21.23.72 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 285 | www.glassdoor.com
| 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 393 | 104.17.142.12 |
104.17.142.12 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 16 |
www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | |
124 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | |
330 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 77 |
cloudflare | | 349 | ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4 |
h3 | ✅ 成功 | 77 | cloudflare | | 436 | stock.hostmonit.com | 104.21.7.193 |
IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 3 | comicabc.com | 172.67.174.21 |
IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 77 | cf.0sm.com | 104.21.7.133 | IPv4
| h3 | ✅ 成功 | 78 | cloudflare | | 156 | cmcc.877774.xyz | 104.16.149.11 |
IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 354 | 104.18.14.76 | 104.18.14.76 |
IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 172 | cmcc.877774.xyz | 104.16.149.1 |
IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 175 | cmcc.877774.xyz | 104.16.149.4 |
IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 203 | zread.ai | 104.21.76.240 | IPv4
| h3 | ✅ 成功 | 79 | cloudflare | | 232 | xn--b6gac.eu.org | 172.67.153.253 |
IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 298 | 162.159.36.104 | 162.159.36.104
| IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 129 |
yx-auto.pages.dev |
104.21.6.60 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 252 | time.is |
172.67.68.157 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 279 | ip.sb |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 464 | cfip.xxxxxxxx.tk
| 104.18.228.35 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 74 | icook.hk |
172.67.161.104 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 163 | cmcc.877774.xyz
| 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 461 |
cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | |
395 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare | |
408 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare | |
448 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare | |
460 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare
| | 32 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 40 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 217 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 399 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅
成功 | 83 | cloudflare | | 423 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅
成功 | 83 | cloudflare | | 64 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 278 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 84
| cloudflare | | 348 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 446 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 84
| cloudflare | | 130 |
yx-auto.pages.dev |
172.67.134.139 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 197 |
dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 85 |
cloudflare | | 6 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 87 |
cloudflare | | 104 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 87
| cloudflare | | 180 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 87
| cloudflare | | 253 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 87 |
cloudflare | | 316 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功
| 87 | cloudflare | | 379 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 87
| cloudflare | | 2 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 98 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 |
✅ 成功 | 88 | cloudflare | | 126 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 |
✅ 成功 | 88 | cloudflare | | 310 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅
成功 | 88 | cloudflare | | 396 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅
成功 | 88 | cloudflare | | 15 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 |
✅ 成功 | 89 | cloudflare | | 240 | bowen.ns.cloudflare.com | 108.162.195.83 |
IPv4 | h3 | ✅ 成功 | 89 | cloudflare | | 305 | ip.gs | 104.21.14.176 | IPv4 |
h3 | ✅ 成功 | 89 | cloudflare | | 66 | 104.18.254.88 | 104.18.254.88 | IPv4 |
h3 | ✅ 成功 | 90 | cloudflare | | 155 | cmcc.877774.xyz | 104.16.149.10 | IPv4
| h3 | ✅ 成功 | 90 | cloudflare | | 276 | saas.sin.fan | 162.159.36.20 | IPv4 |
h3 | ✅ 成功 | 90 | cloudflare | | 140 | na.877774.xyz | 104.19.74.233 | IPv4 |
h3 | ✅ 成功 | 91 | cloudflare | | 219 | asia.877774.xyz | 104.17.142.146 | IPv4
| h3 | ✅ 成功 | 91 | cloudflare | | 246 | moura.ns.cloudflare.com |
108.162.195.217 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare | | 326 | dnschecker.org
| 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare | | 364 |
yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare | |
390 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 91 |
cloudflare | | 7 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 61 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 81 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 94 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 |
92 | cloudflare | | 121 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 |
92 | cloudflare | | 280 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 69 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 93 | cloudflare | | 185 | kyree.ns.cloudflare.com | 108.162.195.207 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 394 | 172.67.79.211 | 172.67.79.211 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 85 | ct.877774.xyz | 172.64.229.217 |
IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 141 | na.877774.xyz | 104.18.38.235 |
IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 336 |
local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 |
94 | cloudflare | | 452 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅
成功 | 94 | cloudflare | | 417 | lewis.ns.cloudflare.com | 108.162.195.159 |
IPv4 | h3 | ✅ 成功 | 95 | cloudflare | | 51 | ipv4.ip.sb | 104.26.13.31 | IPv4
| h3 | ✅ 成功 | 96 | cloudflare | | 207 | bestcf.030101.xyz | 104.16.149.55 |
IPv4 | h3 | ✅ 成功 | 96 | cloudflare | | 380 | icook.tw | 104.20.28.74 | IPv4 |
h3 | ✅ 成功 | 96 | cloudflare | | 404 | ifconfig.co | 172.67.168.106 | IPv4 |
h3 | ✅ 成功 | 96 | cloudflare | | 419 | lewis.ns.cloudflare.com | 172.64.35.159
| IPv4 | h3 | ✅ 成功 | 96 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 99 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 177 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
