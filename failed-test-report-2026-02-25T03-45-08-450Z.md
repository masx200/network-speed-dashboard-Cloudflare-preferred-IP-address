# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 03:45:08
- **数据来源**: connectivity_results-20260225-034507.json
- **总测试数**: 776
- **失败测试数**: 233
- **成功测试数**: 543
- **失败率**: 30.03%
- **平均延迟**: 102.06ms
- **最小延迟**: 55ms
- **最大延迟**: 2902ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 03:45:08
- **IP地址**: 52.173.219.145
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

- **网络不可达: 网络不可达**: 230 次 (98.7%)
- **连接超时: I/O超时**: 3 次 (1.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (230 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7]:443: connect: network is unreachable |
| 17 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 18 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 21 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 22 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 25 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 26 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 29 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 30 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 36 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 37 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 40 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 41 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 45 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 46 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 48 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 52 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 53 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 54 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 60 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 61 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 62 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 63 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 64 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 71 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 72 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 73 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 76 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 77 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 106 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 110 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 111 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 112 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 124 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 125 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 126 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 130 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 131 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 132 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 136 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 137 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 141 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 142 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 146 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 147 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 148 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 159 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 160 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 161 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 166 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 167 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 168 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 172 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 173 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 174 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 178 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 179 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 180 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 185 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 186 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 187 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 191 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 192 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 196 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 197 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 198 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 199 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 202 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 203 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 210 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:d9:2acf:b5e0:5a46:4358]:443: connect: network is unreachable |
| 211 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:dd:df95:6eb1:ffa4:6779]:443: connect: network is unreachable |
| 214 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 215 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 217 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 220 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 221 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 225 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 226 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 227 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 230 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4fdb:d8a1]:443: connect: network is unreachable |
| 231 | freeyx.cloudflare88.eu.org | 2606:4700:3009::83da:e4f:b8a5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009::83da:e4f:b8a5]:443: connect: network is unreachable |
| 232 | freeyx.cloudflare88.eu.org | 2606:4700:3009:7dab:39d5:a339:79b6:3c1c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:7dab:39d5:a339:79b6:3c1c]:443: connect: network is unreachable |
| 236 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 237 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 238 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 244 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 245 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 246 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 250 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 251 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 252 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 256 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 257 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 258 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 261 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 262 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 269 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 270 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 271 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 275 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 276 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 277 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 285 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 286 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 289 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 290 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 294 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 295 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 296 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 300 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 301 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 302 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 304 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 307 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 308 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 311 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 312 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 314 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 318 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 319 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 320 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 326 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 327 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 328 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 329 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 333 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 334 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 335 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 342 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 345 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 346 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 349 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 352 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 353 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 359 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 360 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 361 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 368 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 369 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 370 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 374 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 375 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 376 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 379 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 384 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 385 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 386 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 390 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 391 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 395 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 396 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 398 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 403 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 404 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 405 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 411 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 412 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 413 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 416 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 417 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 442 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 443 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 444 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 465 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 466 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 467 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 468 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 469 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 470 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 471 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 472 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 473 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 474 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 486 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 487 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 518 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 519 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 520 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 521 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 522 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 523 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 524 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 525 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 526 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 527 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 591 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 592 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 593 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 594 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 595 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 596 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 597 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 598 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 599 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 600 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 651 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 652 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 653 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 654 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 655 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 656 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 657 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 658 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 660 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 661 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 678 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 679 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 680 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 697 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 698 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 699 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 710 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 711 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 712 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 713 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 714 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 715 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 716 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 717 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 718 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 719 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 759 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:6e:e874:db4f:a1d5:2163]:443: connect: network is unreachable |
| 760 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:0:edda:98f0:da65:4271]:443: connect: network is unreachable |
| 761 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:85:ac4c:8137:506:5188]:443: connect: network is unreachable |
| 762 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2]:443: connect: network is unreachable |
| 763 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:fc87:e2d6:88c3:378b]:443: connect: network is unreachable |
| 764 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53]:443: connect: network is unreachable |
| 765 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3]:443: connect: network is unreachable |
| 766 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:4f38:5221:f77f:fe11]:443: connect: network is unreachable |
| 767 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::ee:b8fb:877f]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 341 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 407 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 776 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 230 次 (98.7%)
- **连接超时**: 3 次 (1.3%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 233 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 230 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), iplocation.io (3次), huxley.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 543 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 544 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 498 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 499 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 619 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 681 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 496 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 508 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 573 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 662 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 88 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 212 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 585 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 457 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 576 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 602 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 297 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 577 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 20 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 303 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 330 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 460 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 482 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 639 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 642 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 644 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 704 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 757 | 162.159.12.120 | 162.159.12.120 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 100 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 103 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 201 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 625 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 636 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 637 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 645 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 723 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 95 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 485 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 550 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 562 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 580 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 632 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 633 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 671 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 675 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 682 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 701 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 145 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 165 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 228 | freeyx.cloudflare88.eu.org | 141.101.120.101 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 438 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 451 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 456 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 531 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 240 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 477 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 529 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 540 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 691 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 702 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 739 | 104.26.6.247 | 104.26.6.247 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 748 | 104.25.241.198 | 104.25.241.198 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 24 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 195 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 493 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 558 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 588 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 622 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 686 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 726 | 172.64.53.165 | 172.64.53.165 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 8 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 50 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 356 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 435 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 746 | 104.17.97.228 | 104.17.97.228 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 758 | 162.159.3.128 | 162.159.3.128 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 399 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 424 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 511 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 533 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 567 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 613 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 501 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 504 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 665 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 753 | 104.18.166.232 | 104.18.166.232 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 611 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 621 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 635 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 75 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 749 | 104.25.244.239 | 104.25.244.239 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 755 | 104.25.250.174 | 104.25.250.174 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 23 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 31 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 188 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 216 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 377 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 488 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 507 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 78 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 119 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 204 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 283 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 382 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 401 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 481 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 571 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 685 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 15 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 578 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 99 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 351 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 450 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 676 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 117 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 175 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 282 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 393 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 555 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 720 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 140 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 170 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 184 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 239 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 263 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 397 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 559 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 579 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 11 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 49 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 135 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 155 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 229 | freeyx.cloudflare88.eu.org | 141.101.121.138 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 414 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 420 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 479 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 534 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 574 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 589 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 664 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 670 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 85 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 149 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 189 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 392 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 394 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 429 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 554 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 687 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 700 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 750 | 108.162.198.70 | 108.162.198.70 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 2 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 51 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 177 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 267 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 339 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 340 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 354 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 366 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 378 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 423 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 447 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 455 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 512 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 620 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 624 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 674 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 693 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 733 | 104.18.42.16 | 104.18.42.16 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 91 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 143 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 194 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 313 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 317 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 406 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 464 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 476 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 503 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 537 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 545 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 616 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 618 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 626 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 696 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 16 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 33 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 47 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 107 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 108 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 121 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 181 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 218 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 337 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 348 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 400 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 421 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 426 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 500 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 510 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 538 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 556 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 565 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 643 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 692 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 703 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 705 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 80 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 84 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 87 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 156 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 279 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 332 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 336 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 432 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 483 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 484 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 557 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 667 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 689 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 706 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 55 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 57 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 94 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 101 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 208 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 255 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 259 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 260 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 355 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 449 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 506 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 575 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 587 | www.7749tv.com | 104.19.255.188 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 638 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 708 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 740 | 104.26.14.117 | 104.26.14.117 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 752 | 104.17.111.150 | 104.17.111.150 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 35 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 97 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 120 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 133 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 264 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 274 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 280 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 293 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 437 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 439 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 458 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 495 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 561 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 747 | 104.26.13.110 | 104.26.13.110 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 14 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 34 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 42 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 68 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 92 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 123 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 248 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 273 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 321 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 338 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 452 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 453 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 462 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 532 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 541 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 546 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 628 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 629 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 673 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 727 | 172.64.153.140 | 172.64.153.140 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 742 | 172.67.65.150 | 172.67.65.150 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 773 | 162.159.1.39 | 162.159.1.39 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 213 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 288 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 315 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 343 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 436 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 454 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 547 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 669 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 738 | 104.20.28.239 | 104.20.28.239 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 81 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 242 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 428 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 513 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 646 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 730 | 172.64.52.189 | 172.64.52.189 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 754 | 104.25.245.233 | 104.25.245.233 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 32 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 66 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 647 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 79 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 291 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 306 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 344 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 431 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 553 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 627 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 743 | 104.26.0.210 | 104.26.0.210 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 6 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 151 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 171 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 316 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 497 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 505 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 572 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 586 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 607 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 672 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 707 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 741 | 104.20.25.181 | 104.20.25.181 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 774 | 198.41.222.191 | 198.41.222.191 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 150 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 309 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 362 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 430 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 492 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 768 | 162.159.58.17 | 162.159.58.17 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 58 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 134 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 278 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 305 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 480 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 491 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 612 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 43 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 56 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 86 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 93 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 98 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 102 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 371 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 381 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 408 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 434 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 463 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 528 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 583 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 744 | 104.26.8.171 | 104.26.8.171 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 83 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 154 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 158 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 563 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 601 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 617 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 634 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 724 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 756 | 162.159.11.128 | 162.159.11.128 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 770 | 172.64.53.202 | 172.64.53.202 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 284 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 357 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 427 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 490 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 688 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 163 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 234 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 358 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 364 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 365 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 446 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 683 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 728 | 108.162.198.85 | 108.162.198.85 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 745 | 104.20.20.192 | 104.20.20.192 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 751 | 104.18.172.20 | 104.18.172.20 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 44 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 139 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 152 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 205 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 224 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 241 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 325 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 347 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 459 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 542 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 614 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 721 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 722 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 10 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 28 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 129 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 164 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 209 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 281 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 322 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 584 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 615 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 65 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 82 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 183 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 445 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 709 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 19 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 39 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 200 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 265 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 292 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 299 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 543 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 566 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 623 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 12 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 27 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 113 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 114 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 169 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 266 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 324 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 387 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 516 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 735 | 172.64.229.134 | 172.64.229.134 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 737 | 104.26.1.88 | 104.26.1.88 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 90 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 109 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 118 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 249 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 253 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 560 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 570 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 603 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 736 | 162.159.36.205 | 162.159.36.205 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 74 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 207 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 219 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 235 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 247 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 331 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 383 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 461 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 489 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 548 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 640 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 650 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 769 | 162.159.38.134 | 162.159.38.134 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 530 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 659 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 690 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 127 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 287 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 350 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 389 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 415 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 569 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 630 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 648 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 684 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 310 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 425 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 433 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 478 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 494 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 604 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 608 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 3 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 272 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 402 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 475 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 539 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 551 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 649 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 729 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 116 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 193 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 422 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 514 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 606 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 373 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 388 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 440 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 631 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 418 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 666 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 694 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 38 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 67 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 153 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 363 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 515 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 725 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 367 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 536 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 581 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 298 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 609 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 502 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 410 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 564 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 734 | 162.159.39.146 | 162.159.39.146 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 549 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 372 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 641 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 677 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 732 | 162.159.44.101 | 162.159.44.101 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 5 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 182 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 448 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 89 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 582 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 668 | 162.159.38.226 | 162.159.38.226 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 441 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 254 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 590 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 268 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 144 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 610 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 663 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 243 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 772 | 162.159.39.180 | 162.159.39.180 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 535 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 96 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 568 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 605 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 552 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 517 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 206 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 409 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 323 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 122 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 128 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 70 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 190 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 509 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 695 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 157 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 731 | 162.159.45.65 | 162.159.45.65 | IPv4 | h3 | ✅ 成功 | 160 | cloudflare |
| 176 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 7 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 233 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare |
| 419 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 223 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 189 | cloudflare |
| 9 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 193 | cloudflare |
| 13 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 210 | cloudflare |
| 69 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 213 | cloudflare |
| 771 | 104.17.193.113 | 104.17.193.113 | IPv4 | h3 | ✅ 成功 | 226 | cloudflare |
| 222 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 238 | cloudflare |
| 4 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 316 | cloudflare |
| 380 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 348 | cloudflare |
| 138 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 480 | cloudflare |
| 162 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 500 | cloudflare |
| 775 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 842 | cloudflare |
| 115 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 2902 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 366 条记录
- **正常 (100-200ms)**: 167 条记录
- **慢 (200-500ms)**: 7 条记录
- **很慢 (>500ms)**: 3 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 230 次

### 按协议统计

- **none**: 233 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
