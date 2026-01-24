# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/24 00:08:19
- **数据来源**: connectivity_results-20260124-000819.json
- **总测试数**: 783
- **失败测试数**: 232
- **成功测试数**: 551
- **失败率**: 29.63%
- **平均延迟**: 92.92ms
- **最小延迟**: 38ms
- **最大延迟**: 520ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/24 00:08:19
- **IP地址**: 172.183.135.246
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8835, -87.6305
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 229 次 (98.7%)
- **连接超时: I/O超时**: 3 次 (1.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (229 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 25 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 26 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 27 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 37 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 38 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 39 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 40 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 41 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 42 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 43 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 44 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 45 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 46 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 56 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 57 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 63 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 64 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 65 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 77 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 105 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 106 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 110 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 111 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 121 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 164 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 165 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 166 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 169 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 170 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 173 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 174 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 177 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 178 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 181 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 182 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 188 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 189 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 195 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 196 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 197 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 212 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 213 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 214 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 215 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 216 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 220 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 251 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 252 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 256 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 257 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 258 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 264 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 265 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 266 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 269 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 275 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 276 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 277 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 280 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 281 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 290 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 291 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 298 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 299 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 300 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 304 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 305 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 306 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 313 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 314 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 315 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 321 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 322 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 323 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 328 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 329 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 330 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 334 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 335 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 336 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 340 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 341 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 342 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 346 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 347 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 348 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 352 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 353 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 354 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 359 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 360 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 364 | freeyx.cloudflare88.eu.org | 2606:4700:3010:d:d45d:5d93:5a0d:c76 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:d:d45d:5d93:5a0d:c76]:443: connect: network is unreachable |
| 365 | freeyx.cloudflare88.eu.org | 2606:4700:3010:e070:5d82:552a:2064:72e5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:e070:5d82:552a:2064:72e5]:443: connect: network is unreachable |
| 366 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 372 | bestcf.030101.xyz | 2606:4700::7e99:e849:a432:54e0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::7e99:e849:a432:54e0]:443: connect: network is unreachable |
| 373 | bestcf.030101.xyz | 2606:4700::28:3815 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::28:3815]:443: connect: network is unreachable |
| 376 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 377 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 378 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 379 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 380 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 381 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 382 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 383 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 384 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 385 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 403 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 404 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 408 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 409 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 421 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 434 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 435 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 436 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 437 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 438 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 442 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 448 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 449 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 450 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 454 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 455 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 461 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 462 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 463 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 467 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 468 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 469 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 473 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 474 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 475 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 481 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 482 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 488 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 489 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 494 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 495 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 496 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 501 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 502 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 503 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 506 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 507 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 511 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 512 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 513 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 517 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 518 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 519 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 521 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 524 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 525 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 527 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 529 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 530 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 531 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 532 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 538 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 539 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 540 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 550 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 551 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 555 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 556 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 557 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 558 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 564 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 565 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 566 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 567 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 582 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 583 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 584 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 587 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 588 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 591 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 592 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 600 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 601 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 602 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 609 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 616 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 620 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 621 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 630 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 631 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 632 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 637 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 638 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 639 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 646 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 647 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 648 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 649 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 650 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 651 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 652 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 655 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 656 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 657 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 658 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 659 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 662 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 667 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 675 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 676 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 703 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 704 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 705 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 708 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 713 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 714 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 715 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 716 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 717 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 718 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 719 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 720 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 721 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 765 | 2803:f800:51:b883:8fe2:9a40:3016:f74 | 2803:f800:51:b883:8fe2:9a40:3016:f74 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:b883:8fe2:9a40:3016:f74]:443: connect: network is unreachable |
| 766 | 2a06:98c1:3109:ecc2:5371:912b:e506:cd47 | 2a06:98c1:3109:ecc2:5371:912b:e506:cd47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:ecc2:5371:912b:e506:cd47]:443: connect: network is unreachable |
| 767 | 2a06:98c1:3103:ca:fa78:84d3:2596:f679 | 2a06:98c1:3103:ca:fa78:84d3:2596:f679 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:ca:fa78:84d3:2596:f679]:443: connect: network is unreachable |
| 768 | 2803:f800:50:41ba:5536:115f:5191:1805 | 2803:f800:50:41ba:5536:115f:5191:1805 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:41ba:5536:115f:5191:1805]:443: connect: network is unreachable |
| 769 | 2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf | 2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf]:443: connect: network is unreachable |
| 770 | 2400:cb00:2049:b1aa:a619:c233:939c:74 | 2400:cb00:2049:b1aa:a619:c233:939c:74 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:b1aa:a619:c233:939c:74]:443: connect: network is unreachable |
| 771 | 2a06:98c1:3105::4c1a:e9f2:d41e | 2a06:98c1:3105::4c1a:e9f2:d41e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::4c1a:e9f2:d41e]:443: connect: network is unreachable |
| 772 | 2a06:98c1:3107:0:f6:2182:d4a8:339f | 2a06:98c1:3107:0:f6:2182:d4a8:339f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:f6:2182:d4a8:339f]:443: connect: network is unreachable |
| 773 | 2a06:98c1:310a:c1:ab96:14d6:b31a:6d03 | 2a06:98c1:310a:c1:ab96:14d6:b31a:6d03 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:c1:ab96:14d6:b31a:6d03]:443: connect: network is unreachable |
| 774 | 2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2 | 2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 578 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 782 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 783 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 229 次 (98.7%)
- **连接超时**: 3 次 (1.3%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 232 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 229 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 551 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 367 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 505 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 15 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 167 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 431 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 523 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 598 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 622 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 70 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 245 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 422 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 433 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 268 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 522 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 571 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 625 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 755 | 104.16.156.40 | 104.16.156.40 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 289 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 447 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 711 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 735 | 104.20.16.97 | 104.20.16.97 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 127 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 297 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 307 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 410 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 428 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 441 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 464 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 534 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 593 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 603 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 155 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 176 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 198 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 670 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 753 | 104.17.145.189 | 104.17.145.189 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 757 | 104.16.249.149 | 104.16.249.149 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 405 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 500 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 728 | 172.64.48.78 | 172.64.48.78 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 249 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 293 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 351 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 355 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 483 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 544 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 561 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 585 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 589 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 612 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 673 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 688 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 692 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 752 | 104.18.80.38 | 104.18.80.38 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 389 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 456 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 614 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 727 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 104 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 180 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 370 | bestcf.030101.xyz | 104.19.156.118 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 397 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 545 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 549 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 642 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 672 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 695 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 712 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 758 | 104.19.60.36 | 104.19.60.36 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 248 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 386 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 457 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 617 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 623 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 684 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 700 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 748 | 104.26.3.109 | 104.26.3.109 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 172 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 363 | freeyx.cloudflare88.eu.org | 141.101.121.231 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 491 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 615 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 706 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 709 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 75 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 199 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 374 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 399 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 458 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 666 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 741 | 104.26.15.22 | 104.26.15.22 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 742 | 104.26.7.8 | 104.26.7.8 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 83 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 288 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 423 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 484 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 485 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 595 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 679 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 682 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 175 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 235 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 246 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 250 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 274 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 777 | 172.64.48.160 | 172.64.48.160 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 211 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 411 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 478 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 125 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 158 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 316 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 412 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 21 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 368 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 487 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 597 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 664 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 697 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 724 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 751 | 104.16.154.33 | 104.16.154.33 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 759 | 104.17.106.109 | 104.17.106.109 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 498 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 596 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 690 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 303 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 674 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 749 | 104.26.1.119 | 104.26.1.119 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 205 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 282 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 429 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 562 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 725 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 740 | 104.26.2.45 | 104.26.2.45 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 183 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 261 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 402 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 418 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 629 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 723 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 141 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 362 | freeyx.cloudflare88.eu.org | 141.101.121.206 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 415 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 660 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 11 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 114 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 345 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 96 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 286 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 394 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 425 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 509 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 698 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 119 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 325 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 398 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 440 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 493 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 604 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 134 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 626 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 665 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 327 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 477 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 611 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 685 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 146 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 49 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 84 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 109 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 126 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 130 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 154 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 207 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 267 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 407 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 594 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 756 | 104.17.109.58 | 104.17.109.58 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 99 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 331 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 607 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 645 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 100 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 311 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 624 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 710 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 159 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 689 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 200 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 204 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 417 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 480 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 528 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 50 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 103 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 309 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 318 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 778 | 172.64.52.145 | 172.64.52.145 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 81 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 580 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 112 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 339 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 568 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 694 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 699 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 732 | 162.159.45.11 | 162.159.45.11 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 7 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 10 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 28 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 36 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 69 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 136 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 149 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 160 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 185 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 273 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 401 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 446 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 581 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 76 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 184 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 271 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 677 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 88 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 217 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 420 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 443 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 497 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 499 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 693 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 744 | 104.26.11.221 | 104.26.11.221 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 73 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 87 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 101 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 143 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 163 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 413 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 66 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 171 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 294 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 390 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 516 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 575 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 764 | 162.159.34.8 | 162.159.34.8 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 8 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 20 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 116 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 371 | bestcf.030101.xyz | 104.18.94.61 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 605 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 13 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 19 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 142 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 206 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 762 | 172.64.229.167 | 172.64.229.167 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 53 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 118 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 131 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 153 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 192 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 541 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 633 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 643 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 680 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 729 | 108.162.198.97 | 108.162.198.97 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 92 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 128 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 392 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 414 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 424 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 636 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 82 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 98 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 113 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 432 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 586 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 681 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 737 | 104.18.37.234 | 104.18.37.234 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 761 | 162.159.44.235 | 162.159.44.235 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 51 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 54 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 210 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 387 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 445 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 504 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 563 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 608 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 743 | 104.26.5.55 | 104.26.5.55 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 157 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 209 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 319 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 375 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 400 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 451 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 490 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 613 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 730 | 172.64.53.182 | 172.64.53.182 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 55 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 260 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 317 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 537 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 570 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 702 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 543 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 635 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 750 | 104.17.153.22 | 104.17.153.22 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 59 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 120 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 472 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 542 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 576 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 731 | 162.159.39.75 | 162.159.39.75 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 190 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 239 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 242 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 369 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 508 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 747 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 779 | 108.162.198.221 | 108.162.198.221 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 416 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 430 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 470 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 546 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 548 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 654 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 5 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 661 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 238 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 263 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 358 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 686 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 135 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 137 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 168 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 253 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 536 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 547 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 653 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 776 | 162.159.39.5 | 162.159.39.5 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 72 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 123 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 147 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 186 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 349 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 515 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 552 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 663 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 3 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 33 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 71 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 419 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 514 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 17 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 29 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 30 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 138 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 152 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 202 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 479 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 6 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 18 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 117 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 144 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 148 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 203 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 393 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 599 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 22 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 74 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 78 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 406 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 426 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 640 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 80 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 272 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 471 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 610 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 763 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 68 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 218 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 287 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 466 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 62 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 162 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 350 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 669 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 734 | 172.64.52.45 | 172.64.52.45 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 775 | 162.159.38.160 | 162.159.38.160 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 4 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 93 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 574 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 691 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 754 | 104.26.6.10 | 104.26.6.10 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 115 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 140 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 526 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 577 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 89 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 219 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 284 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 395 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 683 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 701 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 255 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 337 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 476 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 553 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 241 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 308 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 310 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 465 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 722 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 738 | 172.64.229.129 | 172.64.229.129 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 187 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 343 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 459 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 554 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 678 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 34 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 52 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 58 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 67 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 156 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 292 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 590 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 627 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 739 | 104.26.9.186 | 104.26.9.186 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 24 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 97 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 240 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 707 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 61 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 191 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 243 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 247 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 285 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 302 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 2 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 151 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 295 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 634 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 760 | 104.17.168.76 | 104.17.168.76 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 201 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 244 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 572 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 86 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 16 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 23 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 453 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 573 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 696 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 619 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 9 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 102 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 139 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 262 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 279 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 641 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 32 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 452 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 618 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 283 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 628 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 60 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 150 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 332 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 671 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 687 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 161 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 194 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 326 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 357 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 733 | 162.159.44.197 | 162.159.44.197 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 746 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 35 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 47 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 133 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 388 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 486 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 85 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 254 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 270 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 122 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 736 | 162.159.38.32 | 162.159.38.32 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 579 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 94 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 301 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 95 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 124 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 396 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 91 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 79 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 333 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 444 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 726 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 460 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 559 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 14 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 644 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 344 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 296 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 427 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 90 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 108 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 193 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 780 | 172.64.53.175 | 172.64.53.175 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 48 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 259 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 338 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 153 | cloudflare |
| 179 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 510 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 533 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 668 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 320 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 535 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 569 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 107 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 132 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare |
| 439 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 208 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 169 | cloudflare |
| 520 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 312 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 176 | cloudflare |
| 278 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 179 | cloudflare |
| 745 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare |
| 129 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 191 | cloudflare |
| 145 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 192 | cloudflare |
| 781 | 162.159.45.118 | 162.159.45.118 | IPv4 | h3 | ✅ 成功 | 195 | cloudflare |
| 606 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 196 | cloudflare |
| 492 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 203 | cloudflare |
| 31 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 206 | cloudflare |
| 356 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 224 | cloudflare |
| 12 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 318 | cloudflare |
| 560 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 350 | cloudflare |
| 391 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 497 | cloudflare |
| 324 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 508 | cloudflare |
| 361 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 520 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 57 条记录
- **快 (50-100ms)**: 291 条记录
- **正常 (100-200ms)**: 195 条记录
- **慢 (200-500ms)**: 6 条记录
- **很慢 (>500ms)**: 2 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 229 次

### 按协议统计

- **none**: 232 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
