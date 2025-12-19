# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:11:18
- **数据来源**: connectivity_results-20251219-021118.json
- **总测试数**: 457
- **失败测试数**: 172
- **成功测试数**: 285
- **失败率**: 37.64%
- **平均延迟**: 80.58ms
- **最小延迟**: 36ms
- **最大延迟**: 464ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:11:18
- **IP地址**: 52.159.244.73
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

- **网络不可达: 网络不可达**: 169 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (169 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 5 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 6 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 9 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 10 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 13 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 14 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 17 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 18 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 21 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 22 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 28 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 29 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 35 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 36 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 37 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 38 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 39 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 45 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 60 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 61 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 62 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 67 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 68 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 69 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 72 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 73 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 76 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 80 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 81 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 82 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 86 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 87 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 88 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 91 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 92 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 102 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 103 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 104 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 119 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 120 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 152 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 153 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 154 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 159 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:12d8:5d82 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:f5e8:7af2:12d8:5d82]:443: connect: network is unreachable |
| 163 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 164 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 165 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 169 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 170 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 171 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 175 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 176 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 177 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 181 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 182 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 183 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 187 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 188 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 189 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 193 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 194 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 195 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 198 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 199 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 202 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 203 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 206 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 209 | bestcf.030101.xyz | 2606:4700:0:3701:6875:ca63:41c9:a311 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:3701:6875:ca63:41c9:a311]:443: connect: network is unreachable |
| 210 | bestcf.030101.xyz | 2606:4700:0:57:503f:ebaa:486e:53f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:57:503f:ebaa:486e:53f7]:443: connect: network is unreachable |
| 213 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 214 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 220 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 221 | xn--b6gac.eu.org | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 223 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 226 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 227 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 231 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 232 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 233 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 238 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 239 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 243 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 244 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 245 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 249 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 250 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 251 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 255 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 256 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 257 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 264 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 265 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 266 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 272 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 273 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 274 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 278 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 279 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 280 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 283 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 284 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 290 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 291 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 292 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 296 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 297 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 300 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 301 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 305 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 306 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 307 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 311 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 312 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 313 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 317 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 318 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 319 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 321 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 325 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 326 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 327 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 330 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 334 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 335 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 336 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 339 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 340 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 344 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 347 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 348 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 352 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 353 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 354 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 357 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 358 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 361 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 364 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 365 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 370 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 371 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 376 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 377 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 378 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 382 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 387 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 388 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 389 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 391 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 399 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 400 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable |
| 404 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 405 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 406 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 410 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 411 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 412 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 417 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 418 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 423 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 424 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 430 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 431 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 432 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 436 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 437 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 438 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 442 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 443 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 444 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 451 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
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

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), iplocation.io (3次), huxley.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 247 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 157 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 96 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 222 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 329 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 63 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 192 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 366 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 374 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 425 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 133 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 218 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 397 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 345 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 30 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 212 | cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 234 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 268 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 289 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 396 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 429 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 434 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 56 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 201 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 267 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 310 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 414 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 136 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 236 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 315 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 426 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 449 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 4 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 100 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 309 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 390 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 420 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 435 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 78 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 149 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 294 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 308 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 341 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 427 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 110 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 167 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 385 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 407 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 54 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 299 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 23 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 179 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 237 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 31 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 66 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 112 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 128 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 298 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 197 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 277 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 97 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 379 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 85 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 184 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 174 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 351 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 401 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 452 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 47 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 121 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 132 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 367 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 32 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 43 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 52 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 281 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 402 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 433 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 455 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 26 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 207 | bestcf.030101.xyz | 162.159.133.251 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 8 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 115 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 386 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 448 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 24 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 49 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 99 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 105 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 137 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 142 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 217 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 269 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 356 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 75 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 98 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 122 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 125 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 130 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 131 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 162 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 178 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 7 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 12 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 53 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 79 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 147 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 253 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 331 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 42 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 44 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 114 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 116 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 134 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 138 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 141 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 158 | freeyx.cloudflare88.eu.org | 141.101.120.95 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 211 | cf.zhetengsha.eu.org | 172.66.44.77 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 240 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 302 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 337 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 368 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 398 | ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 27 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 74 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 276 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 453 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 454 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 3 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 190 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 422 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 55 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 118 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 148 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 288 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 11 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 41 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 229 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 342 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 322 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 338 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 395 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 441 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 20 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 77 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 127 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 263 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 328 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 421 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 450 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 46 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 186 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 191 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 314 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 447 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 65 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 71 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 84 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 111 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 304 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 393 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 93 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 258 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 155 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 259 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 90 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 166 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 282 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 360 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 70 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 196 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 419 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 126 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 440 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 109 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 123 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 235 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 15 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 95 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 135 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 349 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 375 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 113 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 270 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 108 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 139 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 168 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 293 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 346 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 40 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 129 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 208 | bestcf.030101.xyz | 104.19.53.201 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 286 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 333 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 409 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 19 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 143 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 44 条记录
- **快 (50-100ms)**: 156 条记录
- **正常 (100-200ms)**: 0 条记录
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
