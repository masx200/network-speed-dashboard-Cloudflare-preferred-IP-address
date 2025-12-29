# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:17:42
- **数据来源**: connectivity_results-20251229-171742.json
- **总测试数**: 495
- **失败测试数**: 181
- **成功测试数**: 314
- **失败率**: 36.57%
- **平均延迟**: 76.71ms
- **最小延迟**: 35ms
- **最大延迟**: 486ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:17:42
- **IP地址**: 52.159.225.196
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

- **网络不可达: 网络不可达**: 178 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (178 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 6 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 7 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 8 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 11 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 12 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 15 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 16 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 27 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 28 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 31 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 32 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 36 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 37 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 38 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 41 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 42 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 46 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 47 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 54 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 55 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 56 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 61 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 62 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 63 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 70 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 71 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 72 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 73 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 74 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 78 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 79 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 80 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 88 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 89 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 93 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 94 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 95 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 99 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 100 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 101 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 102 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 131 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 132 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 140 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 141 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 150 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 151 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 152 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 155 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 156 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 165 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 166 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 167 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 170 | freeyx.cloudflare88.eu.org | 2606:4700:130:e73c:228:ba38:b230:c33f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:130:e73c:228:ba38:b230:c33f]:443: connect: network is unreachable |
| 177 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 178 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 179 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 183 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 184 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 185 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 189 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 190 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 191 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 195 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 196 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 197 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 200 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 201 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 206 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 207 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 208 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 216 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 217 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 218 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 219 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 222 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 223 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 226 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 227 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 231 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 232 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 233 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 236 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 237 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 239 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 242 | bestcf.030101.xyz | 2606:4700:0:c5:4803:8845:8bde:1897 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:c5:4803:8845:8bde:1897]:443: connect: network is unreachable |
| 243 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:e7ac:854f:c15c:d3b1:fc6a]:443: connect: network is unreachable |
| 246 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 247 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 252 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 253 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 254 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 258 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 259 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 260 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 266 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 267 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 268 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 272 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 273 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 277 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 278 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 279 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 286 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 287 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 288 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 292 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 293 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 294 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 302 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 303 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 307 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 308 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 309 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 313 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 314 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 315 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 318 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 319 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 321 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 325 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 326 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 327 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 330 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 331 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 332 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 339 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 340 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 341 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 343 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 348 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 349 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 350 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 354 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 355 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 356 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 357 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 362 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 363 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 364 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 367 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 368 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 375 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 376 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 377 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 384 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 385 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 386 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 390 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 391 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 393 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 397 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 398 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 399 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 405 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 406 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 407 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 411 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 412 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 413 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 415 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 430 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 431 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 437 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 438 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 459 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 463 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 464 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 465 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 475 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 476 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 477 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 478 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 479 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 480 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 481 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 482 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 488 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 493 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 495 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 289 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 388 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 409 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 234 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 262 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 282 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 359 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 378 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 441 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 60 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 143 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 271 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 284 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 420 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 443 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 450 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 417 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 447 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 209 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 296 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 456 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 370 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 428 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 52 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 310 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 387 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 427 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 466 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 204 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 281 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 109 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 176 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 211 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 442 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 426 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 90 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 346 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 453 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 487 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 10 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 35 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 107 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 305 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 424 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 138 | icook.hk | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 244 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 26 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 194 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 320 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 17 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 49 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 118 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 128 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 161 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 168 | freeyx.cloudflare88.eu.org | 141.101.121.38 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 469 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 474 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 108 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 111 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 419 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 110 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 19 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 316 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 374 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 115 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 299 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 467 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 249 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 122 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 416 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 120 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 180 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 285 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 446 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 491 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 22 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 43 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 106 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 171 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 182 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 297 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 429 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 20 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 24 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 57 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 163 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 230 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 335 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 365 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 304 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 389 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 418 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 45 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 112 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 134 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 213 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 221 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 317 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 433 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 436 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 444 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 451 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 457 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 458 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 21 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 53 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 91 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 116 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 154 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 159 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 198 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 210 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 220 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 270 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 276 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 51 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 58 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 124 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 145 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 23 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 87 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 127 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 139 | icook.hk | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 153 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 169 | freeyx.cloudflare88.eu.org | 141.101.121.109 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 224 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 336 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 338 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 372 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 439 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 454 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 485 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 121 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 129 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 181 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 215 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 261 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 333 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 380 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 422 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 187 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 263 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 311 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 322 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 344 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 351 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 395 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 81 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 126 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 369 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 423 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 432 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 85 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 238 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 290 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 97 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 142 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 228 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 468 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 18 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 29 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 39 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 69 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 162 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 235 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 264 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 300 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 396 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 401 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 64 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 136 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 205 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 404 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 471 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 86 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 137 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 157 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 275 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 360 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 470 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 489 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 5 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 125 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 192 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 251 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 265 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 295 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 298 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 421 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 96 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 329 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 371 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 403 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 76 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 135 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 306 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 448 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |

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
- **IPv6 失败**: 178 次

### 按协议统计

- **none**: 181 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
