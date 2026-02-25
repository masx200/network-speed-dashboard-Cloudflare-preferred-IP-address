# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 04:39:02
- **数据来源**: connectivity_results-20260225-043901.json
- **总测试数**: 835
- **失败测试数**: 243
- **成功测试数**: 592
- **失败率**: 29.10%
- **平均延迟**: 76.83ms
- **最小延迟**: 48ms
- **最大延迟**: 546ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 04:39:02
- **IP地址**: 172.182.226.227
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4532, -112.0748
- **时区**: America/Phoenix
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 239 次 (98.4%)
- **连接超时: I/O超时**: 4 次 (1.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (239 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 9 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 10 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 11 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 15 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 16 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 17 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 20 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 21 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 24 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 25 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 28 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 60 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 61 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 62 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 67 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 68 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 69 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 77 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 78 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 84 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 85 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 86 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 87 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 88 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 94 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 95 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 99 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 100 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 101 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 108 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 109 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 110 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 114 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 115 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 116 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 120 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cdf7:f50d:f763 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:cdf7:f50d:f763]:443: connect: network is unreachable |
| 121 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:51:d1e4:65f0:5ed1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:51:d1e4:65f0:5ed1]:443: connect: network is unreachable |
| 125 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 126 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 127 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 131 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 132 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 133 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 138 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 139 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 140 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 144 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 145 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 146 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 149 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:dd:df95:6eb1:ffa4:6779]:443: connect: network is unreachable |
| 150 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:d9:2acf:b5e0:5a46:4358]:443: connect: network is unreachable |
| 155 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 160 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 161 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 162 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 165 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 166 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 169 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 170 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 173 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 174 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 175 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 178 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 179 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 186 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 187 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 188 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 191 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 192 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 196 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 197 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 198 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 201 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 202 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 206 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 207 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 208 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 212 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 213 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 214 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 222 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 223 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 224 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 227 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 228 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 234 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 235 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 236 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 241 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 242 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 243 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 247 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 248 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 249 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 254 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 255 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 259 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 260 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 261 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 264 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 265 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 269 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 270 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 271 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 272 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 276 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 277 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 278 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 283 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 284 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 285 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 287 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 289 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 295 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 296 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 297 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 301 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 302 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 303 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 306 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 309 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 310 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 313 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 314 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 319 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 320 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 327 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 328 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 329 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 330 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 334 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 335 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 337 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 339 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 343 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 344 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 349 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 350 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 351 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 355 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 356 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 357 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 362 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 363 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 364 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 367 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 368 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 377 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 378 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 379 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 420 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 421 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 422 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 423 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 424 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 425 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 426 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 427 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 428 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 429 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 470 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 471 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 472 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 473 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 474 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 475 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 476 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 477 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 478 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 479 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 530 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 538 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 539 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 542 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 543 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 549 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 550 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 553 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 554 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 560 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 561 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 573 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 574 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 575 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 581 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 582 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 583 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 592 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 593 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 594 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 595 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 596 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 597 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 598 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 599 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 600 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 601 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 652 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 653 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 654 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 655 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 656 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 657 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 658 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 659 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 660 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 661 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 699 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 700 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 702 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 705 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 706 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 707 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 708 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 709 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 711 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 712 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 713 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 750 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53]:443: connect: network is unreachable |
| 754 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:0:edda:98f0:da65:4271]:443: connect: network is unreachable |
| 755 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:85:ac4c:8137:506:5188]:443: connect: network is unreachable |
| 756 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2]:443: connect: network is unreachable |
| 757 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:fc87:e2d6:88c3:378b]:443: connect: network is unreachable |
| 758 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3]:443: connect: network is unreachable |
| 759 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:6e:e874:db4f:a1d5:2163]:443: connect: network is unreachable |
| 760 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:4f38:5221:f77f:fe11]:443: connect: network is unreachable |
| 761 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::ee:b8fb:877f]:443: connect: network is unreachable |
| 763 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7]:443: connect: network is unreachable |
| 800 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104::f3:8fed:cac0]:443: connect: network is unreachable |
| 804 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:a594:2926:2b16:6e8d:843e]:443: connect: network is unreachable |
| 805 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:aa:3e22:dd48:6279:eeb9]:443: connect: network is unreachable |
| 806 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea]:443: connect: network is unreachable |
| 807 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b]:443: connect: network is unreachable |
| 808 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:da84:1c63:f149:4d21:b339]:443: connect: network is unreachable |
| 809 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9516:e4a1:4ba9:1c5e:7533]:443: connect: network is unreachable |
| 810 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f702:ebbf:618b:76c:9ba7]:443: connect: network is unreachable |
| 811 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:54:2c60:eafc:f14d:ca4b]:443: connect: network is unreachable |
| 812 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:6a:7ba4:346b:e06c:71c7]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 622 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 820 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 828 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | none | N/A | 0 | N/A | dial tcp 104.20.255.53:443: i/o timeout |
| 832 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 239 次 (98.4%)
- **连接超时**: 4 次 (1.6%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 243 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 239 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 592 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 780 | 104.26.11.33 | 104.26.11.33 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 203 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 665 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 685 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 727 | 104.20.28.239 | 104.20.28.239 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 194 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 318 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 546 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 670 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 710 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 787 | 104.26.5.194 | 104.26.5.194 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 831 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 164 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 231 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 614 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 631 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 679 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 716 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 199 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 358 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 442 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 511 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 576 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 587 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 610 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 678 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 745 | 172.64.153.140 | 172.64.153.140 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 797 | 172.64.153.183 | 172.64.153.183 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 801 | 172.64.144.132 | 172.64.144.132 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 66 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 292 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 565 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 674 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 690 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 723 | 162.159.44.101 | 162.159.44.101 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 768 | 172.64.53.41 | 172.64.53.41 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 814 | 172.64.229.15 | 172.64.229.15 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 19 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 348 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 382 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 394 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 486 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 505 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 740 | 104.25.241.198 | 104.25.241.198 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 746 | 104.25.250.174 | 104.25.250.174 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 129 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 226 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 275 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 305 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 324 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 338 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 444 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 446 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 591 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 602 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 627 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 753 | 172.64.53.202 | 172.64.53.202 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 128 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 168 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 256 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 280 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 294 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 541 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 572 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 589 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 605 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 668 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 673 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 718 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 784 | 104.20.22.91 | 104.20.22.91 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 785 | 172.67.72.212 | 172.67.72.212 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 279 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 400 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 417 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 435 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 483 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 540 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 568 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 624 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 751 | 162.159.3.128 | 162.159.3.128 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 72 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 190 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 233 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 244 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 353 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 372 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 386 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 490 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 512 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 569 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 577 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 579 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 645 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 651 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 664 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 731 | 104.20.25.181 | 104.20.25.181 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 375 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 445 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 480 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 491 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 563 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 667 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 677 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 680 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 686 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 695 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 725 | 104.18.42.16 | 104.18.42.16 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 744 | 104.17.111.150 | 104.17.111.150 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 775 | 162.159.44.202 | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 793 | 104.25.242.249 | 104.25.242.249 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 830 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 36 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 75 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 171 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 200 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 258 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 317 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 432 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 433 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 437 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 507 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 518 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 570 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 571 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 623 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 632 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 687 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 721 | 172.64.53.165 | 172.64.53.165 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 730 | 104.26.0.210 | 104.26.0.210 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 774 | 108.162.198.198 | 108.162.198.198 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 777 | 172.64.229.156 | 172.64.229.156 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 781 | 172.67.67.0 | 172.67.67.0 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 783 | 104.26.4.135 | 104.26.4.135 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 55 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 180 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 238 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 246 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 288 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 290 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 321 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 342 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 405 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 413 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 431 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 487 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 515 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 544 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 619 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 642 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 736 | 104.26.13.110 | 104.26.13.110 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 741 | 104.25.244.239 | 104.25.244.239 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 778 | 162.159.0.41 | 162.159.0.41 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 8 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 58 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 63 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 80 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 83 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 105 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 156 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 195 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 331 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 336 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 393 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 522 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 534 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 694 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 698 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 762 | 198.41.222.191 | 198.41.222.191 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 770 | 162.159.39.20 | 162.159.39.20 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 772 | 172.64.52.224 | 172.64.52.224 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 6 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 151 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 177 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 181 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 237 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 281 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 300 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 345 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 373 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 458 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 466 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 467 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 580 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 608 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 634 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 636 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 647 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 684 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 719 | 108.162.198.85 | 108.162.198.85 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 720 | 162.159.45.65 | 162.159.45.65 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 822 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 51 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 52 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 57 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 153 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 189 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 215 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 274 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 346 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 359 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 409 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 411 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 453 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 500 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 552 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 604 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 606 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 633 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 717 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 733 | 172.67.65.150 | 172.67.65.150 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 835 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 47 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 53 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 71 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 204 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 220 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 307 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 315 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 403 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 449 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 465 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 495 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 499 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 517 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 523 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 525 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 533 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 635 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 688 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 734 | 104.26.8.171 | 104.26.8.171 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 771 | 104.20.31.132 | 104.20.31.132 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 813 | 162.159.43.50 | 162.159.43.50 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 73 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 74 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 172 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 205 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 239 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 240 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 250 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 282 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 340 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 380 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 398 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 402 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 404 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 406 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 448 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 450 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 503 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 514 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 519 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 524 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 531 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 536 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 557 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 559 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 662 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 671 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 722 | 162.159.38.226 | 162.159.38.226 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 752 | 162.159.11.128 | 162.159.11.128 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 33 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 38 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 40 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 46 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 48 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 54 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 65 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 79 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 137 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 152 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 185 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 193 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 251 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 434 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 508 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 646 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 649 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 682 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 715 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 748 | 162.159.38.134 | 162.159.38.134 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 791 | 104.17.56.177 | 104.17.56.177 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 802 | 162.159.39.74 | 162.159.39.74 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 827 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 12 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 27 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 31 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 32 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 49 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 118 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 135 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 142 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 154 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 163 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 225 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 399 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 461 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 464 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 537 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 629 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 696 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 742 | 104.17.193.113 | 104.17.193.113 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 34 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 37 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 56 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 70 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 157 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 229 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 268 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 293 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 312 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 365 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 408 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 410 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 452 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 459 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 460 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 502 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 513 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 532 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 545 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 615 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 620 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 663 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 724 | 104.26.6.247 | 104.26.6.247 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 825 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 159 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 210 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 211 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 387 | www.7749tv.com | 104.19.13.63 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 395 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 412 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 454 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 455 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 457 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 481 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 617 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 644 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 714 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 738 | 104.17.97.228 | 104.17.97.228 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 782 | 172.67.70.56 | 172.67.70.56 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 798 | 104.17.62.194 | 104.17.62.194 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 826 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 7 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 14 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 44 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 59 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 104 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 232 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 347 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 361 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 419 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 451 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 484 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 586 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 675 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 821 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 824 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 42 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 230 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 366 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 414 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 528 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 666 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 726 | 172.64.229.134 | 172.64.229.134 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 26 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 148 | bestcf.030101.xyz | 104.19.153.222 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 333 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 488 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 494 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 504 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 556 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 639 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 669 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 799 | 104.18.160.38 | 104.18.160.38 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 833 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 106 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 147 | bestcf.030101.xyz | 104.17.222.192 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 298 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 316 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 396 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 501 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 641 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 689 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 5 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 45 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 304 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 360 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 381 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 469 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 498 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 506 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 607 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 625 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 693 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 764 | 162.159.12.120 | 162.159.12.120 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 96 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 107 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 122 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 136 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 245 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 388 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 415 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 447 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 497 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 510 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 555 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 585 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 735 | 104.20.20.192 | 104.20.20.192 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 743 | 104.18.172.20 | 104.18.172.20 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 818 | 162.159.38.83 | 162.159.38.83 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 89 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 112 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 117 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 167 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 184 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 221 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 253 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 332 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 370 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 436 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 564 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 566 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 643 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 648 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 676 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 691 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 4 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 43 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 91 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 216 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 383 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 384 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 385 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 463 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 628 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 701 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 41 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 308 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 397 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 492 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 590 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 630 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 767 | 108.162.198.70 | 108.162.198.70 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 29 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 119 | freeyx.cloudflare88.eu.org | 141.101.121.110 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 369 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 391 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 456 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 496 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 613 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 618 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 779 | 162.159.20.46 | 162.159.20.46 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 141 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 374 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 551 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 562 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 683 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 749 | 104.25.245.233 | 104.25.245.233 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 788 | 104.20.19.180 | 104.20.19.180 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 217 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 322 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 390 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 468 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 482 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 526 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 672 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 18 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 39 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 158 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 493 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 739 | 104.18.166.232 | 104.18.166.232 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 22 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 266 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 401 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 489 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 609 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 311 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 441 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 732 | 104.26.14.117 | 104.26.14.117 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 796 | 162.159.33.191 | 162.159.33.191 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 50 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 102 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 123 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 637 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 681 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 440 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 443 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 509 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 829 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 291 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 815 | 172.64.52.181 | 172.64.52.181 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 521 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 703 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 776 | 162.159.45.67 | 162.159.45.67 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 64 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 389 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 558 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 13 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 35 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 143 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 438 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 766 | 162.159.39.180 | 162.159.39.180 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 92 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 176 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 113 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 325 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 567 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 817 | 172.64.151.253 | 172.64.151.253 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 30 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 90 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 209 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 535 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 548 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 729 | 104.26.1.88 | 104.26.1.88 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 267 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 616 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 692 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 2 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 218 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 257 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 263 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 352 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 794 | 104.25.240.227 | 104.25.240.227 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 323 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 439 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 485 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 516 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 547 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 262 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 273 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 376 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 462 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 790 | 104.25.254.89 | 104.25.254.89 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 130 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 392 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 789 | 104.17.143.82 | 104.17.143.82 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 584 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 728 | 162.159.39.146 | 162.159.39.146 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 103 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 124 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 326 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 578 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 773 | 162.159.38.45 | 162.159.38.45 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 816 | 162.159.45.150 | 162.159.45.150 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 299 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 341 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 520 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 611 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 823 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 97 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 626 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 3 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 23 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 252 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 792 | 104.25.244.87 | 104.25.244.87 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 286 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 529 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 588 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 603 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 621 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 697 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 416 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 527 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 638 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 704 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 769 | 162.159.1.39 | 162.159.1.39 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 650 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 803 | 104.16.245.121 | 104.16.245.121 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 182 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 418 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 76 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 93 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 183 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 371 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 430 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 98 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 795 | 104.25.246.24 | 104.25.246.24 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 765 | 162.159.58.17 | 162.159.58.17 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 612 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 786 | 104.20.24.244 | 104.20.24.244 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 834 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 111 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 81 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 354 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 82 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 160 | cloudflare |
| 737 | 172.64.52.189 | 172.64.52.189 | IPv4 | h3 | ✅ 成功 | 251 | cloudflare |
| 407 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 281 | cloudflare |
| 219 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 317 | cloudflare |
| 747 | 162.159.36.205 | 162.159.36.205 | IPv4 | h3 | ✅ 成功 | 319 | cloudflare |
| 134 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 409 | cloudflare |
| 819 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 478 | cloudflare |
| 640 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 546 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 532 条记录
- **正常 (100-200ms)**: 52 条记录
- **慢 (200-500ms)**: 6 条记录
- **很慢 (>500ms)**: 1 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 4 次
- **IPv6 失败**: 239 次

### 按协议统计

- **none**: 243 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
