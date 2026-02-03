# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/3 19:45:33
- **数据来源**: connectivity_results-20260203-194533.json
- **总测试数**: 712
- **失败测试数**: 215
- **成功测试数**: 497
- **失败率**: 30.20%
- **平均延迟**: 230.43ms
- **最小延迟**: 46ms
- **最大延迟**: 9944ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/3 19:45:33
- **IP地址**: 172.178.117.218
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 38.7095, -78.1539
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 206 次 (95.8%)
- **连接超时: I/O超时**: 4 次 (1.9%)
- **DNS解析错误: 其他DNS错误**: 3 次 (1.4%)
- **连接超时: 上下文超时**: 2 次 (0.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (206 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 5 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 6 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 17 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 18 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 24 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 25 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 26 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 27 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 28 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 29 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 30 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 32 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 33 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 37 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 38 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 68 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 74 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 75 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 76 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 90 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 91 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 92 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 95 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 96 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 100 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 101 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 102 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 106 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 107 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 108 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 112 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 113 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 114 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 119 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 120 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 128 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 129 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 130 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 134 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 135 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 136 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 140 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 141 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 142 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 149 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 150 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 151 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 155 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 156 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 157 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 160 | freeyx.cloudflare88.eu.org | 2606:4700:3010:bf:5dba:14ae:9501:48f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:bf:5dba:14ae:9501:48f7]:443: connect: network is unreachable |
| 163 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 164 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 166 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 167 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 173 | bestcf.030101.xyz | 2606:4700::8d:f082:8938:66d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::8d:f082:8938:66d8]:443: connect: network is unreachable |
| 174 | bestcf.030101.xyz | 2606:4700::fffd:819d:acda | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::fffd:819d:acda]:443: connect: network is unreachable |
| 175 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 176 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 177 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 181 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 182 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 183 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 184 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 185 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 186 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 190 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 191 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 192 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 195 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 196 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 198 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 216 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 217 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 221 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 222 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 223 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 224 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 225 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 233 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 234 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 235 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 247 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 248 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 252 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 253 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 254 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 258 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 259 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 260 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 261 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 262 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 263 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 270 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 271 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 272 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 273 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 274 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 275 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 277 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 284 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 285 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 287 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 294 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 295 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 296 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 299 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 302 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 303 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 306 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 309 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 310 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 316 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 317 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 318 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 322 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 323 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 324 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 328 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 329 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 330 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 331 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 333 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 335 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 339 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 340 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 341 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 345 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 346 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 358 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 359 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 360 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 378 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 379 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 380 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 395 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 396 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 405 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 406 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 407 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 408 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 409 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 410 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 411 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 412 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 413 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 414 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 421 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 456 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 457 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 458 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 459 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 460 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 461 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 462 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 463 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 464 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 465 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 504 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 505 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 520 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 521 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 522 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 533 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 534 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 535 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 536 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 537 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 538 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 539 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 540 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 541 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 542 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 588 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 593 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 594 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 595 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 596 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 597 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 598 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 599 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 600 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 601 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 602 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 644 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 645 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 646 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 647 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 648 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 649 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 650 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 651 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 652 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 690 | 2a06:98c1:3100:f7e3:cbcf:b6c7:6124:2fe2 | 2a06:98c1:3100:f7e3:cbcf:b6c7:6124:2fe2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f7e3:cbcf:b6c7:6124:2fe2]:443: connect: network is unreachable |
| 693 | 2a06:98c1:3108:f98c:209d:50fc:b788:b65c | 2a06:98c1:3108:f98c:209d:50fc:b788:b65c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:f98c:209d:50fc:b788:b65c]:443: connect: network is unreachable |
| 694 | 2400:cb00:f00e:d69d:66dd:1321:64a4:5076 | 2400:cb00:f00e:d69d:66dd:1321:64a4:5076 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:d69d:66dd:1321:64a4:5076]:443: connect: network is unreachable |
| 695 | 2400:cb00:f00e:d382:fa2e:2623:6358:92de | 2400:cb00:f00e:d382:fa2e:2623:6358:92de | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:d382:fa2e:2623:6358:92de]:443: connect: network is unreachable |
| 696 | 2a06:98c1:310c:96ca:8a3d:395:146c:c376 | 2a06:98c1:310c:96ca:8a3d:395:146c:c376 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:96ca:8a3d:395:146c:c376]:443: connect: network is unreachable |
| 697 | 2a06:98c1:3108:f969:876d:b1a7:497d:57dd | 2a06:98c1:3108:f969:876d:b1a7:497d:57dd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:f969:876d:b1a7:497d:57dd]:443: connect: network is unreachable |
| 698 | 2a06:98c1:310d:f860:89a7:7bc8:11ef:4685 | 2a06:98c1:310d:f860:89a7:7bc8:11ef:4685 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:f860:89a7:7bc8:11ef:4685]:443: connect: network is unreachable |
| 699 | 2a06:98c1:310b:636a:5598:d530:8b94:e23c | 2a06:98c1:310b:636a:5598:d530:8b94:e23c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:636a:5598:d530:8b94:e23c]:443: connect: network is unreachable |
| 700 | 2a06:98c1:3107:1c8c:c953:32f0:877e:d809 | 2a06:98c1:3107:1c8c:c953:32f0:877e:d809 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:1c8c:c953:32f0:877e:d809]:443: connect: network is unreachable |
| 701 | 2a06:98c1:3100:0:92d7:326:fa62:11cf | 2a06:98c1:3100:0:92d7:326:fa62:11cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:92d7:326:fa62:11cf]:443: connect: network is unreachable |
| 707 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 708 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 709 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 165 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 201 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | none | N/A | 0 | N/A | dial tcp 104.20.255.53:443: i/o timeout |
| 212 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 366 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

#### DNS解析错误: 其他DNS错误 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 122 | www.4chan.org | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析无结果 |
| 226 | asia.877774.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析无结果 |
| 354 | eur.877774.xyz | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析无结果 |

#### 连接超时: 上下文超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 347 | ashton.ns.cloudflare.com | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 660 | www.wto.org | Unknown | Unknown | none | N/A | 0 | N/A | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 206 次 (95.8%)
- **连接超时**: 6 次 (2.8%)
- **DNS解析错误**: 3 次 (1.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 215 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 206 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), huxley.ns.cloudflare.com (3次), iplocation.io (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 497 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 638 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 586 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 642 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 283 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 674 | 104.17.147.230 | 104.17.147.230 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 584 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 279 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 661 | 172.64.229.246 | 172.64.229.246 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 439 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 84 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 625 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 637 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 635 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 688 | 108.162.198.248 | 108.162.198.248 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 257 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 626 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 630 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 663 | 172.64.34.185 | 172.64.34.185 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 232 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 686 | 172.64.52.178 | 172.64.52.178 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 567 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 453 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 269 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 527 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 36 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 606 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 469 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 632 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 40 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 583 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 19 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 636 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 631 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 320 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 489 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 21 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 44 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 137 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 608 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 45 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 433 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 478 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 557 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 711 | 172.67.76.81 | 172.67.76.81 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 214 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 66 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 124 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 612 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 49 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 319 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 58 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 276 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 531 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 65 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 159 | freeyx.cloudflare88.eu.org | 141.101.120.232 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 213 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 417 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 444 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 476 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 85 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 289 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 321 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 503 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 111 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 570 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 580 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 675 | 104.17.30.217 | 104.17.30.217 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 143 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 415 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 662 | 172.64.50.250 | 172.64.50.250 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 666 | 172.67.78.158 | 172.67.78.158 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 683 | 104.17.148.211 | 104.17.148.211 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 48 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 426 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 467 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 587 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 9 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 78 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 154 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 301 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 389 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 585 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 46 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 11 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 55 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 133 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 197 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 298 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 454 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 495 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 574 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 576 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 685 | 162.159.39.132 | 162.159.39.132 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 8 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 83 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 88 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 304 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 532 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 664 | 172.67.65.106 | 172.67.65.106 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 689 | 162.159.45.21 | 162.159.45.21 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 47 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 246 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 367 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 450 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 483 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 624 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 634 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 400 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 628 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 60 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 639 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 641 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 171 | bestcf.030101.xyz | 104.17.144.235 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 383 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 571 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 677 | 104.17.215.215 | 104.17.215.215 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 42 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 239 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 286 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 332 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 554 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 31 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 34 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 67 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 227 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 229 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 308 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 555 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 622 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 655 | 108.162.198.121 | 108.162.198.121 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 658 | 162.159.38.203 | 162.159.38.203 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 52 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 549 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 131 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 205 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 418 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 425 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 497 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 121 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 265 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 267 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 278 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 325 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 438 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 568 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 681 | 104.17.211.235 | 104.17.211.235 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 110 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 424 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 449 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 490 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 543 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 680 | 104.17.155.181 | 104.17.155.181 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 240 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 348 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 384 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 523 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 416 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 529 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 352 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 419 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 546 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 562 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 657 | 162.159.44.59 | 162.159.44.59 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 35 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 53 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 169 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 364 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 387 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 430 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 440 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 516 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 592 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 300 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 353 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 582 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 605 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 654 | 162.159.45.120 | 162.159.45.120 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 7 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 180 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 204 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 207 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 251 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 507 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 544 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 548 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 194 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 230 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 243 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 311 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 282 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 338 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 627 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 706 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 264 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 617 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 640 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 172 | bestcf.030101.xyz | 172.64.159.213 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 679 | 104.17.51.240 | 104.17.51.240 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 710 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 10 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 138 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 145 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 399 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 498 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 553 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 561 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 39 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 56 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 189 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 403 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 436 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 609 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 619 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 158 | freeyx.cloudflare88.eu.org | 141.101.120.150 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 368 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 445 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 614 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 146 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 168 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 170 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 237 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 290 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 362 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 479 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 510 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 73 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 215 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 313 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 610 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 618 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 50 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 187 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 397 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 427 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 569 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 615 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 209 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 220 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 363 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 466 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 80 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 98 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 392 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 578 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 139 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 266 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 492 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 525 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 575 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 671 | 162.159.46.137 | 162.159.46.137 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 682 | 104.17.149.8 | 104.17.149.8 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 305 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 342 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 475 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 499 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 705 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 41 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 51 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 178 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 437 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 477 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 116 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 241 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 491 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 691 | 162.159.44.173 | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 12 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 144 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 218 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 515 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 4 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 203 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 256 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 514 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 676 | 104.17.117.119 | 104.17.117.119 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 249 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 356 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 398 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 452 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 455 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 591 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 603 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 231 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 293 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 134 | cloudflare |
| 443 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 607 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 208 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 228 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 292 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 327 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 385 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 471 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 484 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 487 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 653 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 70 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 87 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 494 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 69 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 109 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 448 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 556 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 672 | 172.67.71.67 | 172.67.71.67 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 105 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 123 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 376 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 547 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 451 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 473 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 493 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 621 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 643 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 61 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 63 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 388 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 429 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 16 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 350 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 435 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 512 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 43 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 365 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 572 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 611 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 23 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 255 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 423 | www.7749tv.com | 104.24.211.67 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 432 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 506 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 71 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 377 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 200 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 314 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 482 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 590 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 349 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 447 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 355 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 372 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 374 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 518 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 62 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 343 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 236 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 401 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 402 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 616 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 665 | 172.67.73.69 | 172.67.73.69 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 470 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 550 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 13 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 54 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 202 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 334 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 472 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 589 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 613 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 370 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 153 | cloudflare |
| 442 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 153 | cloudflare |
| 508 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 153 | cloudflare |
| 127 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 148 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 219 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 386 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 404 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 77 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 156 | cloudflare |
| 551 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare |
| 604 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare |
| 244 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 545 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 20 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 288 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 307 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 656 | 172.64.53.51 | 172.64.53.51 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 3 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 160 | cloudflare |
| 245 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 160 | cloudflare |
| 315 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 160 | cloudflare |
| 629 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 188 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare |
| 393 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare |
| 117 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 164 | cloudflare |
| 336 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 164 | cloudflare |
| 361 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 474 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 670 | 172.67.69.253 | 172.67.69.253 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 22 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 79 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 179 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 577 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 633 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 59 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 86 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 153 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 281 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 193 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 169 | cloudflare |
| 93 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 280 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 170 | cloudflare |
| 673 | 172.67.75.58 | 172.67.75.58 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 441 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 171 | cloudflare |
| 519 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 171 | cloudflare |
| 14 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 57 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 174 | cloudflare |
| 394 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 175 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 176 | cloudflare |
| 524 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 176 | cloudflare |
| 375 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare |
| 468 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare |
| 485 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare |
| 500 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 177 | cloudflare |
| 371 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 178 | cloudflare |
| 250 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 179 | cloudflare |
| 509 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 179 | cloudflare |
| 581 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 179 | cloudflare |
| 64 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 180 | cloudflare |
| 351 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 181 | cloudflare |
| 659 | 162.159.39.252 | 162.159.39.252 | IPv4 | h3 | ✅ 成功 | 181 | cloudflare |
| 369 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 560 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 82 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 183 | cloudflare |
| 564 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 183 | cloudflare |
| 115 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 184 | cloudflare |
| 496 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 184 | cloudflare |
| 501 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 528 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 390 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare |
| 428 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare |
| 620 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare |
| 559 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare |
| 126 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 190 | cloudflare |
| 206 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 193 | cloudflare |
| 210 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 193 | cloudflare |
| 238 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 193 | cloudflare |
| 526 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 193 | cloudflare |
| 2 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 194 | cloudflare |
| 344 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 196 | cloudflare |
| 446 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 196 | cloudflare |
| 125 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 197 | cloudflare |
| 391 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare |
| 565 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare |
| 162 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 202 | cloudflare |
| 703 | 172.64.53.118 | 172.64.53.118 | IPv4 | h3 | ✅ 成功 | 202 | cloudflare |
| 211 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 206 | cloudflare |
| 312 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 206 | cloudflare |
| 563 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 206 | cloudflare |
| 103 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 208 | cloudflare |
| 161 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 208 | cloudflare |
| 431 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 208 | cloudflare |
| 297 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 209 | cloudflare |
| 337 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 209 | cloudflare |
| 566 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 211 | cloudflare |
| 486 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 212 | cloudflare |
| 687 | 162.159.18.13 | 162.159.18.13 | IPv4 | h3 | ✅ 成功 | 212 | cloudflare |
| 97 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 217 | cloudflare |
| 558 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare |
| 326 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 223 | cloudflare |
| 118 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 230 | cloudflare |
| 513 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 232 | cloudflare |
| 488 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 234 | cloudflare |
| 692 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 252 | cloudflare |
| 132 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 254 | cloudflare |
| 357 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 269 | cloudflare |
| 373 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 269 | cloudflare |
| 502 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 269 | cloudflare |
| 668 | 172.67.67.52 | 172.67.67.52 | IPv4 | h3 | ✅ 成功 | 273 | cloudflare |
| 99 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 280 | cloudflare |
| 517 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 285 | cloudflare |
| 480 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 288 | cloudflare |
| 552 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 293 | cloudflare |
| 481 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 294 | cloudflare |
| 104 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 299 | cloudflare |
| 72 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 301 | cloudflare |
| 381 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 306 | cloudflare |
| 382 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 308 | cloudflare |
| 702 | 162.159.38.59 | 162.159.38.59 | IPv4 | h3 | ✅ 成功 | 310 | cloudflare |
| 704 | 198.41.223.173 | 198.41.223.173 | IPv4 | h3 | ✅ 成功 | 310 | cloudflare |
| 573 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 312 | cloudflare |
| 147 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 317 | cloudflare |
| 152 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 327 | cloudflare |
| 199 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 340 | cloudflare |
| 420 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 345 | cloudflare |
| 669 | 172.67.79.70 | 172.67.79.70 | IPv4 | h3 | ✅ 成功 | 350 | cloudflare |
| 623 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 354 | cloudflare |
| 684 | 172.64.154.63 | 172.64.154.63 | IPv4 | h3 | ✅ 成功 | 358 | cloudflare |
| 268 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 384 | cloudflare |
| 434 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 391 | cloudflare |
| 291 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 542 | cloudflare |
| 81 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 559 | cloudflare |
| 89 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 613 | cloudflare |
| 242 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 763 | cloudflare |
| 422 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 834 | cloudflare |
| 15 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 870 | cloudflare |
| 667 | 172.64.52.17 | 172.64.52.17 | IPv4 | h3 | ✅ 成功 | 903 | cloudflare |
| 530 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 3736 | cloudflare |
| 579 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 5249 | cloudflare |
| 511 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 6168 | cloudflare |
| 678 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 8481 | cloudflare |
| 94 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 9292 | cloudflare |
| 712 | 172.67.72.74 | 172.67.72.74 | IPv4 | h3 | ✅ 成功 | 9944 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 105 条记录
- **正常 (100-200ms)**: 329 条记录
- **慢 (200-500ms)**: 48 条记录
- **很慢 (>500ms)**: 13 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 4 次
- **IPv6 失败**: 206 次

### 按协议统计

- **none**: 215 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
