# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:48:06
- **数据来源**: connectivity_results-20251210-184806.json
- **总测试数**: 483
- **失败测试数**: 179
- **成功测试数**: 304
- **失败率**: 37.06%
- **平均延迟**: 139.14ms
- **最小延迟**: 62ms
- **最大延迟**: 535ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 172 次 (96.1%)
- **连接超时: I/O超时**: 7 次 (3.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (172 次测试)

| 序号 | 主机/域名                             | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | ------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 2    | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 10   | bestcf.030101.xyz                     | 2606:4700:0:c55a:62a8:85ff:cd2c:c647    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c55a:62a8:85ff:cd2c:c647]:443: connect: network is unreachable    |
| 11   | bestcf.030101.xyz                     | 2606:4700:0:cd:9b2d:c8ba:6717:5f84      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:cd:9b2d:c8ba:6717:5f84]:443: connect: network is unreachable      |
| 14   | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 15   | www.pcmag.com                         | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 16   | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 17   | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 21   | dnschecker.org                        | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 22   | dnschecker.org                        | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 23   | dnschecker.org                        | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 27   | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 28   | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 29   | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 32   | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 33   | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 42   | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 43   | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 44   | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 45   | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 54   | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 55   | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 60   | cf.877774.xyz                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 61   | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 64   | tasteatlas.com                        | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 65   | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 69   | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 73   | www.udemy.com                         | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 74   | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 77   | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 78   | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 84   | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 85   | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 88   | cf.877771.xyz                         | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 89   | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 93   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 94   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 97   | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 98   | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 101  | ipinfo.in                             | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 102  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 106  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:1931:ffb6:9dbb:5ba4:26eb | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:1931:ffb6:9dbb:5ba4:26eb]:443: connect: network is unreachable |
| 107  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:e070:5d82:552a:87b5:e42c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:e070:5d82:552a:87b5:e42c]:443: connect: network is unreachable |
| 111  | steamdb.info                          | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 112  | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 117  | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 118  | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 119  | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 122  | cf.0sm.com                            | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 123  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 126  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 127  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 130  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 136  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 137  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 154  | ifconfig.co                           | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 155  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 159  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 160  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 161  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 162  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 167  | yx-auto.pages.dev                     | 2606:4700:3031::ac43:868b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable               |
| 168  | yx-auto.pages.dev                     | 2606:4700:3030::6815:63c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable                |
| 172  | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 173  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 174  | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 178  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 179  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 180  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 183  | www.digitalocean.com                  | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 184  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 188  | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 189  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 190  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 194  | comicabc.com                          | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 195  | comicabc.com                          | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 199  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 200  | japan.com                             | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 201  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 205  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 206  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 210  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 211  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 212  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 216  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 217  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 220  | www.ipget.net                         | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 221  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 228  | ip.sb                                 | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 229  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 230  | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 245  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 246  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 247  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 251  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 252  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 253  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 283  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 284  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 285  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 288  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 289  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 296  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 297  | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 298  | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 299  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 304  | fbi.gov                               | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 305  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 313  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 314  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 315  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 316  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 319  | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 320  | www.gov.ua                            | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 325  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 326  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 327  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 333  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 334  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 335  | singapore.com                         | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 340  | toy-people.com                        | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 341  | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 342  | toy-people.com                        | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 349  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 350  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 351  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 356  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 357  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 358  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 369  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 370  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 371  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 380  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 381  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 382  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 388  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 389  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 390  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 394  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 395  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 396  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 401  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 402  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 403  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 406  | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 407  | icook.hk                              | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 412  | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 413  | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 414  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 418  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 419  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 420  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 424  | palera.in                             | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 425  | palera.in                             | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 429  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 430  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 431  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 440  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 441  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 442  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 445  | yx-auto.pages.dev                     | 2606:4700:3033::ac43:a162               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable               |
| 446  | yx-auto.pages.dev                     | 2606:4700:3034::6815:9e6                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable                |
| 450  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 451  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 452  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 455  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 456  | whatismyipaddress.com                 | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 459  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 460  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 466  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 467  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 468  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 472  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 477  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 479  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 480  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 481  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 482  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 483  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 172 次 (96.1%)
- **连接超时**: 7 次 (3.9%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 121.188（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 179 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 172 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：dnschecker.org (3次),
ashton.ns.cloudflare.com (3次), 456.cloudflare.182682.xyz
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 110 |
steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | | 259 |
cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 233
| cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | | 362 |
eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | | 51 |
172.64.229.249 | 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 186
| www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | |
458 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 250
| dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 70 |
cloudflare | | 443 | yx-auto.pages.dev | 104.21.9.230 | IPv4 | h3 | ✅ 成功 | 70
| cloudflare | | 263 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 71
| cloudflare | | 355 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅
成功 | 75 | cloudflare | | 426 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4
| h3 | ✅ 成功 | 75 | cloudflare | | 377 | moura.ns.cloudflare.com |
108.162.195.217 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 405 | icook.hk |
172.67.161.104 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 433 | www.okcupid.com
| 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 256 | cmcc.877774.xyz
| 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 337 | toy-people.com
| 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 187 |
www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare | |
453 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 301 | 172.64.153.172 | 172.64.153.172 | IPv4 | h3 | ✅ 成功 | 79
| cloudflare | | 238 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 270 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 203 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 |
85 | cloudflare | | 432 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功
| 85 | cloudflare | | 191 | 172.64.157.120 | 172.64.157.120 | IPv4 | h3 | ✅
成功 | 88 | cloudflare | | 294 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 |
88 | cloudflare | | 237 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 |
90 | cloudflare | | 302 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 90 |
cloudflare | | 196 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 165 | yx-auto.pages.dev | 104.21.6.60 | IPv4 | h3 | ✅ 成功 | 93
| cloudflare | | 57 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3 | ✅ 成功 | 94 |
cloudflare | | 50 | 172.64.148.15 | 172.64.148.15 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 140 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 291 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 146 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 261 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 275 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 411 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅
成功 | 97 | cloudflare | | 31 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 239 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 321 | 172.64.146.16 | 172.64.146.16 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 38 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 |
✅ 成功 | 100 | cloudflare | | 142 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3
| ✅ 成功 | 100 | cloudflare | | 386 | kyree.ns.cloudflare.com | 162.159.44.207
| IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 129 | www.4chan.org |
104.16.229.229 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 204 |
yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | |
218 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | |
317 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 447
| trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 101 |
cloudflare | | 81 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 102
| cloudflare | | 255 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 |
102 | cloudflare | | 387 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 |
✅ 成功 | 102 | cloudflare | | 473 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 |
h3 | ✅ 成功 | 102 | cloudflare | | 359 | 172.64.33.67 | 172.64.33.67 | IPv4 |
h3 | ✅ 成功 | 103 | cloudflare | | 9 | bestcf.030101.xyz | 104.17.179.12 | IPv4
| h3 | ✅ 成功 | 104 | cloudflare | | 104 | freeyx.cloudflare88.eu.org |
141.101.120.230 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 115 | iplocation.io
| 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 166 |
yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | |
56 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | |
303 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 26 |
ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 52 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 95 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 |
108 | cloudflare | | 103 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功
| 108 | cloudflare | | 265 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 392 | julio.ns.cloudflare.com | 162.159.44.209 |
IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 436 | www.okcupid.com |
104.16.239.254 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 113 | 172.64.154.18
| 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 171 |
www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | |
373 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 457 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 20
| dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 86
| cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 153
| ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 207 |
bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 331 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 368 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅
成功 | 110 | cloudflare | | 39 | 456.cloudflare.182682.xyz | 172.67.75.208 | IPv4 | h3 | ✅
成功 | 111 | cloudflare | | 134 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 |
h3 | ✅ 成功 | 111 | cloudflare | | 254 | cmcc.877774.xyz | 104.16.148.8 | IPv4
| h3 | ✅ 成功 | 111 | cloudflare | | 385 | kyree.ns.cloudflare.com |
108.162.195.207 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 72 | www.udemy.com
| 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 273 |
cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 366
| uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 18 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 113 |
cloudflare | | 258 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 113
| cloudflare | | 262 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 |
113 | cloudflare | | 148 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功
| 114 | cloudflare | | 224 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功
| 114 | cloudflare | | 12 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功
| 115 | cloudflare | | 145 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅
成功 | 115 | cloudflare | | 417 | pranab.ns.cloudflare.com | 172.64.35.199 |
IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 428 | cris.ns.cloudflare.com |
172.64.35.202 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 470 | www.visa.cn |
162.159.152.2 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 215 |
local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 |
116 | cloudflare | | 374 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功
| 116 | cloudflare | | 391 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 |
h3 | ✅ 成功 | 116 | cloudflare | | 423 | palera.in | 104.21.58.72 | IPv4 | h3 |
✅ 成功 | 116 | cloudflare | | 49 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅
成功 | 117 | cloudflare | | 307 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅
成功 | 117 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 41 条记录
- **正常 (100-200ms)**: 59 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 172 次

### 按协议统计

- **none**: 179 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
