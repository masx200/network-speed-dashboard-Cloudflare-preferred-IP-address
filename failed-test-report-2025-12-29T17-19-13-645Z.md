# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:19:13
- **数据来源**: connectivity_results-20251229-171912.json
- **总测试数**: 496
- **失败测试数**: 183
- **成功测试数**: 313
- **失败率**: 36.90%
- **平均延迟**: 115.35ms
- **最小延迟**: 45ms
- **最大延迟**: 696ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:19:13
- **IP地址**: 20.102.47.194
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 38.7095, -78.1539
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 180 次 (98.4%)
- **连接超时: I/O超时**: 3 次 (1.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (180 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 6 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 7 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 8 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 11 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 12 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 15 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 16 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 19 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 20 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 23 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 24 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 27 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 28 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 32 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 33 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 34 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 37 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 38 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 44 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 45 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 46 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 47 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 48 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 67 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 68 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 69 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 72 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 73 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 77 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 78 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 79 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 84 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 85 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 86 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 118 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 119 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 120 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 128 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 129 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 130 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 135 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 136 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 137 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 146 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 147 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 148 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 151 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 152 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 157 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4fdb:d8a1]:443: connect: network is unreachable |
| 158 | freeyx.cloudflare88.eu.org | 2606:4700:3009::83da:e4f:b8a5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009::83da:e4f:b8a5]:443: connect: network is unreachable |
| 159 | freeyx.cloudflare88.eu.org | 2606:4700:3009:7dab:39d5:a339:79b6:3c1c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:7dab:39d5:a339:79b6:3c1c]:443: connect: network is unreachable |
| 163 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 164 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 165 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 171 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 172 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 176 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 177 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 178 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 184 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 185 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 186 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 190 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 191 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 192 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 196 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 197 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 198 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 202 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 203 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 204 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 207 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 208 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 213 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 214 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 216 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 220 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 221 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 222 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 225 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 226 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 229 | bestcf.030101.xyz | 2606:4700:0:c5:4803:8845:8bde:1897 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:c5:4803:8845:8bde:1897]:443: connect: network is unreachable |
| 230 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:e7ac:854f:c15c:d3b1:fc6a]:443: connect: network is unreachable |
| 234 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 235 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 236 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 238 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 242 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 243 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 247 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 248 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 249 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 253 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 254 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 255 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 259 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 260 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 261 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 267 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 268 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 272 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 273 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 274 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 285 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 286 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 287 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 291 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 292 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 293 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 299 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 300 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 301 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 305 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 306 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 310 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 311 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 312 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 315 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 316 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 318 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 321 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 322 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 325 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 326 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 330 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 331 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 332 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 333 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 338 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 339 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 340 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 344 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 345 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 346 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 351 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 352 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 353 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 354 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 359 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 363 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 364 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 365 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 368 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 369 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 376 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 377 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 384 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 385 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 386 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 387 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 392 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 393 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 394 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 397 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 398 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 399 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 406 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 407 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 408 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 416 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 417 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 419 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 424 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 425 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 426 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 447 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 448 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 467 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 468 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 469 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 470 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 480 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 481 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 482 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 483 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 484 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 485 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 486 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 487 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 428 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 493 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 496 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 180 次 (98.4%)
- **连接超时**: 3 次 (1.6%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 183 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 180 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 429 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 438 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 442 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 444 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 443 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 435 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 375 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 366 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 367 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 54 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 296 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 240 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 411 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 217 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 347 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 450 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 142 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 290 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 409 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 140 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 432 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 441 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 457 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 372 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 431 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 410 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 436 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 415 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 413 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 459 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 489 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 289 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 396 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 70 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 122 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 371 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 404 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 90 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 437 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 117 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 179 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 455 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 101 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 116 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 26 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 303 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 199 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 454 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 209 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 252 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 440 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 472 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 103 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 126 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 183 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 280 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 281 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 445 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 490 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 35 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 144 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 265 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 461 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 4 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 319 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 324 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 156 | freeyx.cloudflare88.eu.org | 141.101.120.89 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 161 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 169 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 195 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 323 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 491 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 39 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 108 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 170 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 233 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 288 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 107 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 141 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 241 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 246 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 14 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 206 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 350 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 418 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 476 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 9 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 64 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 114 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 314 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 341 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 356 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 150 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 180 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 59 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 83 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 263 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 334 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 22 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 57 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 76 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 168 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 304 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 309 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 374 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 30 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 58 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 380 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 414 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 477 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 112 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 155 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 167 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 194 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 55 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 95 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 210 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 370 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 434 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 74 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 201 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 256 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 361 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 421 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 139 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 200 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 383 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 405 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 42 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 51 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 88 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 96 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 138 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 227 | bestcf.030101.xyz | 104.17.99.183 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 379 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 390 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 400 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 193 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 358 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 492 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 40 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 98 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 244 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 271 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 337 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 81 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 327 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 395 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 453 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 109 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 110 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 149 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 224 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 328 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 342 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 466 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 43 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 93 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 228 | bestcf.030101.xyz | 104.17.27.231 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 357 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 25 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 71 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 91 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 269 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 308 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 99 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 205 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 276 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 302 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 403 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 449 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 36 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 174 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 295 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 307 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 329 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 360 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 373 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 381 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 391 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 475 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 53 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 160 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 189 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 378 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 97 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 111 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 123 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 355 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 420 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 456 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 18 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 132 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 282 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 335 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 478 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 61 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 251 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 5 条记录
- **快 (50-100ms)**: 106 条记录
- **正常 (100-200ms)**: 89 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 180 次

### 按协议统计

- **none**: 183 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
