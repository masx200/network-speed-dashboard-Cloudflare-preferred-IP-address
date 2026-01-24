# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/24 00:08:19
- **数据来源**: connectivity_results-20260124-000819.json
- **总测试数**: 781
- **失败测试数**: 231
- **成功测试数**: 550
- **失败率**: 29.58%
- **平均延迟**: 96.05ms
- **最小延迟**: 40ms
- **最大延迟**: 497ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/24 00:08:20
- **IP地址**: 64.236.161.38
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

- **网络不可达: 网络不可达**: 228 次 (98.7%)
- **连接超时: I/O超时**: 3 次 (1.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (228 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | 2a06:98c1:310a:c1:ab96:14d6:b31a:6d03 | 2a06:98c1:310a:c1:ab96:14d6:b31a:6d03 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:c1:ab96:14d6:b31a:6d03]:443: connect: network is unreachable |
| 2 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 8 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 9 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 10 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 14 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 15 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 16 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 27 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 28 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 31 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 32 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 35 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 36 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 42 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 43 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 47 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 48 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 53 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 54 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 58 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 59 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 63 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 64 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 65 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 72 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 73 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 74 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 79 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 80 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 81 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 84 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 85 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 91 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 92 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 93 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 94 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 95 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 98 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 102 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 103 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 104 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 141 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 142 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 148 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 149 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 160 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 161 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 162 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 166 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 167 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 168 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 172 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 173 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 174 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 178 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 179 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 180 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 184 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 185 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 186 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 190 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 191 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 192 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 194 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4f66:caf5]:443: connect: network is unreachable |
| 198 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 199 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 200 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 206 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 207 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 212 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 213 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 214 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 220 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 221 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 222 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 242 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 243 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 244 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 245 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 246 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 247 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 248 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 249 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 250 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 251 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 257 | bestcf.030101.xyz | 2606:4700:0:b21c:66ba:fef:2ef4:2658 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:b21c:66ba:fef:2ef4:2658]:443: connect: network is unreachable |
| 258 | bestcf.030101.xyz | 2606:4700:0:26e6:9bb0:8293:779:dc27 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:26e6:9bb0:8293:779:dc27]:443: connect: network is unreachable |
| 314 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 315 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 316 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 317 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 318 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 319 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 320 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 321 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 322 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 323 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 340 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 341 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 352 | 2803:f800:50:41ba:5536:115f:5191:1805 | 2803:f800:50:41ba:5536:115f:5191:1805 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:41ba:5536:115f:5191:1805]:443: connect: network is unreachable |
| 355 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 356 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 359 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 370 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 371 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 378 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 379 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 380 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 392 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 421 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 422 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 423 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 424 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 425 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 426 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 427 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 428 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 430 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 431 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 440 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 441 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 442 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 454 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 455 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 481 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 482 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 483 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 484 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 485 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 486 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 487 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 488 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 489 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 490 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 531 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 532 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 533 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 535 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 536 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 537 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 538 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 539 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 540 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 541 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 559 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 565 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 568 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 569 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 575 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 576 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 577 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 581 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 582 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 583 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 586 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 587 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 591 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 592 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 593 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 595 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 599 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 600 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 601 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 608 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 609 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 610 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 615 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 616 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 620 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 621 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 622 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 627 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 628 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 629 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 634 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 635 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 636 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 640 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 641 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 642 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 646 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 647 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 650 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 651 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 655 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 656 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 657 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 660 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 661 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 665 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 666 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 667 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 668 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 675 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 676 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 677 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 681 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 682 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 683 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 684 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 689 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 690 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 706 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 707 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 710 | 2a06:98c1:3103:ca:fa78:84d3:2596:f679 | 2a06:98c1:3103:ca:fa78:84d3:2596:f679 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:ca:fa78:84d3:2596:f679]:443: connect: network is unreachable |
| 717 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 718 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 721 | 2a06:98c1:3109:ecc2:5371:912b:e506:cd47 | 2a06:98c1:3109:ecc2:5371:912b:e506:cd47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:ecc2:5371:912b:e506:cd47]:443: connect: network is unreachable |
| 722 | 2803:f800:51:b883:8fe2:9a40:3016:f74 | 2803:f800:51:b883:8fe2:9a40:3016:f74 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:b883:8fe2:9a40:3016:f74]:443: connect: network is unreachable |
| 724 | 2400:cb00:2049:b1aa:a619:c233:939c:74 | 2400:cb00:2049:b1aa:a619:c233:939c:74 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:b1aa:a619:c233:939c:74]:443: connect: network is unreachable |
| 725 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 728 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 729 | 2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf | 2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:70:d7a5:3f5:bb52:dfbf]:443: connect: network is unreachable |
| 730 | 2a06:98c1:3105::4c1a:e9f2:d41e | 2a06:98c1:3105::4c1a:e9f2:d41e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::4c1a:e9f2:d41e]:443: connect: network is unreachable |
| 731 | 2a06:98c1:3107:0:f6:2182:d4a8:339f | 2a06:98c1:3107:0:f6:2182:d4a8:339f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:f6:2182:d4a8:339f]:443: connect: network is unreachable |
| 735 | 2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2 | 2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:6020:3bcb:4a0f:8160:4ca2]:443: connect: network is unreachable |
| 739 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 740 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 761 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 762 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 763 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 767 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 768 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 769 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 774 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 775 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 776 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 711 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 754 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 781 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 228 次 (98.7%)
- **连接超时**: 3 次 (1.3%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.67（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 231 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 228 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 550 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 216 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 449 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 464 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 344 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 404 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 405 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 453 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 612 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 670 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 696 | 104.18.80.38 | 104.18.80.38 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 107 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 226 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 756 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 358 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 407 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 409 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 500 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 195 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 255 | bestcf.030101.xyz | 104.18.94.61 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 385 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 398 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 467 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 504 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 520 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 543 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 669 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 201 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 400 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 460 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 633 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 659 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 700 | 104.16.156.40 | 104.16.156.40 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 702 | 104.19.60.36 | 104.19.60.36 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 758 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 144 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 158 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 369 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 374 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 408 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 436 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 451 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 461 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 685 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 452 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 534 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 663 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 770 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 52 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 78 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 267 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 499 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 505 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 507 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 551 | 172.64.52.45 | 172.64.52.45 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 614 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 658 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 692 | 104.26.3.109 | 104.26.3.109 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 755 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 133 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 228 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 196 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 266 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 286 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 389 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 438 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 513 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 693 | 104.26.6.10 | 104.26.6.10 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 737 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 151 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 297 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 351 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 525 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 645 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 698 | 104.17.145.189 | 104.17.145.189 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 40 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 271 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 382 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 523 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 552 | 172.64.229.129 | 172.64.229.129 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 570 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 716 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 132 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 177 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 290 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 468 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 773 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 720 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 777 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 298 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 413 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 626 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 637 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 648 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 302 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 529 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 772 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 415 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 618 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 687 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 734 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 131 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 252 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 274 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 439 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 613 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 61 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 375 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 603 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 193 | freeyx.cloudflare88.eu.org | 141.101.120.161 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 279 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 723 | 172.64.48.160 | 172.64.48.160 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 227 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 304 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 362 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 502 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 606 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 653 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 203 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 396 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 450 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 511 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 514 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 719 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 197 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 437 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 556 | 104.26.2.45 | 104.26.2.45 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 204 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 699 | 104.17.153.22 | 104.17.153.22 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 459 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 465 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 580 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 159 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 239 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 240 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 283 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 501 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 542 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 757 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 272 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 406 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 90 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 445 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 560 | 104.26.15.22 | 104.26.15.22 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 579 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 292 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 296 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 376 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 738 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 742 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 106 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 256 | bestcf.030101.xyz | 104.19.156.118 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 462 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 517 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 589 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 631 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 434 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 496 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 510 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 596 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 714 | 172.64.53.175 | 172.64.53.175 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 745 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 748 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 39 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 49 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 137 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 143 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 275 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 312 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 515 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 530 | 104.26.7.8 | 104.26.7.8 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 68 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 295 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 309 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 348 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 448 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 562 | 104.26.11.221 | 104.26.11.221 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 643 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 82 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 109 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 136 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 233 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 354 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 62 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 67 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 150 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 349 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 365 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 366 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 474 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 544 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 573 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 736 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 24 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 97 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 128 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 327 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 331 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 469 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 478 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 493 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 764 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 45 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 69 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 86 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 138 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 183 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 268 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 521 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 545 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 46 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 338 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 528 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 554 | 104.18.37.234 | 104.18.37.234 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 662 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 21 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 119 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 345 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 346 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 361 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 457 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 704 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 750 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 96 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 211 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 477 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 664 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 116 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 231 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 265 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 285 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 417 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 433 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 498 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 547 | 162.159.39.75 | 162.159.39.75 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 360 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 384 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 473 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 549 | 162.159.44.197 | 162.159.44.197 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 205 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 342 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 357 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 566 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 726 | 162.159.44.235 | 162.159.44.235 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 83 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 124 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 187 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 260 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 479 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 555 | 172.64.48.78 | 172.64.48.78 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 691 | 162.159.45.118 | 162.159.45.118 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 778 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 202 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 390 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 522 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 524 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 574 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 602 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 746 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 125 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 126 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 241 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 330 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 397 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 506 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 519 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 644 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 673 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 22 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 33 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 44 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 111 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 112 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 289 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 300 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 326 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 567 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 727 | 172.64.52.145 | 172.64.52.145 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 147 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 308 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 329 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 563 | 104.26.5.55 | 104.26.5.55 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 18 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 108 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 157 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 238 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 280 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 353 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 619 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 60 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 105 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 165 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 229 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 291 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 301 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 399 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 518 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 607 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 117 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 262 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 264 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 281 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 386 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 553 | 162.159.38.32 | 162.159.38.32 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 572 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 219 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 305 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 310 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 334 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 373 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 432 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 527 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 639 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 50 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 225 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 548 | 108.162.198.97 | 108.162.198.97 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 694 | 104.26.1.119 | 104.26.1.119 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 88 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 101 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 223 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 276 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 336 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 402 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 470 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 564 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 584 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 713 | 162.159.38.160 | 162.159.38.160 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 114 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 277 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 293 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 303 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 313 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 343 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 466 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 497 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 697 | 104.17.168.76 | 104.17.168.76 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 134 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 189 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 306 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 503 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 594 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 678 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 130 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 259 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 446 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 475 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 480 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 494 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 550 | 172.64.53.182 | 172.64.53.182 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 590 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 26 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 30 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 38 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 210 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 328 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 364 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 416 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 491 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 37 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 70 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 155 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 208 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 217 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 236 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 288 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 377 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 632 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 649 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 712 | 162.159.34.8 | 162.159.34.8 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 153 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 253 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 403 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 476 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 509 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 571 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 625 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 674 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 765 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 7 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 118 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 333 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 339 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 394 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 71 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 89 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 127 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 171 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 311 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 388 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 420 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 472 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 672 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 145 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 299 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 383 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 146 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 414 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 456 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 753 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 99 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 110 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 367 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 463 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 471 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 623 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 652 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 123 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 135 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 215 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 447 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 492 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 732 | 108.162.198.221 | 108.162.198.221 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 154 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 235 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 269 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 273 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 561 | 104.26.9.186 | 104.26.9.186 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 624 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 766 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 779 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 237 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 695 | 104.16.154.33 | 104.16.154.33 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 701 | 104.17.109.58 | 104.17.109.58 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 75 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 113 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 120 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 121 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 337 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 557 | 104.20.16.97 | 104.20.16.97 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 679 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 705 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 733 | 162.159.39.5 | 162.159.39.5 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 19 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 56 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 169 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 278 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 611 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 749 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 139 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 163 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 164 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 224 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 294 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 307 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 638 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 100 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 230 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 254 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 419 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 617 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 743 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 29 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 347 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 630 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 13 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 335 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 715 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 741 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 57 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 115 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 182 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 23 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 209 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 261 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 429 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 688 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 752 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 325 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 411 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 680 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 25 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 122 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 350 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 76 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 760 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 709 | 172.64.229.167 | 172.64.229.167 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 284 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 387 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 395 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 412 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 3 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 6 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 443 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 597 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 751 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 175 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 282 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 512 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 232 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 270 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 435 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 11 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 77 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 368 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 444 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 20 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 176 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 418 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 508 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 516 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 17 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 372 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 4 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 391 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 605 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 703 | 104.17.106.109 | 104.17.106.109 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 34 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 495 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 546 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 181 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 234 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 287 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 332 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 771 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 188 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 381 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 140 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 401 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 458 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 41 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 66 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 578 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 654 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 759 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 218 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 588 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 526 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 12 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 51 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 324 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 263 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 162 | cloudflare |
| 671 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 164 | cloudflare |
| 747 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 55 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 170 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 558 | 162.159.45.11 | 162.159.45.11 | IPv4 | h3 | ✅ 成功 | 181 | cloudflare |
| 87 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 183 | cloudflare |
| 604 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 188 | cloudflare |
| 598 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 192 | cloudflare |
| 393 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 198 | cloudflare |
| 410 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 208 | cloudflare |
| 585 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 217 | cloudflare |
| 5 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 222 | cloudflare |
| 363 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 257 | cloudflare |
| 129 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 290 | cloudflare |
| 744 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 336 | cloudflare |
| 780 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 348 | cloudflare |
| 708 | 104.16.249.149 | 104.16.249.149 | IPv4 | h3 | ✅ 成功 | 352 | cloudflare |
| 686 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 474 | cloudflare |
| 152 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 493 | cloudflare |
| 156 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 497 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 47 条记录
- **快 (50-100ms)**: 279 条记录
- **正常 (100-200ms)**: 213 条记录
- **慢 (200-500ms)**: 11 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 228 次

### 按协议统计

- **none**: 231 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
