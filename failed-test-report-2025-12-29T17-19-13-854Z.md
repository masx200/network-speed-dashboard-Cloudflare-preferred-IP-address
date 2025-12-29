# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:19:13
- **数据来源**: connectivity_results-20251229-171913.json
- **总测试数**: 496
- **失败测试数**: 183
- **成功测试数**: 313
- **失败率**: 36.90%
- **平均延迟**: 112.35ms
- **最小延迟**: 66ms
- **最大延迟**: 489ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:19:13
- **IP地址**: 52.161.45.231
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1437, -104.8117
- **时区**: America/Denver
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
| 5 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 6 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 10 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 11 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 12 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 15 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 16 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 19 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 20 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 23 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 24 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 27 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 28 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 31 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 32 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 38 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 39 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 40 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 41 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 42 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 48 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 49 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 50 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 53 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 54 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 61 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 76 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 77 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 78 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 82 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 83 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 84 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 114 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 120 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 121 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 122 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 125 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 126 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 131 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 132 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 133 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 142 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 143 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 144 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 149 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 150 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 155 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 156 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 157 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 159 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4fdb:d8a1]:443: connect: network is unreachable |
| 160 | freeyx.cloudflare88.eu.org | 2606:4700:3009::83da:e4f:b8a5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009::83da:e4f:b8a5]:443: connect: network is unreachable |
| 161 | freeyx.cloudflare88.eu.org | 2606:4700:3009:7dab:39d5:a339:79b6:3c1c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:7dab:39d5:a339:79b6:3c1c]:443: connect: network is unreachable |
| 164 | cf.877771.xyz | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 165 | cf.877771.xyz | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 174 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 175 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 176 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 180 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 181 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 182 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 186 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 187 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 188 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 192 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 193 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 194 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 197 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 198 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 202 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 203 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 204 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 209 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 210 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 212 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 216 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 217 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 218 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 221 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:e7ac:854f:c15c:d3b1:fc6a]:443: connect: network is unreachable |
| 222 | bestcf.030101.xyz | 2606:4700:0:c5:4803:8845:8bde:1897 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:c5:4803:8845:8bde:1897]:443: connect: network is unreachable |
| 229 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 230 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 231 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 236 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 237 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 238 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 242 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 243 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 244 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 247 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 248 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 252 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 253 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 254 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 260 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 261 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 262 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 265 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 266 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 270 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 271 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 272 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 275 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 276 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 283 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 284 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 285 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 292 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 293 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 294 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 297 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 298 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 304 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 305 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 306 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 310 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 311 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 312 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 314 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 317 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 318 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 322 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 323 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 324 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 327 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 328 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 329 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 333 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 334 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 335 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 340 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 344 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 345 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 346 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 349 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 350 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 353 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 357 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 358 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 359 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 362 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 363 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 364 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 367 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 368 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 372 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 373 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 382 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 383 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 384 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 388 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 389 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 390 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 393 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 399 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 400 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 401 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 405 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 406 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 407 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 410 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 417 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 418 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 426 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 427 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 428 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 440 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 441 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 445 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 446 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 447 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 478 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 479 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 480 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 481 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 483 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 484 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 485 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 486 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 487 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 429 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 494 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
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
| 365 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 475 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 381 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 185 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 347 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 360 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 391 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 191 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 371 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 439 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 449 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 279 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 295 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 454 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 455 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 303 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 413 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 442 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 456 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 457 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 459 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 200 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 278 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 336 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 352 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 392 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 415 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 430 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 443 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 474 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 51 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 170 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 378 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 438 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 460 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 220 | bestcf.030101.xyz | 104.17.27.231 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 422 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 473 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 259 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 412 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 315 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 351 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 385 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 469 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 370 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 375 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 434 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 256 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 398 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 464 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 201 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 416 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 274 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 338 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 404 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 471 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 482 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 109 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 239 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 436 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 320 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 437 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 468 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 458 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 111 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 376 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 433 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 321 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 341 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 414 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 123 | icook.hk | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 179 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 219 | bestcf.030101.xyz | 104.17.99.183 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 307 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 425 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 205 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 29 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 73 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 94 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 110 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 167 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 172 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 178 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 215 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 424 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 463 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 26 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 37 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 138 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 232 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 325 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 408 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 409 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 423 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 467 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 34 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 60 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 63 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 67 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 86 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 97 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 135 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 190 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 227 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 287 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 300 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 366 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 420 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 432 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 490 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 493 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 43 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 56 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 70 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 145 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 146 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 168 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 465 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 52 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 55 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 69 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 89 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 90 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 103 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 116 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 196 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 282 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 348 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 134 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 137 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 151 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 301 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 435 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 4 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 92 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 98 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 208 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 302 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 309 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 14 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 158 | freeyx.cloudflare88.eu.org | 141.101.120.89 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 163 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 207 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 319 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 489 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 71 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 177 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 342 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 354 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 444 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 472 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 91 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 119 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 241 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 245 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 268 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 411 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 451 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 62 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 139 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 267 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 277 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 290 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 419 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 93 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 153 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 166 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 184 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 286 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 214 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 330 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 355 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 403 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 107 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 251 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 337 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 30 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 115 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 130 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 246 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 299 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 316 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 339 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 18 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 21 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 99 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 108 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 154 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 269 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 379 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 386 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 22 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 87 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 128 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 356 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 377 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 8 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 81 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 87 条记录
- **正常 (100-200ms)**: 113 条记录
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
