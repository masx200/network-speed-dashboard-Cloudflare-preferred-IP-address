# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/23 02:37:50
- **数据来源**: connectivity_results-20260123-023749.json
- **总测试数**: 781
- **失败测试数**: 231
- **成功测试数**: 550
- **失败率**: 29.58%
- **平均延迟**: 97.68ms
- **最小延迟**: 41ms
- **最大延迟**: 505ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/23 02:37:50
- **IP地址**: 172.183.132.68
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
| 1 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:335e:fabe:8b11:3107:3685]:443: connect: network is unreachable |
| 7 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 8 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 11 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 12 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 15 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 16 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 19 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 20 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 23 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 24 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 27 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 28 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 33 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 34 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 44 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 45 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 46 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 47 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 55 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 56 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 57 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 58 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 59 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 63 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 64 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 68 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 69 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 70 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 82 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 86 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 87 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 88 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 91 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 92 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 96 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 97 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 98 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 117 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 118 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 119 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 125 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 126 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 131 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4f66:caf5]:443: connect: network is unreachable |
| 135 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 136 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 137 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 144 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 145 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 146 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 150 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 151 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 152 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 155 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 156 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 160 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 161 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 162 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 166 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 167 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 168 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 172 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 173 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 174 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 179 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 180 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 181 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 211 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 212 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 213 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 216 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 217 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 220 | bestcf.030101.xyz | 2606:4700:0:67b9:b065:a90a:a0a2:2889 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:67b9:b065:a90a:a0a2:2889]:443: connect: network is unreachable |
| 221 | bestcf.030101.xyz | 2606:4700:0:6704:6c5a:ab4f:1bd5:b002 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:6704:6c5a:ab4f:1bd5:b002]:443: connect: network is unreachable |
| 225 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 226 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 227 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 230 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 231 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 236 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 239 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 240 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 243 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 244 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 248 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 249 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 250 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 254 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 255 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 256 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 260 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 261 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 262 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 266 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 267 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 268 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 274 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 275 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 276 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 280 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 281 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 282 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 290 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 291 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 297 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 298 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 302 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 303 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 304 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 308 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 309 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 310 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 314 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 315 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 317 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 321 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 322 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 323 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 327 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 328 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 329 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 330 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 334 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 335 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 336 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 342 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 343 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 345 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 351 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 352 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 353 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 354 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 358 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 359 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 360 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 363 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 364 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 367 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 368 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 373 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 374 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 381 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 382 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 383 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 385 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 391 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 392 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 393 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 394 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 404 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 405 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 406 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 408 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 412 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 413 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 416 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 417 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 421 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 422 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 423 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 436 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 437 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 438 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 475 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 476 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 477 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 479 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 480 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 481 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 482 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 483 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 484 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 485 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 486 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 487 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 488 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 492 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 493 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 494 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 534 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 535 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 536 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 537 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 538 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 539 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 540 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 541 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 542 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 543 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 603 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 604 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 605 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 606 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 607 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 608 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 609 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 610 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 611 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 612 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 663 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 664 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 665 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 666 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 667 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 668 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 669 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 670 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 671 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 672 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 712 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 713 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 714 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 715 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 716 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 717 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 718 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 719 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 720 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 721 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 752 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:1f7b:99bd:a6f2:b984]:443: connect: network is unreachable |
| 761 | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d]:443: connect: network is unreachable |
| 762 | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:37c9:e79d:f57f:5979:246b]:443: connect: network is unreachable |
| 763 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837]:443: connect: network is unreachable |
| 764 | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:0:9437:4b55:9f2c:c3a]:443: connect: network is unreachable |
| 765 | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:c7:ea87:3a0c:9a73:a94a]:443: connect: network is unreachable |
| 766 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:0:b1cc:b849:730:3d20]:443: connect: network is unreachable |
| 767 | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b]:443: connect: network is unreachable |
| 768 | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:0:9fb9:570f:6b2a:983f]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 777 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 780 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 781 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 228 次 (98.7%)
- **连接超时**: 3 次 (1.3%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 231 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 228 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), iplocation.io (3次), huxley.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 550 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 559 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 699 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 594 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 680 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 143 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 520 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 618 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 710 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 731 | 172.64.229.233 | 172.64.229.233 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 279 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 464 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 273 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 320 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 695 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 110 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 570 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 582 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 631 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 410 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 512 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 587 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 696 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 735 | 104.26.10.159 | 104.26.10.159 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 754 | 104.17.57.66 | 104.17.57.66 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 411 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 449 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 450 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 505 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 556 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 566 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 778 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 402 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 630 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 366 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 703 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 390 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 401 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 585 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 602 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 474 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 515 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 626 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 632 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 649 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 400 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 456 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 457 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 510 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 658 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 700 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 504 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 159 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 517 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 625 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 685 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 460 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 741 | 104.26.15.207 | 104.26.15.207 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 641 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 747 | 104.17.181.90 | 104.17.181.90 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 749 | 104.18.92.250 | 104.18.92.250 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 639 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 338 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 478 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 531 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 653 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 215 | xn--b6gac.eu.org | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 362 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 427 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 591 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 629 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 251 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 589 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 288 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 654 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 65 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 575 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 622 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 687 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 283 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 458 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 580 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 89 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 433 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 497 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 548 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 593 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 600 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 770 | 172.64.52.204 | 172.64.52.204 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 228 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 514 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 107 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 233 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 446 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 455 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 616 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 643 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 692 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 10 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 30 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 36 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 85 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 214 | xn--b6gac.eu.org | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 428 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 701 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 723 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 83 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 293 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 471 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 553 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 635 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 756 | 172.64.229.3 | 172.64.229.3 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 294 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 432 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 451 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 522 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 549 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 646 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 48 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 84 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 115 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 299 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 340 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 378 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 398 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 409 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 650 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 651 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 681 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 776 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 29 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 37 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 116 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 175 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 195 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 196 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 208 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 296 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 361 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 652 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 683 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 758 | 172.64.53.110 | 172.64.53.110 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 18 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 132 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 138 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 202 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 415 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 561 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 690 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 104 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 154 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 431 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 507 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 521 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 586 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 634 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 682 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 61 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 129 | freeyx.cloudflare88.eu.org | 141.101.121.145 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 198 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 209 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 440 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 645 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 686 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 724 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 32 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 388 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 532 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 43 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 101 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 142 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 307 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 407 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 414 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 550 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 633 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 661 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 38 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 171 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 355 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 424 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 498 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 576 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 677 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 25 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 49 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 76 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 94 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 102 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 191 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 272 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 316 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 348 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 370 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 379 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 403 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 462 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 533 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 557 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 569 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 675 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 35 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 188 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 190 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 332 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 339 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 554 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 740 | 104.26.3.200 | 104.26.3.200 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 177 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 218 | bestcf.030101.xyz | 104.18.40.176 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 234 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 264 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 286 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 399 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 429 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 627 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 655 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 232 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 247 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 301 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 371 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 524 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 560 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 583 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 684 | 172.64.52.80 | 172.64.52.80 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 184 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 772 | 198.41.223.250 | 198.41.223.250 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 31 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 67 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 130 | freeyx.cloudflare88.eu.org | 141.101.120.120 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 141 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 148 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 518 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 733 | 162.159.40.162 | 162.159.40.162 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 751 | 104.17.58.98 | 104.17.58.98 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 760 | 162.159.38.113 | 162.159.38.113 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 109 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 153 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 157 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 242 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 246 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 467 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 525 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 563 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 567 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 599 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 689 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 6 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 21 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 165 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 472 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 624 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 628 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 660 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 694 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 726 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 743 | 104.26.8.60 | 104.26.8.60 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 750 | 104.17.177.91 | 104.17.177.91 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 779 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 259 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 289 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 454 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 506 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 509 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 577 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 659 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 22 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 113 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 163 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 186 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 193 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 357 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 380 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 387 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 528 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 545 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 106 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 444 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 448 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 601 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 742 | 172.67.71.177 | 172.67.71.177 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 746 | 104.16.155.40 | 104.16.155.40 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 5 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 185 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 287 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 295 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 344 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 459 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 495 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 503 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 578 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 598 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 707 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 737 | 104.20.27.188 | 104.20.27.188 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 775 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 111 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 178 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 389 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 465 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 60 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 103 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 375 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 376 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 511 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 558 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 588 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 595 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 621 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 642 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 727 | 162.159.44.17 | 162.159.44.17 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 9 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 13 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 41 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 72 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 122 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 183 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 346 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 461 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 513 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 529 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 744 | 104.26.13.215 | 104.26.13.215 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 39 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 50 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 430 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 500 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 572 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 638 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 748 | 104.18.94.137 | 104.18.94.137 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 17 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 99 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 189 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 235 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 237 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 270 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 319 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 331 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 499 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 523 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 648 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 657 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 757 | 104.17.221.233 | 104.17.221.233 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 771 | 162.159.44.249 | 162.159.44.249 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 90 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 192 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 350 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 544 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 688 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 697 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 341 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 442 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 468 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 489 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 555 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 581 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 584 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 709 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 745 | 172.67.73.198 | 172.67.73.198 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 425 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 443 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 673 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 736 | 104.20.23.55 | 104.20.23.55 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 80 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 245 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 434 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 441 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 508 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 568 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 615 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 617 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 123 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 613 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 636 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 706 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 732 | 162.159.38.179 | 162.159.38.179 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 759 | 162.159.48.112 | 162.159.48.112 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 372 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 676 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 105 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 182 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 265 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 312 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 313 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 324 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 447 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 552 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 564 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 565 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 620 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 729 | 162.159.45.138 | 162.159.45.138 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 730 | 162.159.39.95 | 162.159.39.95 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 26 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 54 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 241 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 722 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 187 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 219 | bestcf.030101.xyz | 104.17.182.228 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 229 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 269 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 386 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 501 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 551 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 75 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 120 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 128 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 278 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 325 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 349 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 397 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 519 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 40 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 42 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 62 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 253 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 292 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 491 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 562 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 592 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 623 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 705 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 77 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 571 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 574 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 640 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 644 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 108 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 395 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 452 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 463 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 490 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 656 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 356 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 502 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 526 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 708 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 78 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 79 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 112 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 284 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 426 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 678 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 169 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 200 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 369 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 384 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 527 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 579 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 637 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 725 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 197 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 238 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 306 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 201 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 300 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 547 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 377 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 439 | www.7749tv.com | 104.25.196.30 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 453 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 647 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 728 | 108.162.198.183 | 108.162.198.183 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 738 | 104.26.5.25 | 104.26.5.25 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 74 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 271 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 679 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 711 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 139 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 337 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 466 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 14 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 418 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 419 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 470 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 619 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 546 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 769 | 108.162.198.146 | 108.162.198.146 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 147 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 53 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 66 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 223 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 691 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 445 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 596 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 473 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 516 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 590 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 698 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 52 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 252 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 662 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 124 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 224 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 95 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 133 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 396 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 140 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 318 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 435 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 2 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 3 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 93 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 263 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 164 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 222 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 530 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 704 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 81 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 158 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 753 | 172.64.158.41 | 172.64.158.41 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 170 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 702 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 71 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 73 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 4 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 194 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 420 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 149 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 305 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 693 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 114 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 573 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 210 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 176 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 496 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 734 | 162.159.39.194 | 162.159.39.194 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 739 | 162.159.36.94 | 162.159.36.94 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 127 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 674 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 365 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 326 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 156 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare |
| 204 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 277 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 258 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 257 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 333 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 169 | cloudflare |
| 199 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 469 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 171 | cloudflare |
| 755 | 104.19.38.110 | 104.19.38.110 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 773 | 162.159.45.140 | 162.159.45.140 | IPv4 | h3 | ✅ 成功 | 176 | cloudflare |
| 614 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 188 | cloudflare |
| 51 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 189 | cloudflare |
| 100 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 199 | cloudflare |
| 285 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 199 | cloudflare |
| 121 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 214 | cloudflare |
| 134 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 240 | cloudflare |
| 597 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 371 | cloudflare |
| 311 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 478 | cloudflare |
| 774 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 483 | cloudflare |
| 347 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 505 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 24 条记录
- **快 (50-100ms)**: 306 条记录
- **正常 (100-200ms)**: 214 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 1 条记录


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
