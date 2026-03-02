# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/3/2 12:40:22
- **数据来源**: connectivity_results-20260302-124022.json
- **总测试数**: 939
- **失败测试数**: 261
- **成功测试数**: 678
- **失败率**: 27.80%
- **平均延迟**: 89.35ms
- **最小延迟**: 61ms
- **最大延迟**: 653ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/3/2 12:40:22
- **IP地址**: 172.215.217.65
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1437, -104.8117
- **时区**: America/Denver
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 257 次 (98.5%)
- **连接超时: I/O超时**: 3 次 (1.1%)
- **DNS解析错误: 其他DNS错误**: 1 次 (0.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (257 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 13 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 14 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 15 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 18 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 19 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 22 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 23 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 26 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 27 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 30 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 31 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 34 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 35 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 38 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 39 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 44 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 45 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 46 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 49 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 50 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 59 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 62 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 63 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 68 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 69 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 70 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 74 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 75 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 81 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 82 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 83 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 84 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 85 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 92 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 93 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 94 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 95 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 125 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 126 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 127 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 133 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 134 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 135 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 144 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 145 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 146 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 150 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 151 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 152 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 156 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 157 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 158 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 165 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 166 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 169 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 170 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 174 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 175 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 176 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 180 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:d9:2acf:b5e0:5a46:4358]:443: connect: network is unreachable |
| 181 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:dd:df95:6eb1:ffa4:6779]:443: connect: network is unreachable |
| 188 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 189 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 192 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 193 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 198 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 199 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 203 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 204 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 205 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 209 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 210 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 211 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 212 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 218 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 219 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 220 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 227 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 228 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 229 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 231 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 238 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 239 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 240 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 244 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 245 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 246 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 251 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 252 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 256 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 257 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 258 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 262 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 263 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 269 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 270 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 271 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 274 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 275 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 279 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 280 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 281 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 285 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 286 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 287 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 291 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 292 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 293 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 297 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 298 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 299 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 309 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 310 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 311 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 315 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 316 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 317 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 319 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 322 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 325 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 326 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 332 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 333 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 334 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 338 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 339 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 340 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 341 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 345 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 346 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 350 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 351 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 352 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 353 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 356 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 357 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 360 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 363 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 364 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 367 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 368 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 378 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 379 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 380 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 384 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 385 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 387 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 391 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 392 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 393 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 399 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 400 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 401 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 406 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 415 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 416 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 417 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 426 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 427 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 428 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 437 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 438 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
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
| 480 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 481 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 482 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 521 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 522 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 523 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 524 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 525 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 526 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 527 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 528 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 529 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 530 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 599 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 600 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 611 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 612 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 613 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 614 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 616 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 617 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 618 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 619 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 620 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 621 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 656 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 665 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 666 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 667 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 668 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 669 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 670 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 671 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 699 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 700 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 701 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 702 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 703 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 704 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 705 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 706 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 707 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 708 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 733 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 734 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 752 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:0:edda:98f0:da65:4271]:443: connect: network is unreachable |
| 753 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:85:ac4c:8137:506:5188]:443: connect: network is unreachable |
| 754 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2]:443: connect: network is unreachable |
| 755 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:fc87:e2d6:88c3:378b]:443: connect: network is unreachable |
| 756 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53]:443: connect: network is unreachable |
| 758 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:6e:e874:db4f:a1d5:2163]:443: connect: network is unreachable |
| 759 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:4f38:5221:f77f:fe11]:443: connect: network is unreachable |
| 760 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::ee:b8fb:877f]:443: connect: network is unreachable |
| 761 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3]:443: connect: network is unreachable |
| 762 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7]:443: connect: network is unreachable |
| 798 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:da84:1c63:f149:4d21:b339]:443: connect: network is unreachable |
| 804 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:a594:2926:2b16:6e8d:843e]:443: connect: network is unreachable |
| 805 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:aa:3e22:dd48:6279:eeb9]:443: connect: network is unreachable |
| 806 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea]:443: connect: network is unreachable |
| 807 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b]:443: connect: network is unreachable |
| 809 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9516:e4a1:4ba9:1c5e:7533]:443: connect: network is unreachable |
| 810 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f702:ebbf:618b:76c:9ba7]:443: connect: network is unreachable |
| 811 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:54:2c60:eafc:f14d:ca4b]:443: connect: network is unreachable |
| 812 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:6a:7ba4:346b:e06c:71c7]:443: connect: network is unreachable |
| 813 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104::f3:8fed:cac0]:443: connect: network is unreachable |
| 872 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:94:16cd:b988:5dae:1295]:443: connect: network is unreachable |
| 873 | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:ec9e:b468:412c:1558:69cb]:443: connect: network is unreachable |
| 874 | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c]:443: connect: network is unreachable |
| 875 | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:94:1604:ebd:f1ec:37be]:443: connect: network is unreachable |
| 876 | 2606:4700:59:764d:d406:c823:e52f:4f3a | 2606:4700:59:764d:d406:c823:e52f:4f3a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:59:764d:d406:c823:e52f:4f3a]:443: connect: network is unreachable |
| 877 | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f]:443: connect: network is unreachable |
| 878 | 2a06:98c1:51:8:7944:48b0:1301:5ced | 2a06:98c1:51:8:7944:48b0:1301:5ced | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8:7944:48b0:1301:5ced]:443: connect: network is unreachable |
| 879 | 2a06:98c1:310b::fda8:fa9e:4a3e | 2a06:98c1:310b::fda8:fa9e:4a3e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b::fda8:fa9e:4a3e]:443: connect: network is unreachable |
| 880 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:cfd2:7ebe:a043:8535]:443: connect: network is unreachable |
| 881 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:e263:6cdc:a8ce:a339]:443: connect: network is unreachable |
| 918 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:5d:a92a:97f:6fa3:f803]:443: connect: network is unreachable |
| 919 | 2a06:98c1:310c::dd:f399:427e | 2a06:98c1:310c::dd:f399:427e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c::dd:f399:427e]:443: connect: network is unreachable |
| 920 | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:e1e7:ae26:af07:44a6:82da]:443: connect: network is unreachable |
| 921 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3]:443: connect: network is unreachable |
| 922 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:b3:af54:9923:e84:af58]:443: connect: network is unreachable |
| 923 | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:22:21cb:7546:1cd8:a79f]:443: connect: network is unreachable |
| 924 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:878:e123:da31:b2ee:2017]:443: connect: network is unreachable |
| 926 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:de:2b25:85a5:8a26]:443: connect: network is unreachable |
| 927 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:22:21ad:d760:d542:16c8]:443: connect: network is unreachable |
| 929 | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:8768:b929:7455:f040:5aee]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 233 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 403 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 939 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 886 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | N/A | 0 | N/A | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": read tcp 10.1.0.96:39244->3.0.50.69:443: read: connection reset by peer |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 257 次 (98.5%)
- **连接超时**: 3 次 (1.1%)
- **DNS解析错误**: 1 次 (0.4%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 260 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 257 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 678 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 123 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 908 | 104.20.18.125 | 104.20.18.125 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 463 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 578 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 736 | 104.25.241.198 | 104.25.241.198 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 763 | 198.41.222.191 | 198.41.222.191 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 842 | 108.162.198.232 | 108.162.198.232 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 857 | 172.67.76.61 | 172.67.76.61 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 932 | 162.159.33.28 | 162.159.33.28 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 248 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 265 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 303 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 535 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 557 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 559 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 572 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 663 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 679 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 721 | 172.64.229.134 | 172.64.229.134 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 741 | 172.64.52.224 | 172.64.52.224 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 796 | 104.25.249.225 | 104.25.249.225 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 864 | 104.25.241.19 | 104.25.241.19 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 884 | 162.159.0.79 | 162.159.0.79 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 910 | 104.25.243.36 | 104.25.243.36 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 925 | 108.162.198.206 | 108.162.198.206 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 173 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 208 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 502 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 545 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 554 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 601 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 635 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 636 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 652 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 677 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 689 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 695 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 742 | 104.17.111.150 | 104.17.111.150 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 793 | 104.25.244.36 | 104.25.244.36 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 797 | 162.159.33.191 | 162.159.33.191 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 808 | 172.64.151.253 | 172.64.151.253 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 841 | 172.64.53.220 | 172.64.53.220 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 867 | 104.25.253.253 | 104.25.253.253 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 887 | 162.159.40.8 | 162.159.40.8 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 916 | 104.25.246.117 | 104.25.246.117 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 148 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 154 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 200 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 372 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 444 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 479 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 510 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 550 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 556 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 580 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 608 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 641 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 693 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 710 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 714 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 751 | 172.64.53.202 | 172.64.53.202 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 784 | 104.26.11.33 | 104.26.11.33 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 791 | 104.25.245.215 | 104.25.245.215 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 821 | 172.67.78.23 | 172.67.78.23 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 854 | 172.67.72.250 | 172.67.72.250 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 882 | 172.64.157.214 | 172.64.157.214 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 893 | 108.162.198.152 | 108.162.198.152 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 905 | 104.26.4.213 | 104.26.4.213 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 914 | 104.25.242.137 | 104.25.242.137 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 915 | 104.25.254.47 | 104.25.254.47 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 930 | 172.64.229.82 | 172.64.229.82 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 164 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 370 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 376 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 398 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 592 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 594 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 683 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 709 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 731 | 104.26.0.210 | 104.26.0.210 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 748 | 162.159.11.128 | 162.159.11.128 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 787 | 104.26.5.194 | 104.26.5.194 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 836 | 104.18.160.38 | 104.18.160.38 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 848 | 104.26.1.55 | 104.26.1.55 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 897 | 104.25.252.192 | 104.25.252.192 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 909 | 104.25.241.235 | 104.25.241.235 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 911 | 104.25.245.173 | 104.25.245.173 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 327 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 441 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 486 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 504 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 506 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 516 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 552 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 582 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 659 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 725 | 104.26.6.247 | 104.26.6.247 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 778 | 172.67.70.56 | 172.67.70.56 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 785 | 104.26.4.135 | 104.26.4.135 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 815 | 162.159.43.50 | 162.159.43.50 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 833 | 104.25.246.24 | 104.25.246.24 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 837 | 104.16.245.121 | 104.16.245.121 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 264 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 377 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 633 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 802 | 172.64.153.183 | 172.64.153.183 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 820 | 172.67.73.196 | 172.67.73.196 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 852 | 172.64.152.85 | 172.64.152.85 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 344 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 454 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 460 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 646 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 413 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 483 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 595 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 904 | 172.67.65.44 | 172.67.65.44 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 65 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 321 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 597 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 661 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 688 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 278 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 491 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 767 | 162.159.1.39 | 162.159.1.39 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 800 | 104.25.250.205 | 104.25.250.205 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 838 | 104.17.62.194 | 104.17.62.194 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 849 | 104.26.14.88 | 104.26.14.88 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 899 | 172.67.73.129 | 172.67.73.129 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 187 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 509 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 769 | 104.20.19.180 | 104.20.19.180 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 779 | 104.20.31.132 | 104.20.31.132 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 503 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 519 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 776 | 162.159.20.46 | 162.159.20.46 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 933 | 172.64.53.103 | 172.64.53.103 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 439 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 853 | 104.20.29.62 | 104.20.29.62 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 407 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 632 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 684 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 389 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 558 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 681 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 405 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 443 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 870 | 162.159.39.156 | 162.159.39.156 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 651 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 720 | 162.159.44.101 | 162.159.44.101 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 77 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 114 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 140 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 448 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 450 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 520 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 537 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 570 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 690 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 745 | 172.64.153.140 | 172.64.153.140 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 892 | 162.159.39.196 | 162.159.39.196 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 110 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 295 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 301 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 419 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 446 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 457 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 478 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 487 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 495 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 563 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 631 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 764 | 162.159.39.180 | 162.159.39.180 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 866 | 104.16.247.125 | 104.16.247.125 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 3 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 6 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 72 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 111 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 115 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 143 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 168 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 343 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 425 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 492 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 494 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 514 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 564 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 584 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 603 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 645 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 654 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 658 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 664 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 696 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 738 | 104.17.97.228 | 104.17.97.228 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 803 | 172.64.144.132 | 172.64.144.132 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 822 | 172.67.79.249 | 172.67.79.249 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 869 | 104.18.44.159 | 104.18.44.159 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 900 | 172.67.77.104 | 172.67.77.104 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 913 | 104.25.250.121 | 104.25.250.121 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 936 | 104.18.40.200 | 104.18.40.200 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 8 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 11 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 33 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 53 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 54 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 64 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 78 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 100 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 118 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 194 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 214 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 289 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 296 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 302 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 320 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 386 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 410 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 412 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 421 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 436 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 488 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 505 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 515 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 607 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 609 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 642 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 675 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 746 | 104.25.245.233 | 104.25.245.233 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 749 | 104.25.250.174 | 104.25.250.174 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 795 | 104.25.247.78 | 104.25.247.78 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 829 | 104.17.56.177 | 104.17.56.177 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 17 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 24 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 32 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 73 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 86 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 106 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 108 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 112 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 138 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 215 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 304 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 314 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 331 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 348 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 349 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 358 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 361 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 369 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 375 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 409 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 418 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 420 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 430 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 431 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 462 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 512 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 542 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 561 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 568 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 571 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 575 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 622 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 660 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 662 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 715 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 743 | 104.18.172.20 | 104.18.172.20 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 747 | 104.18.166.232 | 104.18.166.232 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 757 | 162.159.58.17 | 162.159.58.17 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 792 | 104.18.148.235 | 104.18.148.235 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 819 | 172.64.229.15 | 172.64.229.15 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 828 | 172.67.77.185 | 172.67.77.185 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 858 | 172.67.64.116 | 172.67.64.116 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 917 | 104.18.47.193 | 104.18.47.193 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 928 | 104.25.248.93 | 104.25.248.93 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 935 | 162.159.44.199 | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 4 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 40 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 55 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 76 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 117 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 129 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 155 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 206 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 268 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 273 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 305 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 335 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 365 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 371 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 382 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 404 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 452 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 455 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 456 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 585 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 586 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 587 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 590 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 602 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 628 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 680 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 686 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 718 | 172.64.52.189 | 172.64.52.189 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 862 | 104.25.247.129 | 104.25.247.129 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 863 | 104.17.97.146 | 104.17.97.146 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 71 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 80 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 179 | bestcf.030101.xyz | 104.21.34.29 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 190 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 232 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 237 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 254 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 255 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 276 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 290 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 307 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 328 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 354 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 373 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 394 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 489 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 511 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 513 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 546 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 634 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 640 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 647 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 697 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 783 | 172.67.67.0 | 172.67.67.0 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 856 | 104.26.12.33 | 104.26.12.33 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 871 | 172.64.229.185 | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 888 | 172.64.41.47 | 172.64.41.47 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 12 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 20 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 29 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 36 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 52 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 119 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 120 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 136 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 162 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 224 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 243 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 272 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 435 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 445 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 451 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 547 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 553 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 573 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 596 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 626 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 650 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 655 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 735 | 104.26.8.171 | 104.26.8.171 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 737 | 104.20.20.192 | 104.20.20.192 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 845 | 162.159.39.26 | 162.159.39.26 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 855 | 104.26.6.238 | 104.26.6.238 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 25 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 60 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 113 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 153 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 184 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 223 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 259 | freeyx.cloudflare88.eu.org | 172.64.147.137 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 284 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 312 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 461 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 496 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 639 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 692 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 826 | 172.67.65.81 | 172.67.65.81 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 901 | 172.67.64.123 | 172.67.64.123 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 51 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 221 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 250 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 464 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 507 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 533 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 536 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 562 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 576 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 691 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 786 | 104.20.22.91 | 104.20.22.91 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 860 | 104.25.252.135 | 104.25.252.135 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 906 | 172.67.79.218 | 172.67.79.218 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 938 | 104.26.5.53 | 104.26.5.53 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 185 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 429 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 449 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 543 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 548 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 555 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 567 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 606 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 629 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 657 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 728 | 104.26.14.117 | 104.26.14.117 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 789 | 104.25.240.123 | 104.25.240.123 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 2 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 66 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 107 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 161 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 178 | bestcf.030101.xyz | 104.21.67.166 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 324 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 566 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 625 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 630 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 834 | 104.25.242.249 | 104.25.242.249 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 41 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 122 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 308 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 366 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 423 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 678 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 750 | 162.159.3.128 | 162.159.3.128 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 801 | 104.25.254.14 | 104.25.254.14 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 814 | 172.64.52.181 | 172.64.52.181 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 818 | 162.159.38.83 | 162.159.38.83 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 907 | 172.67.76.20 | 172.67.76.20 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 98 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 101 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 222 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 247 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 381 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 422 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 508 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 531 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 549 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 672 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 740 | 104.25.244.239 | 104.25.244.239 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 898 | 172.64.40.68 | 172.64.40.68 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 912 | 104.25.240.21 | 104.25.240.21 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 9 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 216 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 390 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 539 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 560 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 581 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 598 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 674 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 794 | 104.17.56.208 | 104.17.56.208 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 902 | 104.20.29.234 | 104.20.29.234 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 47 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 48 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 137 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 362 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 383 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 565 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 605 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 623 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 637 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 723 | 162.159.39.146 | 162.159.39.146 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 739 | 104.26.13.110 | 104.26.13.110 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 770 | 172.64.53.41 | 172.64.53.41 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 846 | 162.159.38.52 | 162.159.38.52 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 5 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 97 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 116 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 132 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 149 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 183 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 277 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 329 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 337 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 408 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 442 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 574 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 588 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 638 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 649 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 653 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 673 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 694 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 713 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 724 | 104.18.42.16 | 104.18.42.16 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 730 | 104.20.25.181 | 104.20.25.181 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 765 | 162.159.12.120 | 162.159.12.120 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 771 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 823 | 104.20.24.239 | 104.20.24.239 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 839 | 104.17.143.82 | 104.17.143.82 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 840 | 162.159.44.246 | 162.159.44.246 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 868 | 104.25.255.103 | 104.25.255.103 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 889 | 108.162.198.223 | 108.162.198.223 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 931 | 162.159.49.244 | 162.159.49.244 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 61 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 88 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 89 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 103 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 128 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 300 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 402 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 432 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 490 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 493 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 551 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 589 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 615 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 676 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 698 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 790 | 172.67.72.212 | 172.67.72.212 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 799 | 104.25.241.85 | 104.25.241.85 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 844 | 172.64.52.194 | 172.64.52.194 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 851 | 162.159.21.16 | 162.159.21.16 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 865 | 104.17.129.66 | 104.17.129.66 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 885 | 172.64.53.15 | 172.64.53.15 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 890 | 172.64.52.150 | 172.64.52.150 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 937 | 172.64.146.121 | 172.64.146.121 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 21 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 96 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 99 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 102 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 142 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 167 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 195 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 197 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 234 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 283 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 318 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 396 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 440 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 458 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 484 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 540 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 577 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 604 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 610 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 712 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 719 | 172.64.53.165 | 172.64.53.165 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 830 | 104.25.244.87 | 104.25.244.87 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 832 | 104.25.254.89 | 104.25.254.89 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 847 | 172.64.229.149 | 172.64.229.149 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 859 | 172.67.68.110 | 172.67.68.110 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 16 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 56 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 67 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 124 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 147 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 191 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 196 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 242 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 313 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 424 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 434 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 459 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 485 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 500 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 517 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 579 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 716 | 108.162.198.85 | 108.162.198.85 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 727 | 104.26.1.88 | 104.26.1.88 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 777 | 108.162.198.198 | 108.162.198.198 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 780 | 172.64.229.156 | 172.64.229.156 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 827 | 104.20.22.141 | 104.20.22.141 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 883 | 104.17.171.88 | 104.17.171.88 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 903 | 104.20.20.156 | 104.20.20.156 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 87 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 91 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 163 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 241 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 306 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 374 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 499 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 593 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 711 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 726 | 162.159.36.205 | 162.159.36.205 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 729 | 104.20.28.239 | 104.20.28.239 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 788 | 104.20.24.244 | 104.20.24.244 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 825 | 104.20.26.221 | 104.20.26.221 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 831 | 104.26.3.117 | 104.26.3.117 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 891 | 172.64.53.195 | 172.64.53.195 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 7 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 28 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 43 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 121 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 130 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 230 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 253 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 323 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 388 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 433 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 447 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 453 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 541 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 569 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 624 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 643 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 648 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 687 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 768 | 108.162.198.70 | 108.162.198.70 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 824 | 172.67.75.212 | 172.67.75.212 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 835 | 104.25.240.227 | 104.25.240.227 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 37 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 249 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 282 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 501 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 583 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 772 | 162.159.38.45 | 162.159.38.45 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 58 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 79 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 109 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 139 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 186 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 201 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 217 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 236 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 261 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 359 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 773 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 172 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 411 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 475 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 476 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 685 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 861 | 172.64.146.137 | 172.64.146.137 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 934 | 162.159.27.183 | 162.159.27.183 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 207 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 266 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 355 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 896 | 172.64.229.106 | 172.64.229.106 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 90 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 141 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 182 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 288 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 342 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 414 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 532 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 226 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 591 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 260 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 497 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 518 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 544 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 717 | 162.159.38.226 | 162.159.38.226 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 850 | 162.159.6.186 | 162.159.6.186 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 498 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 894 | 162.159.45.237 | 162.159.45.237 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 225 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 294 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 782 | 162.159.0.41 | 162.159.0.41 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 213 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 267 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 538 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 57 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 774 | 162.159.39.20 | 162.159.39.20 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 534 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 766 | 162.159.38.134 | 162.159.38.134 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 843 | 162.159.45.121 | 162.159.45.121 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 336 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 781 | 162.159.44.202 | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 347 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 397 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 682 | 104.17.193.113 | 104.17.193.113 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 816 | 162.159.39.74 | 162.159.39.74 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 330 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 171 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 477 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 817 | 162.159.45.150 | 162.159.45.150 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 202 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 627 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 775 | 162.159.45.67 | 162.159.45.67 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 895 | 162.159.44.60 | 162.159.44.60 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 744 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 722 | 162.159.45.65 | 162.159.45.65 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 732 | 172.67.65.150 | 172.67.65.150 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 42 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 644 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 235 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 160 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 131 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 10 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 186 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 233 | cloudflare |
| 395 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 294 | cloudflare |
| 159 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 491 | cloudflare |
| 177 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 653 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 559 条记录
- **正常 (100-200ms)**: 115 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 1 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 4 次
- **IPv6 失败**: 257 次

### 按协议统计

- **none**: 260 次失败
- **h2**: 1 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
