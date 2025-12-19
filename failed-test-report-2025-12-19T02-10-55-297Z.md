# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:10:55
- **数据来源**: connectivity_results-20251219-021054.json
- **总测试数**: 457
- **失败测试数**: 172
- **成功测试数**: 285
- **失败率**: 37.64%
- **平均延迟**: 121.48ms
- **最小延迟**: 49ms
- **最大延迟**: 595ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:10:55
- **IP地址**: 20.109.39.56
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 169 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (169 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 5 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 6 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 10 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 11 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 12 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 15 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 16 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 22 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 23 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 24 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 25 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 26 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 29 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 30 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 37 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 38 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 39 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 42 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 43 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 50 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 51 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 52 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 58 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 59 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 60 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 64 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 65 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 66 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 69 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 70 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 71 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 78 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 79 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 102 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 103 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 104 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 105 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 108 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 109 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 121 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 122 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 123 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 126 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 127 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 160 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 161 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 162 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 167 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:12d8:5d82 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:f5e8:7af2:12d8:5d82]:443: connect: network is unreachable |
| 171 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 172 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 173 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 177 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 178 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 179 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 183 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 184 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 188 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 189 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 190 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 194 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 195 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 196 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 200 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 201 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 202 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 205 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 206 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 210 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 211 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 212 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 215 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 216 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 222 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 223 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 224 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 225 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 228 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 229 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 235 | bestcf.030101.xyz | 2606:4700:0:3701:6875:ca63:41c9:a311 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:3701:6875:ca63:41c9:a311]:443: connect: network is unreachable |
| 236 | bestcf.030101.xyz | 2606:4700:0:57:503f:ebaa:486e:53f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:57:503f:ebaa:486e:53f7]:443: connect: network is unreachable |
| 239 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 240 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 243 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 244 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 245 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 251 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 252 | xn--b6gac.eu.org | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 256 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 257 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 258 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 261 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 262 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 266 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 267 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 268 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 269 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 273 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 274 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 275 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 279 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 280 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 281 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 282 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 288 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 289 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 292 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 293 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 301 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 302 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 303 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 307 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 308 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 309 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 313 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 317 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 318 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 319 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 325 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 326 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 327 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 330 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 336 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 337 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 342 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 343 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 344 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 349 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 350 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 351 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 355 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 356 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 357 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 360 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 361 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 366 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 367 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 368 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 372 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 373 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable |
| 376 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 377 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 380 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 381 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 383 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 387 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 388 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 389 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 392 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 393 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 397 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 398 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 399 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 400 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 404 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 405 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 406 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 411 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 414 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 415 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 420 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 421 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 424 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 425 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 430 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 431 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 432 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 442 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 443 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 444 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 450 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 456 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 457 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 169 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 172 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 169 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), time.is (3次), iplocation.io (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 277 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 324 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 407 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 153 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 270 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 408 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 135 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 264 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 413 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 345 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 412 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 150 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 141 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 410 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 426 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 441 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 329 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 95 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 295 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 419 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 130 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 232 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 374 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 246 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 163 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 164 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 192 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 418 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 155 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 21 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 333 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 335 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 35 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 198 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 305 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 84 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 159 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 297 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 197 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 4 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 89 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 143 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 152 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 371 | ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 7 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 74 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 107 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 120 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 207 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 31 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 81 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 98 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 113 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 358 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 403 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 49 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 157 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 283 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 321 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 391 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 422 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 62 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 73 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 77 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 90 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 116 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 278 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 375 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 438 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 451 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 45 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 149 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 231 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 254 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 9 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 40 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 114 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 137 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 151 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 156 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 323 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 452 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 237 | cf.zhetengsha.eu.org | 172.66.44.77 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 242 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 334 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 365 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 409 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 447 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 154 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 332 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 348 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 57 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 82 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 93 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 131 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 138 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 148 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 314 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 401 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 213 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 291 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 175 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 298 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 320 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 72 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 233 | bestcf.030101.xyz | 162.159.133.251 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 36 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 119 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 144 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 191 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 203 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 263 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 385 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 446 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 32 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 41 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 68 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 142 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 176 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 199 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 217 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 248 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 294 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 315 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 339 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 354 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 363 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 17 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 63 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 67 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 220 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 286 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 310 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 423 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 112 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 133 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 170 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 250 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 255 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 338 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 394 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 14 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 48 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 449 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 27 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 34 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 44 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 427 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 28 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 86 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 97 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 219 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 238 | cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 241 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 247 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 271 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 340 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 437 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 209 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 260 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 296 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 306 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 347 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 402 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 106 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 134 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 136 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 259 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 299 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 91 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 111 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 132 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 146 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 187 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 384 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 100 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 140 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 204 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 33 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 75 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 94 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 117 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 129 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 147 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 300 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 346 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 364 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 46 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 174 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 181 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 186 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 230 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 290 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 53 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 92 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 99 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 139 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 218 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 386 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 85 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 73 条记录
- **正常 (100-200ms)**: 126 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 169 次

### 按协议统计

- **none**: 172 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
