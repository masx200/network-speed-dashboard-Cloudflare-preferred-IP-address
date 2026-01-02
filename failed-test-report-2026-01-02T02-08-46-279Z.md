# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/2 02:08:46
- **数据来源**: connectivity_results-20260102-020845.json
- **总测试数**: 721
- **失败测试数**: 221
- **成功测试数**: 500
- **失败率**: 30.65%
- **平均延迟**: 108.82ms
- **最小延迟**: 45ms
- **最大延迟**: 542ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/2 02:08:46
- **IP地址**: 172.177.209.209
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 218 次 (98.6%)
- **连接超时: I/O超时**: 3 次 (1.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (218 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 2 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 5 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 6 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 9 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 10 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 13 | www.ipget.net | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 14 | www.ipget.net | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 18 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 19 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 20 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 24 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 25 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 26 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 30 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 31 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 32 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 36 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 37 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 38 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 46 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 47 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 48 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 49 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 50 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 56 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 57 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 58 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 61 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 62 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 66 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:fb:e00f:f23d:42c6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:fb:e00f:f23d:42c6]:443: connect: network is unreachable |
| 70 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 71 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 72 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 78 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 79 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 83 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 84 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 85 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 89 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 90 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 91 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 94 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 95 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 98 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 99 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 103 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 104 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 105 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 108 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 109 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 115 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 116 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 124 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 125 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 126 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 129 | bestcf.030101.xyz | 2606:4700:0:7a67:3de4:1f89:27a3:6eeb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:7a67:3de4:1f89:27a3:6eeb]:443: connect: network is unreachable |
| 130 | bestcf.030101.xyz | 2606:4700::254f:35ca:8fda:9c69 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::254f:35ca:8fda:9c69]:443: connect: network is unreachable |
| 141 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 142 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 145 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 146 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 150 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 151 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 152 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 158 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 159 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 160 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 164 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 165 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 166 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 171 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 174 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 175 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 179 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 183 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 184 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 187 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 188 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 195 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 196 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 201 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 202 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 203 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 209 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 210 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 211 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 212 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 243 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 244 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 245 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 248 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 249 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 258 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 259 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 260 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 264 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 265 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 266 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 270 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 271 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 272 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 279 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 280 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 281 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 287 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 288 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 289 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 292 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 293 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 299 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 300 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 301 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 304 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 305 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 309 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 310 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 314 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 315 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 316 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 320 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 321 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 322 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 324 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 328 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 329 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 330 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 334 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 335 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 336 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 337 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 344 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 345 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 346 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 349 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 353 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 354 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 357 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 358 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 359 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 364 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 365 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 366 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 369 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 370 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 373 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 374 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 382 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 383 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 384 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 389 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 390 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 391 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 392 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 396 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 403 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 404 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 405 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 409 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 410 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 411 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 413 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 417 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 418 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 421 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 422 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 431 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 432 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 433 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 473 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 474 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 475 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 476 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 477 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 478 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 479 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 480 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 481 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 482 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 508 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 524 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 525 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 526 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 527 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 528 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 529 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 530 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 532 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 533 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 592 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 593 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 594 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 595 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 596 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 597 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 598 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 599 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 600 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 602 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 649 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 653 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 654 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 655 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 656 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 658 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 659 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 660 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 661 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 662 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 701 | 2a06:98c1:3108:69e8:8084:4e8:69fb:42f3 | 2a06:98c1:3108:69e8:8084:4e8:69fb:42f3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:69e8:8084:4e8:69fb:42f3]:443: connect: network is unreachable |
| 703 | 2a06:98c1:3100:6e15:b337:b7f3:946f:f64c | 2a06:98c1:3100:6e15:b337:b7f3:946f:f64c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:6e15:b337:b7f3:946f:f64c]:443: connect: network is unreachable |
| 704 | 2a06:98c1:310c:8b:38b6:2a7:3894:7893 | 2a06:98c1:310c:8b:38b6:2a7:3894:7893 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:8b:38b6:2a7:3894:7893]:443: connect: network is unreachable |
| 705 | 2a06:98c1:51:0:98:2c59:3632:e8dd | 2a06:98c1:51:0:98:2c59:3632:e8dd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:0:98:2c59:3632:e8dd]:443: connect: network is unreachable |
| 706 | 2a06:98c1:310a:0:ae62:421e:28df:1252 | 2a06:98c1:310a:0:ae62:421e:28df:1252 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:ae62:421e:28df:1252]:443: connect: network is unreachable |
| 707 | 2400:cb00:f00e:0:f0:52d4:e4e1:bcd8 | 2400:cb00:f00e:0:f0:52d4:e4e1:bcd8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:0:f0:52d4:e4e1:bcd8]:443: connect: network is unreachable |
| 708 | 2a06:98c1:51:9abb:2942:bacb:7903:ccf8 | 2a06:98c1:51:9abb:2942:bacb:7903:ccf8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:9abb:2942:bacb:7903:ccf8]:443: connect: network is unreachable |
| 709 | 2606:4700:50:8972:9b9d:22a4:4f8:5a18 | 2606:4700:50:8972:9b9d:22a4:4f8:5a18 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:50:8972:9b9d:22a4:4f8:5a18]:443: connect: network is unreachable |
| 710 | 2a06:98c1:310b:aa90:9623:824a:3237:5427 | 2a06:98c1:310b:aa90:9623:824a:3237:5427 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:aa90:9623:824a:3237:5427]:443: connect: network is unreachable |
| 711 | 2a06:98c1:310d:0:b658:30d1:bf2:8feb | 2a06:98c1:310d:0:b658:30d1:bf2:8feb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:0:b658:30d1:bf2:8feb]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 719 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 720 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 721 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 218 次 (98.6%)
- **连接超时**: 3 次 (1.4%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 221 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 218 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), sullivan.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 500 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 576 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 238 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 573 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 385 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 555 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 551 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 127 | bestcf.030101.xyz | 104.17.99.183 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 485 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 490 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 673 | 104.18.47.169 | 104.18.47.169 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 434 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 147 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 242 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 355 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 629 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 689 | 104.18.95.45 | 104.18.95.45 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 319 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 547 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 559 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 425 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 553 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 556 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 568 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 462 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 587 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 618 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 416 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 451 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 539 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 128 | bestcf.030101.xyz | 104.17.27.231 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 420 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 459 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 496 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 542 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 135 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 621 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 647 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 698 | 172.64.229.2 | 172.64.229.2 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 575 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 692 | 104.17.209.156 | 104.17.209.156 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 695 | 104.18.92.221 | 104.18.92.221 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 697 | 104.17.171.35 | 104.17.171.35 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 617 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 680 | 104.20.18.138 | 104.20.18.138 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 567 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 510 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 437 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 356 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 440 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 520 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 669 | 104.18.44.178 | 104.18.44.178 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 690 | 104.17.174.10 | 104.17.174.10 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 133 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 168 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 625 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 77 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 173 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 650 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 516 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 536 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 648 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 117 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 194 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 501 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 506 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 513 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 624 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 668 | 104.18.36.213 | 104.18.36.213 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 140 | cloudflare-ip.mofashi.ltd | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 560 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 614 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 633 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 699 | 172.64.52.8 | 172.64.52.8 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 73 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 291 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 143 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 430 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 45 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 155 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 367 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 427 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 579 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 634 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 170 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 522 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 641 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 712 | 162.159.10.211 | 162.159.10.211 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 121 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 393 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 439 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 561 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 619 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 40 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 122 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 204 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 340 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 423 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 639 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 645 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 93 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 398 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 461 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 534 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 549 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 581 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 691 | 104.19.48.127 | 104.19.48.127 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 139 | cloudflare-ip.mofashi.ltd | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 208 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 295 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 394 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 503 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 69 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 406 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 512 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 557 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 43 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 80 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 262 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 286 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 318 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 372 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 463 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 471 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 489 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 500 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 543 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 687 | 104.16.155.38 | 104.16.155.38 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 715 | 172.64.53.222 | 172.64.53.222 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 181 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 554 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 663 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 3 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 347 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 360 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 368 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 448 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 491 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 298 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 375 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 408 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 455 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 515 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 584 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 630 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 39 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 333 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 415 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 438 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 441 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 469 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 487 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 580 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 674 | 104.18.33.200 | 104.18.33.200 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 52 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 161 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 307 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 350 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 400 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 402 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 450 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 509 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 523 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 42 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 112 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 118 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 197 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 273 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 452 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 472 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 498 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 640 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 644 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 684 | 172.67.76.194 | 172.67.76.194 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 7 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 17 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 106 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 169 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 182 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 235 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 256 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 363 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 377 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 484 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 493 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 583 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 615 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 676 | 104.26.3.78 | 104.26.3.78 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 682 | 104.20.19.4 | 104.20.19.4 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 114 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 153 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 192 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 207 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 274 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 294 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 397 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 407 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 552 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 616 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 34 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 131 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 154 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 317 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 449 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 483 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 514 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 578 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 213 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 284 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 297 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 436 | www.7749tv.com | 104.24.160.160 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 465 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 470 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 541 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 582 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 609 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 636 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 16 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 302 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 326 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 558 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 664 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 679 | 104.26.14.114 | 104.26.14.114 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 693 | 104.19.54.245 | 104.19.54.245 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 296 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 381 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 504 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 517 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 569 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 605 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 627 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 646 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 681 | 104.20.26.29 | 104.20.26.29 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 702 | 172.64.38.159 | 172.64.38.159 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 15 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 123 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 162 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 172 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 189 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 250 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 269 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 341 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 435 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 492 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 612 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 76 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 97 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 136 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 137 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 149 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 277 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 445 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 453 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 585 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 635 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 696 | 162.159.38.238 | 162.159.38.238 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 60 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 67 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 110 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 190 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 200 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 254 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 263 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 343 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 351 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 562 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 632 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 102 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 156 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 303 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 378 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 511 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 521 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 570 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 606 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 671 | 173.245.59.26 | 173.245.59.26 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 713 | 162.159.44.20 | 162.159.44.20 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 54 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 92 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 379 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 395 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 458 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 505 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 586 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 700 | 108.162.198.173 | 108.162.198.173 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 180 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 253 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 507 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 589 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 637 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 177 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 186 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 255 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 257 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 457 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 518 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 601 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 604 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 35 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 68 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 278 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 331 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 332 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 388 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 401 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 499 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 502 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 548 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 563 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 657 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 683 | 104.20.29.50 | 104.20.29.50 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 8 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 27 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 120 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 380 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 443 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 456 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 467 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 535 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 572 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 574 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 29 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 119 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 247 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 323 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 442 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 608 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 113 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 176 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 199 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 240 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 282 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 325 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 342 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 428 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 537 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 607 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 667 | 162.159.3.40 | 162.159.3.40 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 678 | 104.17.154.94 | 104.17.154.94 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 718 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 4 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 74 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 134 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 138 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 185 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 339 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 571 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 588 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 620 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 631 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 716 | 162.159.45.134 | 162.159.45.134 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 51 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 100 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 290 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 426 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 460 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 488 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 111 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 193 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 348 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 468 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 665 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 670 | 172.64.147.232 | 172.64.147.232 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 41 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 65 | freeyx.cloudflare88.eu.org | 141.101.120.40 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 191 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 205 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 267 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 399 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 675 | 172.64.146.218 | 172.64.146.218 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 424 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 566 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 677 | 104.18.35.200 | 104.18.35.200 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 64 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 101 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 206 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 239 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 268 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 386 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 672 | 162.159.21.39 | 162.159.21.39 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 55 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 87 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 163 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 178 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 251 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 564 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 86 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 275 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 531 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 546 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 666 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 686 | 104.20.20.225 | 104.20.20.225 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 283 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 371 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 519 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 642 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 53 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 241 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 313 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 688 | 172.67.69.87 | 172.67.69.87 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 107 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 132 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 148 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 11 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 167 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 338 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 352 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 651 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 81 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 144 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 446 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 447 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 540 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 63 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 261 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 444 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 497 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 361 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 494 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 538 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 590 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 714 | 162.159.39.246 | 162.159.39.246 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 59 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 466 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 75 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 419 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 550 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 565 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 652 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 685 | 104.26.1.232 | 104.26.1.232 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 96 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 387 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 603 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 611 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 613 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 464 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 376 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 28 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 414 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 577 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 198 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 412 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 643 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 308 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 276 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 327 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 362 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 622 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 252 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 23 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 285 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 591 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 82 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 311 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 153 | cloudflare |
| 306 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 610 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 545 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 157 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 160 | cloudflare |
| 486 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 162 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 638 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare |
| 694 | 172.64.145.205 | 172.64.145.205 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 12 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 183 | cloudflare |
| 312 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare |
| 429 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 189 | cloudflare |
| 44 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 190 | cloudflare |
| 246 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 205 | cloudflare |
| 628 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 205 | cloudflare |
| 33 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 216 | cloudflare |
| 88 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 216 | cloudflare |
| 626 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 237 | cloudflare |
| 623 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 247 | cloudflare |
| 544 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 382 | cloudflare |
| 495 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 536 | cloudflare |
| 717 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 539 | cloudflare |
| 454 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 542 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 196 条记录
- **正常 (100-200ms)**: 293 条记录
- **慢 (200-500ms)**: 7 条记录
- **很慢 (>500ms)**: 3 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 218 次

### 按协议统计

- **none**: 221 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
