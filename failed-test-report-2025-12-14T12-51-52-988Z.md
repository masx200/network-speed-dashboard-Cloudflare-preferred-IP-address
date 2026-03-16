# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 20:51:52
- **数据来源**: connectivity_results-20251214-125142.json
- **总测试数**: 450
- **失败测试数**: 168
- **成功测试数**: 282
- **失败率**: 37.33%
- **平均延迟**: 975.92ms
- **最小延迟**: 168ms
- **最大延迟**: 7400ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 20:51:53
- **IP地址**: 101.83.152.37
- **国家/地区**: China (CN)
- **ASN**: AS4812
- **网络组织**: China Telecom Group
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 31.2222, 121.4581
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 161 次 (95.8%)
- **连接超时: I/O超时**: 7 次 (4.2%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (161 次测试)

| 序号 | 主机/域名                             | 目标IP                              | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | ------------------------------------- | ----------------------------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 6    | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable             |
| 7    | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable             |
| 8    | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable             |
| 12   | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable             |
| 13   | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable             |
| 14   | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable             |
| 19   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable           |
| 20   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable           |
| 26   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable           |
| 30   | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable             |
| 31   | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable             |
| 32   | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable             |
| 35   | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable           |
| 36   | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable           |
| 39   | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable           |
| 40   | cf.877774.xyz                         | 2606:4700:4406::ac40:9242           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable           |
| 43   | bestcf.030101.xyz                     | 2606:4700:b:4d5f:ec98:3ef5:721:4971 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:b:4d5f:ec98:3ef5:721:4971]:443: connect: network is unreachable |
| 44   | bestcf.030101.xyz                     | 2606:4700:0:f0:8071:a554:1555:a780  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:f0:8071:a554:1555:a780]:443: connect: network is unreachable  |
| 51   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable             |
| 52   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable             |
| 53   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable             |
| 56   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable           |
| 57   | www.ipget.net                         | 2606:4700:3036::6815:fd4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable            |
| 61   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable             |
| 62   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable             |
| 63   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable             |
| 66   | www.pcmag.com                         | 2606:4700::6810:1576                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                |
| 67   | www.pcmag.com                         | 2606:4700::6810:1476                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                |
| 78   | comicabc.com                          | 2606:4700:3030::ac43:ae15           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable           |
| 79   | comicabc.com                          | 2606:4700:3036::6815:400a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable           |
| 82   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable           |
| 83   | cf.0sm.com                            | 2606:4700:3032::6815:785            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable            |
| 86   | steamdb.info                          | 2606:4700:10::6814:22d4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable             |
| 87   | steamdb.info                          | 2606:4700:10::ac42:affa             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable             |
| 92   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable           |
| 93   | ipinfo.in                             | 2606:4700:3031::6815:1581           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable           |
| 94   | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable           |
| 106  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable             |
| 107  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable             |
| 108  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable             |
| 109  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable            |
| 115  | iplocation.io                         | 2606:4700:20::681a:ade              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable              |
| 116  | iplocation.io                         | 2606:4700:20::681a:bde              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable              |
| 117  | iplocation.io                         | 2606:4700:20::ac43:4664             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable             |
| 121  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable           |
| 122  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable           |
| 126  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable             |
| 127  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable             |
| 128  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable             |
| 131  | www.gov.ua                            | 2606:4700:3031::6815:1748           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable           |
| 132  | www.gov.ua                            | 2606:4700:3033::ac43:d17f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable           |
| 135  | icook.hk                              | 2606:4700:3037::ac43:a168           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable           |
| 136  | icook.hk                              | 2606:4700:3031::6815:5ad2           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable           |
| 140  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable             |
| 141  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable             |
| 142  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable             |
| 147  | www.hugedomains.com                   | 2606:4700:20::681a:725              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable              |
| 148  | www.hugedomains.com                   | 2606:4700:20::681a:625              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable              |
| 149  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable             |
| 155  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable             |
| 156  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable             |
| 157  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable             |
| 159  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable           |
| 162  | fbi.gov                               | 2606:4700::6810:95f4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                |
| 163  | fbi.gov                               | 2606:4700::6810:94f4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                |
| 166  | cf.090227.xyz                         | 2a06:98c1:3101::ac40:919e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable           |
| 167  | cf.090227.xyz                         | 2a06:98c1:3108::6812:2a62           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable           |
| 175  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable             |
| 176  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable             |
| 177  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable             |
| 181  | toy-people.com                        | 2606:4700:20::681a:324              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable              |
| 182  | toy-people.com                        | 2606:4700:20::681a:224              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable              |
| 183  | toy-people.com                        | 2606:4700:20::ac43:4812             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable             |
| 189  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable             |
| 190  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable             |
| 191  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable             |
| 195  | time.is                               | 2606:4700:20::681a:d36              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable              |
| 196  | time.is                               | 2606:4700:20::681a:c36              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable              |
| 197  | time.is                               | 2606:4700:20::ac43:449d             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable             |
| 210  | ip.sb                                 | 2606:4700:20::ac43:4bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable             |
| 211  | ip.sb                                 | 2606:4700:20::681a:c1f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable              |
| 212  | ip.sb                                 | 2606:4700:20::681a:d1f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable              |
| 213  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable           |
| 217  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable             |
| 218  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable             |
| 219  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable             |
| 228  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable             |
| 229  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable             |
| 230  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable             |
| 231  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable           |
| 266  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable           |
| 270  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable             |
| 271  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable             |
| 272  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable             |
| 277  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable             |
| 278  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable             |
| 279  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable             |
| 280  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable           |
| 284  | www.whatismyip.com                    | 2606:4700:20::ac43:4581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable             |
| 285  | www.whatismyip.com                    | 2606:4700:20::681a:c17              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable              |
| 286  | www.whatismyip.com                    | 2606:4700:20::681a:d17              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable              |
| 302  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable           |
| 305  | www.udemy.com                         | 2606:4700::6810:8fed                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                |
| 306  | www.udemy.com                         | 2606:4700::6810:8eed                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                |
| 310  | tasteatlas.com                        | 2606:4700::6811:2569                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                |
| 311  | tasteatlas.com                        | 2606:4700::6811:2469                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                |
| 316  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable             |
| 317  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable             |
| 318  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable             |
| 323  | icook.tw                              | 2606:4700:10::6814:1c4a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable             |
| 324  | icook.tw                              | 2606:4700:10::ac42:9e73             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable             |
| 326  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable              |
| 331  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable             |
| 332  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable             |
| 333  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable             |
| 335  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable            |
| 342  | www.digitalocean.com                  | 2606:4700::6813:ad44                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                |
| 343  | www.digitalocean.com                  | 2606:4700::6813:ae44                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                |
| 347  | www.wto.org                           | 2606:4700:4406::ac40:9242           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable           |
| 348  | www.wto.org                           | 2a06:98c1:3102::6812:29be           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable           |
| 352  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable           |
| 353  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable            |
| 358  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable             |
| 359  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable             |
| 360  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable             |
| 364  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable             |
| 365  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable             |
| 366  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable             |
| 369  | ifconfig.co                           | 2606:4700:3037::6815:365b           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable           |
| 370  | ifconfig.co                           | 2606:4700:3030::ac43:a86a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable           |
| 378  | japan.com                             | 2606:4700:20::681a:53c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable              |
| 379  | japan.com                             | 2606:4700:20::681a:43c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable              |
| 380  | japan.com                             | 2606:4700:20::ac43:465c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable             |
| 385  | singapore.com                         | 2606:4700:20::ac43:4bc2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable             |
| 386  | singapore.com                         | 2606:4700:20::681a:d8c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable              |
| 387  | singapore.com                         | 2606:4700:20::681a:c8c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable              |
| 396  | palera.in                             | 2606:4700:3032::ac43:9d7a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable           |
| 397  | palera.in                             | 2606:4700:3035::6815:3a48           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable           |
| 400  | zread.ai                              | 2606:4700:3032::ac43:ca4e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable           |
| 401  | zread.ai                              | 2606:4700:3033::6815:4cf0           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable           |
| 405  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable              |
| 406  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable             |
| 407  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable              |
| 410  | ip.gs                                 | 2a06:98c1:3120::3                   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                   |
| 411  | ip.gs                                 | 2a06:98c1:3121::3                   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                   |
| 417  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable             |
| 418  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable             |
| 419  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable             |
| 422  | whatismyipaddress.com                 | 2606:4700::6813:df4f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                |
| 423  | whatismyipaddress.com                 | 2606:4700::6813:de4f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                |
| 427  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable             |
| 428  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable             |
| 429  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable             |
| 433  | dnschecker.org                        | 2606:4700:20::681a:659              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable              |
| 434  | dnschecker.org                        | 2606:4700:20::ac43:49d8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable             |
| 435  | dnschecker.org                        | 2606:4700:20::681a:759              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable              |
| 438  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable           |
| 439  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable           |
| 445  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable             |
| 446  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable             |
| 447  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable             |

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 105  | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout |
| 273  | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 293  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 299  | cfip.xxxxxxxx.tk         | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout   |
| 320  | 172.67.49.134            | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 381  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 450  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 161 次 (95.8%)
- **连接超时**: 7 次 (4.2%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 108.162（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 161 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：trevor.ns.cloudflare.com (4次),
uriah.ns.cloudflare.com (3次), bowen.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 362  | craig.ns.cloudflare.com    | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 74   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 177      | cloudflare |
| 58   | huxley.ns.cloudflare.com   | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 71   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 188      | cloudflare |
| 314  | otto.ns.cloudflare.com     | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 206      | cloudflare |
| 28   | julio.ns.cloudflare.com    | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 212      | cloudflare |
| 4    | uriah.ns.cloudflare.com    | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 68   | ct.877774.xyz              | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 269  | kyree.ns.cloudflare.com    | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 73   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 186  | benedict.ns.cloudflare.com | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 237      | cloudflare |
| 137  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 227  | sullivan.ns.cloudflare.com | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 390  | 162.159.36.104             | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 248      | cloudflare |
| 415  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 249      | cloudflare |
| 363  | craig.ns.cloudflare.com    | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 250      | cloudflare |
| 171  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 253      | cloudflare |
| 357  | lewis.ns.cloudflare.com    | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 60   | huxley.ns.cloudflare.com   | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 257      | cloudflare |
| 165  | cf.090227.xyz              | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 328  | damien.ns.cloudflare.com   | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 414  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 259      | cloudflare |
| 48   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 263      | cloudflare |
| 24   | 172.64.154.18              | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 264      | cloudflare |
| 124  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 264      | cloudflare |
| 3    | uriah.ns.cloudflare.com    | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 265      | cloudflare |
| 215  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 265      | cloudflare |
| 225  | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 315  | otto.ns.cloudflare.com     | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 267      | cloudflare |
| 275  | dylan.ns.cloudflare.com    | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 269      | cloudflare |
| 276  | dylan.ns.cloudflare.com    | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 270      | cloudflare |
| 442  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 270      | cloudflare |
| 345  | www.wto.org                | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 272      | cloudflare |
| 330  | damien.ns.cloudflare.com   | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 279      | cloudflare |
| 27   | julio.ns.cloudflare.com    | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 280      | cloudflare |
| 103  | trevor.ns.cloudflare.com   | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 281      | cloudflare |
| 152  | cris.ns.cloudflare.com     | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 282      | cloudflare |
| 416  | abdullah.ns.cloudflare.com | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 282      | cloudflare |
| 9    | bowen.ns.cloudflare.com    | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 49   | wilson.ns.cloudflare.com   | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 426  | ashton.ns.cloudflare.com   | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 284      | cloudflare |
| 424  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 293      | cloudflare |
| 101  | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 295      | cloudflare |
| 104  | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 295      | cloudflare |
| 361  | craig.ns.cloudflare.com    | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 298      | cloudflare |
| 5    | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 299      | cloudflare |
| 172  | moura.ns.cloudflare.com    | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 301      | cloudflare |
| 59   | huxley.ns.cloudflare.com   | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 309      | cloudflare |
| 153  | cris.ns.cloudflare.com     | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 316      | cloudflare |
| 216  | rustam.ns.cloudflare.com   | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 327      | cloudflare |
| 70   | ct.877774.xyz              | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 328      | cloudflare |
| 226  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 335      | cloudflare |
| 448  | saas.sin.fan               | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 337      | cloudflare |
| 125  | braden.ns.cloudflare.com   | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 339      | cloudflare |
| 72   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 340      | cloudflare |
| 99   | 34.143.159.175             | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 348      | cloudflare |
| 174  | moura.ns.cloudflare.com    | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 352      | cloudflare |
| 334  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 352      | cloudflare |
| 10   | bowen.ns.cloudflare.com    | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 354      | cloudflare |
| 69   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 355      | cloudflare |
| 173  | moura.ns.cloudflare.com    | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 355      | cloudflare |
| 1    | 104.18.42.26               | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 356      | cloudflare |
| 88   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 363      | cloudflare |
| 234  | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 363      | cloudflare |
| 261  | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 370      | cloudflare |
| 188  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 375      | cloudflare |
| 22   | www.visa.com.hk            | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 379      | cloudflare |
| 441  | www.visa.com.sg            | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 379      | cloudflare |
| 300  | cfip.xxxxxxxx.tk           | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 383      | cloudflare |
| 46   | asia.877774.xyz            | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 75   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 437  | yx-auto.pages.dev          | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 443  | pranab.ns.cloudflare.com   | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 184  | www.glassdoor.com          | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 387      | cloudflare |
| 111  | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 274  | dylan.ns.cloudflare.com    | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 389      | cloudflare |
| 42   | bestcf.030101.xyz          | 104.19.61.113   | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 65   | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 400      | cloudflare |
| 329  | damien.ns.cloudflare.com   | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 402      | cloudflare |
| 118  | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 405      | cloudflare |
| 295  | cfip.xxxxxxxx.tk           | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 406      | cloudflare |
| 33   | cf.zhetengsha.eu.org       | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 411      | cloudflare |
| 409  | ip.gs                      | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 412      | cloudflare |
| 164  | cf.090227.xyz              | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 413      | cloudflare |
| 50   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 415      | cloudflare |
| 23   | www.7749tv.com             | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 417      | cloudflare |
| 29   | julio.ns.cloudflare.com    | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 417      | cloudflare |
| 313  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 418      | cloudflare |
| 221  | www.ipchicken.com          | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 268  | kyree.ns.cloudflare.com    | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 425  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 423      | cloudflare |
| 346  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 425      | cloudflare |
| 233  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 432      | cloudflare |
| 303  | www.udemy.com              | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 432      | cloudflare |
| 214  | rustam.ns.cloudflare.com   | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 435      | cloudflare |
| 289  | cfip.xxxxxxxx.tk           | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 436      | cloudflare |
| 371  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 436      | cloudflare |
| 154  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 437      | cloudflare |
| 138  | decker.ns.cloudflare.com   | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 440      | cloudflare |
| 393  | 108.162.198.54             | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 442      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 4 条记录
- **慢 (200-500ms)**: 96 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 161 次

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
