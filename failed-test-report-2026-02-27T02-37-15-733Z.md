# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/27 02:37:15
- **数据来源**: connectivity_results-20260227-023715.json
- **总测试数**: 1004
- **失败测试数**: 272
- **成功测试数**: 732
- **失败率**: 27.09%
- **平均延迟**: 61.45ms
- **最小延迟**: 22ms
- **最大延迟**: 1108ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/27 02:37:16
- **IP地址**: 172.184.220.151
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 37.3388, -121.8916
- **时区**: America/Los_Angeles
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 268 次 (98.5%)
- **连接超时: I/O超时**: 4 次 (1.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (268 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 7 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 8 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 9 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 12 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 13 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 16 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 17 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 18 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 29 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 30 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 31 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 36 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 37 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 38 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 44 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 45 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 46 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 47 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 48 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 52 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 53 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 54 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 62 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 63 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 67 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 68 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 69 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 100 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 101 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 105 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 106 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 107 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 114 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 115 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 116 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 120 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 121 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 122 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 126 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 127 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 128 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 133 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 134 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 138 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:a7e7:26a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:f5e8:7af2:a7e7:26a1]:443: connect: network is unreachable |
| 142 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 143 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 144 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 145 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 149 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 150 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 151 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 154 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 155 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 158 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 159 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 163 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 164 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 165 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 169 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 170 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 171 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 172 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 178 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 179 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 186 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 187 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 188 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 191 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 192 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 195 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:d9:2acf:b5e0:5a46:4358]:443: connect: network is unreachable |
| 196 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:dd:df95:6eb1:ffa4:6779]:443: connect: network is unreachable |
| 200 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 201 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 202 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 209 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 210 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 211 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 214 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 215 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 219 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 220 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 221 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 230 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 231 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 232 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 236 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 237 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 238 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 241 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 242 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 246 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 247 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 248 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 252 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 253 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 254 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 257 | ip.gs | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 258 | ip.gs | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 261 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 262 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 266 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 267 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 268 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 272 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 273 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 274 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 275 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 279 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 282 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 285 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 286 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 290 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 291 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 292 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 295 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 296 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 297 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 300 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 304 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 305 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 306 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 311 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 312 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 320 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 321 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 322 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 323 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 326 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 327 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 330 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 336 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 337 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 342 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 346 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 347 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 348 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 352 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 353 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 354 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 358 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 359 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 360 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 366 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 367 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 374 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 375 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 396 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 397 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 398 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 417 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 418 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 419 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 420 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 421 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 422 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 423 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 424 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 425 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 426 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 467 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 468 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 469 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 470 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 471 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 472 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 473 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 474 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 475 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 476 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 530 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 538 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 539 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 541 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 542 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 543 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 544 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 545 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 555 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 556 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 557 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 559 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 562 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 564 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 565 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 592 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 593 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 607 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 608 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 611 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 612 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 617 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 618 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 623 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 624 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 625 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 628 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 629 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 636 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 637 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 644 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 645 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 646 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 649 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 667 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 668 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 724 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 726 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 727 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 728 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 729 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 730 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 731 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 732 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 733 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 734 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 735 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 736 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 738 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 739 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:85:ac4c:8137:506:5188]:443: connect: network is unreachable |
| 762 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:0:edda:98f0:da65:4271]:443: connect: network is unreachable |
| 763 | 2a06:98c1:51:db13:f0d1:44cc:d4a3:582c | 2a06:98c1:51:db13:f0d1:44cc:d4a3:582c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:db13:f0d1:44cc:d4a3:582c]:443: connect: network is unreachable |
| 822 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:94:16cd:b988:5dae:1295]:443: connect: network is unreachable |
| 823 | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:ec9e:b468:412c:1558:69cb]:443: connect: network is unreachable |
| 824 | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c]:443: connect: network is unreachable |
| 825 | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:94:1604:ebd:f1ec:37be]:443: connect: network is unreachable |
| 826 | 2606:4700:59:764d:d406:c823:e52f:4f3a | 2606:4700:59:764d:d406:c823:e52f:4f3a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:59:764d:d406:c823:e52f:4f3a]:443: connect: network is unreachable |
| 827 | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f]:443: connect: network is unreachable |
| 828 | 2a06:98c1:51:8:7944:48b0:1301:5ced | 2a06:98c1:51:8:7944:48b0:1301:5ced | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8:7944:48b0:1301:5ced]:443: connect: network is unreachable |
| 829 | 2a06:98c1:310b::fda8:fa9e:4a3e | 2a06:98c1:310b::fda8:fa9e:4a3e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b::fda8:fa9e:4a3e]:443: connect: network is unreachable |
| 830 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:cfd2:7ebe:a043:8535]:443: connect: network is unreachable |
| 831 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:e263:6cdc:a8ce:a339]:443: connect: network is unreachable |
| 871 | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:8768:b929:7455:f040:5aee]:443: connect: network is unreachable |
| 872 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:de:2b25:85a5:8a26]:443: connect: network is unreachable |
| 873 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:22:21ad:d760:d542:16c8]:443: connect: network is unreachable |
| 874 | 2a06:98c1:310c::dd:f399:427e | 2a06:98c1:310c::dd:f399:427e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c::dd:f399:427e]:443: connect: network is unreachable |
| 875 | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:e1e7:ae26:af07:44a6:82da]:443: connect: network is unreachable |
| 877 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3]:443: connect: network is unreachable |
| 878 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:b3:af54:9923:e84:af58]:443: connect: network is unreachable |
| 879 | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:22:21cb:7546:1cd8:a79f]:443: connect: network is unreachable |
| 880 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:878:e123:da31:b2ee:2017]:443: connect: network is unreachable |
| 881 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:5d:a92a:97f:6fa3:f803]:443: connect: network is unreachable |
| 921 | 2803:f800:51:0:7c43:3bc0:435:fd7f | 2803:f800:51:0:7c43:3bc0:435:fd7f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:7c43:3bc0:435:fd7f]:443: connect: network is unreachable |
| 922 | 2a06:98c1:3101:923b:5f69:671a:d74d:697b | 2a06:98c1:3101:923b:5f69:671a:d74d:697b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:923b:5f69:671a:d74d:697b]:443: connect: network is unreachable |
| 923 | 2a06:98c1:310f:0:e963:a0b5:a697:c9ac | 2a06:98c1:310f:0:e963:a0b5:a697:c9ac | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:0:e963:a0b5:a697:c9ac]:443: connect: network is unreachable |
| 924 | 2a06:98c1:310b:0:e000:14c1:940:2095 | 2a06:98c1:310b:0:e000:14c1:940:2095 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e000:14c1:940:2095]:443: connect: network is unreachable |
| 925 | 2a06:98c1:310b:c5:4f:3c80:c19e:b358 | 2a06:98c1:310b:c5:4f:3c80:c19e:b358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:c5:4f:3c80:c19e:b358]:443: connect: network is unreachable |
| 926 | 2a06:98c1:3100:f5b1:2ce9:7430:9027:4c89 | 2a06:98c1:3100:f5b1:2ce9:7430:9027:4c89 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f5b1:2ce9:7430:9027:4c89]:443: connect: network is unreachable |
| 927 | 2a06:98c1:310a:d4b7:9608:9cfe:e697:55b9 | 2a06:98c1:310a:d4b7:9608:9cfe:e697:55b9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:d4b7:9608:9cfe:e697:55b9]:443: connect: network is unreachable |
| 928 | 2a06:98c1:3103:e8:2985:d043:e227:8130 | 2a06:98c1:3103:e8:2985:d043:e227:8130 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:e8:2985:d043:e227:8130]:443: connect: network is unreachable |
| 929 | 2400:cb00:2049:d6f5:3f86:37d0:9155:3773 | 2400:cb00:2049:d6f5:3f86:37d0:9155:3773 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:d6f5:3f86:37d0:9155:3773]:443: connect: network is unreachable |
| 947 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:aa:3e22:dd48:6279:eeb9]:443: connect: network is unreachable |
| 954 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:a594:2926:2b16:6e8d:843e]:443: connect: network is unreachable |
| 956 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2]:443: connect: network is unreachable |
| 957 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:fc87:e2d6:88c3:378b]:443: connect: network is unreachable |
| 958 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53]:443: connect: network is unreachable |
| 959 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:6e:e874:db4f:a1d5:2163]:443: connect: network is unreachable |
| 960 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:4f38:5221:f77f:fe11]:443: connect: network is unreachable |
| 961 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::ee:b8fb:877f]:443: connect: network is unreachable |
| 962 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3]:443: connect: network is unreachable |
| 963 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7]:443: connect: network is unreachable |
| 964 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9516:e4a1:4ba9:1c5e:7533]:443: connect: network is unreachable |
| 965 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea]:443: connect: network is unreachable |
| 966 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b]:443: connect: network is unreachable |
| 967 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:da84:1c63:f149:4d21:b339]:443: connect: network is unreachable |
| 968 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:54:2c60:eafc:f14d:ca4b]:443: connect: network is unreachable |
| 969 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f702:ebbf:618b:76c:9ba7]:443: connect: network is unreachable |
| 976 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:6a:7ba4:346b:e06c:71c7]:443: connect: network is unreachable |
| 978 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104::f3:8fed:cac0]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 583 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 989 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 991 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | none | N/A | 0 | N/A | dial tcp 104.20.255.53:443: i/o timeout |
| 998 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 268 次 (98.5%)
- **连接超时**: 4 次 (1.5%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 272 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 268 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), sullivan.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 732 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 1003 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 22 | cloudflare |
| 799 | 172.67.78.23 | 172.67.78.23 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 459 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 552 | 172.64.52.189 | 172.64.52.189 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 561 | 104.20.28.239 | 104.20.28.239 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 647 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 660 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 691 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 704 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 719 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 760 | 198.41.222.191 | 198.41.222.191 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 785 | 104.17.62.194 | 104.17.62.194 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 811 | 104.16.247.125 | 104.16.247.125 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 821 | 172.64.157.214 | 172.64.157.214 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 833 | 162.159.0.79 | 162.159.0.79 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 908 | 104.17.58.25 | 104.17.58.25 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 937 | 104.26.5.194 | 104.26.5.194 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 61 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 240 | palera.in | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 598 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 633 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 635 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 712 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 792 | 162.159.6.186 | 162.159.6.186 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 810 | 104.25.241.19 | 104.25.241.19 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 812 | 104.25.255.103 | 104.25.255.103 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 864 | 104.25.248.93 | 104.25.248.93 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 869 | 172.64.53.103 | 172.64.53.103 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 904 | 104.18.229.68 | 104.18.229.68 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 910 | 104.25.245.153 | 104.25.245.153 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 193 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 315 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 382 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 414 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 505 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 508 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 524 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 531 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 558 | 172.64.229.134 | 172.64.229.134 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 597 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 632 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 640 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 669 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 682 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 684 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 695 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 746 | 104.20.24.239 | 104.20.24.239 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 768 | 172.67.75.212 | 172.67.75.212 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 801 | 172.67.72.250 | 172.67.72.250 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 862 | 104.25.254.47 | 104.25.254.47 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 914 | 104.17.98.142 | 104.17.98.142 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 934 | 172.64.144.132 | 172.64.144.132 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 89 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 118 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 313 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 372 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 386 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 390 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 395 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 432 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 439 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 494 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 498 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 503 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 509 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 512 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 515 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 519 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 576 | 104.17.97.228 | 104.17.97.228 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 622 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 670 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 681 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 809 | 104.17.97.146 | 104.17.97.146 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 818 | 172.64.53.15 | 172.64.53.15 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 834 | 172.64.152.85 | 172.64.152.85 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 837 | 172.64.52.150 | 172.64.52.150 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 839 | 172.64.229.106 | 172.64.229.106 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 907 | 104.25.255.4 | 104.25.255.4 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 936 | 104.20.19.180 | 104.20.19.180 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 975 | 172.64.52.224 | 172.64.52.224 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 990 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 185 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 245 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 278 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 357 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 373 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 436 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 488 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 578 | 104.25.241.198 | 104.25.241.198 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 614 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 656 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 659 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 710 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 737 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 782 | 104.17.143.82 | 104.17.143.82 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 945 | 104.25.254.14 | 104.25.254.14 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 982 | 162.159.0.41 | 162.159.0.41 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 992 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 334 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 456 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 461 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 495 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 506 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 526 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 532 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 549 | 108.162.198.85 | 108.162.198.85 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 589 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 654 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 675 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 757 | 162.159.12.120 | 162.159.12.120 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 787 | 172.64.53.220 | 172.64.53.220 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 797 | 104.26.14.88 | 104.26.14.88 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 847 | 172.67.73.129 | 172.67.73.129 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 856 | 104.25.250.121 | 104.25.250.121 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 866 | 162.159.27.183 | 162.159.27.183 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 897 | 172.67.68.156 | 172.67.68.156 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 899 | 104.20.17.85 | 104.20.17.85 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 980 | 108.162.198.198 | 108.162.198.198 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 522 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 599 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 620 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 754 | 162.159.11.128 | 162.159.11.128 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 767 | 108.162.198.70 | 108.162.198.70 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 769 | 172.67.73.196 | 172.67.73.196 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 780 | 104.25.240.227 | 104.25.240.227 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 851 | 172.67.76.20 | 172.67.76.20 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 863 | 104.25.242.137 | 104.25.242.137 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 919 | 172.64.229.68 | 172.64.229.68 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 951 | 172.64.151.253 | 172.64.151.253 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 953 | 162.159.43.50 | 162.159.43.50 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 985 | 104.17.193.113 | 104.17.193.113 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 997 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 381 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 407 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 594 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 602 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 606 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 661 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 758 | 162.159.58.17 | 162.159.58.17 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 852 | 104.20.18.125 | 104.20.18.125 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 868 | 172.64.229.82 | 172.64.229.82 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 903 | 172.67.66.16 | 172.67.66.16 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 912 | 104.25.251.82 | 104.25.251.82 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 175 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 427 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 490 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 518 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 641 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 786 | 104.16.245.121 | 104.16.245.121 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 802 | 104.26.6.238 | 104.26.6.238 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 844 | 172.67.64.123 | 172.67.64.123 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 865 | 104.25.246.117 | 104.25.246.117 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 882 | 172.64.146.121 | 172.64.146.121 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 909 | 104.18.146.232 | 104.18.146.232 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 378 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 652 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 805 | 104.25.252.135 | 104.25.252.135 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 994 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 679 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 803 | 104.26.12.33 | 104.26.12.33 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 340 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 600 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 673 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 745 | 172.64.153.140 | 172.64.153.140 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 777 | 104.17.56.177 | 104.17.56.177 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 843 | 172.64.41.47 | 172.64.41.47 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 885 | 104.18.40.200 | 104.18.40.200 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 932 | 108.162.198.127 | 108.162.198.127 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 434 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 442 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 706 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 751 | 104.20.22.91 | 104.20.22.91 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 281 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 491 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 631 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 905 | 172.67.71.178 | 172.67.71.178 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 103 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 124 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 141 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 428 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 672 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 761 | 172.64.53.202 | 172.64.53.202 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 835 | 162.159.40.8 | 162.159.40.8 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 848 | 172.67.77.104 | 172.67.77.104 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 849 | 104.20.29.234 | 104.20.29.234 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 948 | 104.25.250.205 | 104.25.250.205 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 993 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 1001 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 452 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 855 | 104.25.241.235 | 104.25.241.235 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 451 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 20 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 363 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 507 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 766 | 162.159.1.39 | 162.159.1.39 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 527 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 643 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 884 | 104.18.47.193 | 104.18.47.193 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 938 | 172.67.72.212 | 172.67.72.212 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 950 | 162.159.33.191 | 162.159.33.191 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 79 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 387 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 389 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 465 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 504 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 571 | 172.67.65.150 | 172.67.65.150 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 575 | 104.26.8.171 | 104.26.8.171 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 653 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 705 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 743 | 104.17.111.150 | 104.17.111.150 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 749 | 172.67.70.56 | 172.67.70.56 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 946 | 104.25.240.123 | 104.25.240.123 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 75 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 83 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 112 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 125 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 160 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 168 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 181 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 341 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 379 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 497 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 694 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 723 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 770 | 104.26.5.53 | 104.26.5.53 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 783 | 104.25.242.249 | 104.25.242.249 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 800 | 104.20.29.62 | 104.20.29.62 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 836 | 108.162.198.223 | 108.162.198.223 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 900 | 172.67.70.197 | 172.67.70.197 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 942 | 104.25.244.36 | 104.25.244.36 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 972 | 172.64.229.15 | 172.64.229.15 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 24 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 39 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 72 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 108 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 174 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 176 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 205 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 206 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 213 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 276 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 280 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 299 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 316 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 376 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 511 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 687 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 713 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 784 | 104.18.160.38 | 104.18.160.38 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 796 | 104.26.1.55 | 104.26.1.55 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 814 | 104.17.129.66 | 104.17.129.66 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 815 | 104.25.253.253 | 104.25.253.253 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 918 | 162.159.8.49 | 162.159.8.49 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 973 | 172.64.53.41 | 172.64.53.41 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 19 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 56 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 78 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 82 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 85 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 92 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 99 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 156 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 167 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 190 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 229 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 239 | palera.in | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 255 | ip.gs | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 338 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 384 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 411 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 438 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 443 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 446 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 466 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 477 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 482 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 492 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 535 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 609 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 627 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 657 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 697 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 700 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 711 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 741 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 748 | 104.20.31.132 | 104.20.31.132 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 778 | 104.25.244.87 | 104.25.244.87 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 838 | 172.64.53.195 | 172.64.53.195 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 902 | 104.20.24.177 | 104.20.24.177 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 906 | 172.67.73.173 | 172.67.73.173 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 930 | 172.64.41.232 | 172.64.41.232 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 939 | 104.18.148.235 | 104.18.148.235 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 73 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 80 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 88 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 93 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 110 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 226 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 260 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 293 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 328 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 329 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 332 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 356 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 463 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 510 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 516 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 550 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 563 | 162.159.36.205 | 162.159.36.205 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 619 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 688 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 709 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 750 | 172.67.67.0 | 172.67.67.0 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 860 | 104.25.243.36 | 104.25.243.36 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 895 | 104.26.6.62 | 104.26.6.62 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 913 | 104.17.215.219 | 104.17.215.219 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 916 | 104.18.47.249 | 104.18.47.249 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 933 | 162.159.25.10 | 162.159.25.10 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 983 | 172.64.229.156 | 172.64.229.156 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 11 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 22 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 70 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 91 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 203 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 222 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 259 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 263 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 294 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 307 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 331 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 371 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 401 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 404 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 454 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 462 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 493 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 499 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 517 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 585 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 604 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 605 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 683 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 696 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 698 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 717 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 720 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 747 | 104.25.245.233 | 104.25.245.233 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 781 | 104.25.246.24 | 104.25.246.24 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 813 | 104.25.247.129 | 104.25.247.129 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 940 | 104.17.56.208 | 104.17.56.208 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 995 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 23 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 42 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 71 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 86 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 129 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 137 | freeyx.cloudflare88.eu.org | 141.101.121.171 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 224 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 250 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 288 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 335 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 368 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 409 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 430 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 448 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 500 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 501 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 513 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 569 | 104.26.6.247 | 104.26.6.247 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 651 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 680 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 689 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 699 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 703 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 774 | 172.67.65.81 | 172.67.65.81 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 793 | 172.64.229.149 | 172.64.229.149 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 795 | 162.159.21.16 | 162.159.21.16 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 807 | 172.64.146.137 | 172.64.146.137 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 861 | 104.25.252.192 | 104.25.252.192 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 867 | 162.159.49.244 | 162.159.49.244 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 917 | 172.64.53.93 | 172.64.53.93 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 999 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 1004 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 33 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 51 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 183 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 204 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 212 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 216 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 269 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 287 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 301 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 309 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 345 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 365 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 393 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 400 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 403 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 457 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 484 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 579 | 104.26.13.110 | 104.26.13.110 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 581 | 104.25.244.239 | 104.25.244.239 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 603 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 702 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 708 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 804 | 172.67.64.116 | 172.67.64.116 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 870 | 162.159.44.199 | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 893 | 172.64.156.202 | 172.64.156.202 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 58 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 87 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 95 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 98 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 117 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 119 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 140 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 235 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 251 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 314 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 364 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 388 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 412 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 437 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 450 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 480 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 658 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 664 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 773 | 104.20.22.141 | 104.20.22.141 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 779 | 104.25.254.89 | 104.25.254.89 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 806 | 172.67.76.61 | 172.67.76.61 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 901 | 172.67.76.220 | 172.67.76.220 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 977 | 162.159.39.20 | 162.159.39.20 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 74 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 153 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 189 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 391 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 447 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 528 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 533 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 570 | 104.20.25.181 | 104.20.25.181 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 615 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 663 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 665 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 678 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 701 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 788 | 172.64.52.194 | 172.64.52.194 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 894 | 172.64.49.77 | 172.64.49.77 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 55 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 64 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 97 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 132 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 135 | freeyx.cloudflare88.eu.org | 141.101.120.153 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 152 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 225 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 256 | ip.gs | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 284 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 429 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 455 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 588 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 621 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 996 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 4 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 43 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 148 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 464 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 486 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 666 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 690 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 816 | 172.64.229.185 | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 886 | 172.64.53.223 | 172.64.53.223 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 207 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 662 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 744 | 104.18.172.20 | 104.18.172.20 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 854 | 172.67.79.218 | 172.67.79.218 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 5 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 339 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 362 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 574 | 104.26.0.210 | 104.26.0.210 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 580 | 104.20.20.192 | 104.20.20.192 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 859 | 104.25.245.173 | 104.25.245.173 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 81 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 109 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 146 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 157 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 308 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 560 | 162.159.39.146 | 162.159.39.146 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 890 | 162.159.44.16 | 162.159.44.16 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 941 | 104.25.245.215 | 104.25.245.215 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 94 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 194 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 264 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 440 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 10 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 460 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 648 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 756 | 104.25.250.174 | 104.25.250.174 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 949 | 104.25.249.225 | 104.25.249.225 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 60 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 180 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 277 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 416 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 433 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 489 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 529 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 534 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 554 | 172.64.53.165 | 172.64.53.165 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 32 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 408 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 753 | 104.20.24.244 | 104.20.24.244 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 775 | 172.67.77.185 | 172.67.77.185 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 790 | 108.162.198.232 | 108.162.198.232 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 850 | 104.20.20.156 | 104.20.20.156 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 952 | 172.64.153.183 | 172.64.153.183 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 102 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 162 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 199 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 217 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 249 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 319 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 344 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 441 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 520 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 676 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 911 | 104.17.107.237 | 104.17.107.237 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 944 | 104.25.241.85 | 104.25.241.85 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 34 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 444 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 587 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 596 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 721 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 740 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 771 | 172.67.79.249 | 172.67.79.249 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 915 | 104.25.240.182 | 104.25.240.182 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 935 | 104.18.41.193 | 104.18.41.193 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 1000 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 1002 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 14 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 96 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 161 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 198 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 243 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 413 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 502 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 567 | 104.18.42.16 | 104.18.42.16 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 591 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 616 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 677 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 685 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 718 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 759 | 162.159.3.128 | 162.159.3.128 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 772 | 104.20.26.221 | 104.20.26.221 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 776 | 104.26.3.117 | 104.26.3.117 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 858 | 104.25.240.21 | 104.25.240.21 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 979 | 162.159.45.67 | 162.159.45.67 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 65 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 111 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 324 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 349 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 370 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 377 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 410 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 449 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 485 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 638 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 674 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 876 | 108.162.198.206 | 108.162.198.206 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 943 | 104.25.247.78 | 104.25.247.78 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 59 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 130 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 173 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 177 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 223 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 283 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 325 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 392 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 406 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 841 | 108.162.198.152 | 108.162.198.152 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 883 | 162.159.33.28 | 162.159.33.28 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 21 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 35 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 244 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 385 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 453 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 551 | 162.159.20.46 | 162.159.20.46 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 955 | 162.159.45.150 | 162.159.45.150 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 350 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 399 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 402 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 521 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 523 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 686 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 714 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 755 | 104.18.166.232 | 104.18.166.232 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 853 | 172.67.65.44 | 172.67.65.44 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 888 | 162.159.45.180 | 162.159.45.180 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 889 | 162.159.39.203 | 162.159.39.203 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 49 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 57 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 131 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 302 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 318 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 333 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 525 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 752 | 104.26.11.33 | 104.26.11.33 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 808 | 172.67.68.110 | 172.67.68.110 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 896 | 172.64.229.202 | 172.64.229.202 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 920 | 162.159.26.107 | 162.159.26.107 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 25 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 84 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 445 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 458 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 613 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 626 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 634 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 639 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 692 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 693 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 817 | 104.17.171.88 | 104.17.171.88 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 820 | 162.159.39.156 | 162.159.39.156 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 986 | 104.26.4.135 | 104.26.4.135 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 28 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 77 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 228 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 595 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 610 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 840 | 162.159.45.237 | 162.159.45.237 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 233 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 271 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 361 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 481 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 546 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 601 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 650 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 671 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 707 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 764 | 162.159.39.180 | 162.159.39.180 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 970 | 172.64.52.181 | 172.64.52.181 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 27 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 383 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 405 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 573 | 104.26.14.117 | 104.26.14.117 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 590 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 887 | 172.64.52.68 | 172.64.52.68 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 76 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 136 | freeyx.cloudflare88.eu.org | 141.101.120.51 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 265 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 351 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 586 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 715 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 15 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 26 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 182 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 380 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 415 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 655 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 819 | 104.18.44.159 | 104.18.44.159 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 892 | 108.162.198.216 | 108.162.198.216 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 310 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 355 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 487 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 496 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 742 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 90 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 123 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 227 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 298 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 435 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 514 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 630 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 547 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 974 | 162.159.38.83 | 162.159.38.83 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 289 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 479 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 483 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 898 | 104.20.25.24 | 104.20.25.24 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 234 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 343 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 540 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 842 | 162.159.44.60 | 162.159.44.60 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 104 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 113 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 642 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 789 | 162.159.44.246 | 162.159.44.246 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 798 | 162.159.39.26 | 162.159.39.26 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 50 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 431 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 536 | 162.159.38.226 | 162.159.38.226 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 582 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 208 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 147 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 794 | 162.159.38.52 | 162.159.38.52 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 857 | 104.26.4.213 | 104.26.4.213 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 984 | 162.159.44.202 | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 270 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 548 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 846 | 172.64.40.68 | 172.64.40.68 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 981 | 162.159.38.45 | 162.159.38.45 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 553 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 791 | 162.159.45.121 | 162.159.45.121 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 891 | 162.159.38.9 | 162.159.38.9 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 317 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 303 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 572 | 104.26.1.88 | 104.26.1.88 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 722 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 197 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 537 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 716 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 139 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 166 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 725 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 66 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 218 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 971 | 162.159.39.74 | 162.159.39.74 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 568 | 162.159.44.101 | 162.159.44.101 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 765 | 162.159.38.134 | 162.159.38.134 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 931 | 162.159.44.99 | 162.159.44.99 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 566 | 162.159.45.65 | 162.159.45.65 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 845 | 162.159.39.196 | 162.159.39.196 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 478 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 584 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 394 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 577 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 184 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 41 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare |
| 6 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 40 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 227 | cloudflare |
| 2 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 227 | cloudflare |
| 3 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 234 | cloudflare |
| 369 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 278 | cloudflare |
| 988 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 423 | cloudflare |
| 832 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 435 | cloudflare |
| 987 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 1108 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 212 条记录
- **快 (50-100ms)**: 491 条记录
- **正常 (100-200ms)**: 22 条记录
- **慢 (200-500ms)**: 6 条记录
- **很慢 (>500ms)**: 1 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 4 次
- **IPv6 失败**: 268 次

### 按协议统计

- **none**: 272 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
