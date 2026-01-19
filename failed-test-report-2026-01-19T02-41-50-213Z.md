# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/19 02:41:50
- **数据来源**: connectivity_results-20260119-024149.json
- **总测试数**: 785
- **失败测试数**: 231
- **成功测试数**: 554
- **失败率**: 29.43%
- **平均延迟**: 99.21ms
- **最小延迟**: 43ms
- **最大延迟**: 790ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/19 02:41:50
- **IP地址**: 64.236.142.225
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
| 4 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 5 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 8 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 9 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 13 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 14 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 15 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 19 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 20 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 21 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 25 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 26 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 32 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 33 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 34 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 35 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 36 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 40 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 41 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 44 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 59 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 60 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 64 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 65 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 66 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 70 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 74 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 75 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 76 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 79 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 80 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 84 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 85 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 86 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 100 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 101 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 102 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 113 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 114 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 120 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 121 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 122 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 126 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 127 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 128 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 135 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 136 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 137 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 167 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 168 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 169 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 173 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 174 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 175 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 178 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 179 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 182 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 183 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 188 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 189 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 190 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 195 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 196 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 197 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 200 | bestcf.030101.xyz | 2606:4700:0:8b4f:d68d:41ec:64bb:f6bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:8b4f:d68d:41ec:64bb:f6bf]:443: connect: network is unreachable |
| 201 | bestcf.030101.xyz | 2606:4700::f982:f9ca:1eca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::f982:f9ca:1eca]:443: connect: network is unreachable |
| 202 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 208 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 209 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 212 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 213 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 216 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 217 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 223 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 224 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 225 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 226 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 230 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 231 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 232 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 236 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 237 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 241 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 242 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 243 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 247 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 248 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 249 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 253 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 254 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 255 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 259 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 260 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 261 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 269 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 270 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 274 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 275 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 276 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 282 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cdf7:bf7:57ea | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:cdf7:bf7:57ea]:443: connect: network is unreachable |
| 288 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 289 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 290 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 294 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 295 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 296 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 300 | ip.gs | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 301 | ip.gs | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 304 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 305 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 307 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 311 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 312 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 313 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 317 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 318 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 319 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 322 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 326 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 327 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 328 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 334 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 335 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 336 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 338 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 342 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 343 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 346 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 347 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 348 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 353 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 354 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 355 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 358 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 359 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 363 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 364 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 373 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 374 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 375 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 376 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 381 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 382 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 383 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 385 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 389 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 393 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 394 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 395 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 402 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 403 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 404 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 408 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 409 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 412 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 413 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 420 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 421 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 422 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 464 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 465 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 466 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 467 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 468 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 469 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 470 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 471 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 472 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 473 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 514 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 515 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 516 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 517 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 518 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 519 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 520 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 521 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 522 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 523 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 583 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 584 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 585 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 586 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 587 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 588 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 589 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 590 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 591 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 592 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 626 | 2a06:98c1:3108:2538:ff37:9d72:2b2e:a61f | 2a06:98c1:3108:2538:ff37:9d72:2b2e:a61f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:2538:ff37:9d72:2b2e:a61f]:443: connect: network is unreachable |
| 631 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 632 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 637 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 638 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 654 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 655 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 656 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 657 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 658 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 659 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 660 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 661 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 662 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 663 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 680 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 681 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 682 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 687 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 688 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 714 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 715 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 716 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 717 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 718 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 719 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 720 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 722 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 723 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 724 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 764 | 2803:f800:51:b8:746f:a352:43e8:3c2a | 2803:f800:51:b8:746f:a352:43e8:3c2a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:b8:746f:a352:43e8:3c2a]:443: connect: network is unreachable |
| 765 | 2a06:98c1:3106:0:54b2:2751:f330:de0a | 2a06:98c1:3106:0:54b2:2751:f330:de0a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:54b2:2751:f330:de0a]:443: connect: network is unreachable |
| 766 | 2a06:98c1:310a:1a6a:bc43:5b7:9d0f:62f3 | 2a06:98c1:310a:1a6a:bc43:5b7:9d0f:62f3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:1a6a:bc43:5b7:9d0f:62f3]:443: connect: network is unreachable |
| 767 | 2803:f800:51::717b:266f:2bc | 2803:f800:51::717b:266f:2bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51::717b:266f:2bc]:443: connect: network is unreachable |
| 768 | 2a06:98c1:3100:61:d3df:bc30:67f7:c5a | 2a06:98c1:3100:61:d3df:bc30:67f7:c5a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:61:d3df:bc30:67f7:c5a]:443: connect: network is unreachable |
| 769 | 2a06:98c1:310b:e48e:b8b6:72b6:a637:5a5d | 2a06:98c1:310b:e48e:b8b6:72b6:a637:5a5d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:e48e:b8b6:72b6:a637:5a5d]:443: connect: network is unreachable |
| 770 | 2a06:98c1:310f:398a:fb7c:46a:71b3:4cd0 | 2a06:98c1:310f:398a:fb7c:46a:71b3:4cd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:398a:fb7c:46a:71b3:4cd0]:443: connect: network is unreachable |
| 771 | 2a06:98c1:3105:6e:f984:95fe:9b26:f194 | 2a06:98c1:3105:6e:f984:95fe:9b26:f194 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:6e:f984:95fe:9b26:f194]:443: connect: network is unreachable |
| 772 | 2a06:98c1:3108:0:7a27:7d2:b20f:dd0d | 2a06:98c1:3108:0:7a27:7d2:b20f:dd0d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:0:7a27:7d2:b20f:dd0d]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 781 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 784 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 785 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 228 次 (98.7%)
- **连接超时**: 3 次 (1.3%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 231 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 228 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 554 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 624 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 95 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 495 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 640 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 782 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 207 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 491 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 639 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 684 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 156 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 678 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 710 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 432 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 536 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 734 | 104.18.41.10 | 104.18.41.10 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 754 | 104.16.145.125 | 104.16.145.125 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 244 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 384 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 544 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 578 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 725 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 750 | 104.19.149.143 | 104.19.149.143 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 184 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 462 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 563 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 612 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 685 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 690 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 701 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 221 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 293 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 476 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 509 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 543 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 548 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 629 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 696 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 763 | 162.159.13.208 | 162.159.13.208 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 777 | 172.64.144.130 | 172.64.144.130 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 533 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 560 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 344 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 556 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 730 | 162.159.3.132 | 162.159.3.132 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 118 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 333 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 357 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 532 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 566 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 708 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 753 | 104.17.126.38 | 104.17.126.38 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 22 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 264 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 673 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 756 | 104.19.155.152 | 104.19.155.152 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 42 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 94 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 320 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 447 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 503 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 524 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 542 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 405 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 424 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 438 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 633 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 686 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 159 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 321 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 330 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 446 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 620 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 705 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 748 | 104.26.1.44 | 104.26.1.44 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 28 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 106 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 111 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 455 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 478 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 568 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 575 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 738 | 104.20.29.127 | 104.20.29.127 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 749 | 104.17.30.63 | 104.17.30.63 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 98 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 110 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 177 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 219 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 262 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 286 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 287 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 461 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 649 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 674 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 139 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 371 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 775 | 162.159.24.205 | 162.159.24.205 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 214 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 365 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 386 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 499 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 607 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 463 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 565 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 636 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 712 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 737 | 172.64.154.12 | 172.64.154.12 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 747 | 104.20.28.71 | 104.20.28.71 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 452 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 481 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 564 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 689 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 185 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 437 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 497 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 576 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 644 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 679 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 703 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 92 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 504 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 546 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 731 | 162.159.42.32 | 162.159.42.32 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 30 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 155 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 257 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 431 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 562 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 732 | 104.18.42.35 | 104.18.42.35 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 77 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 103 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 104 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 152 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 541 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 668 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 758 | 162.159.40.85 | 162.159.40.85 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 24 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 87 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 303 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 569 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 697 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 141 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 391 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 417 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 454 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 474 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 759 | 162.159.18.2 | 162.159.18.2 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 116 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 271 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 345 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 439 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 554 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 628 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 733 | 162.159.49.39 | 162.159.49.39 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 47 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 138 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 180 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 755 | 104.16.240.165 | 104.16.240.165 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 83 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 132 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 140 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 166 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 187 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 193 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 440 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 615 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 695 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 369 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 429 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 529 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 751 | 172.67.64.153 | 172.67.64.153 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 51 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 332 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 477 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 506 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 627 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 52 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 229 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 350 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 611 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 134 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 291 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 482 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 510 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 572 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 218 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 666 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 115 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 267 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 310 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 641 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 711 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 280 | freeyx.cloudflare88.eu.org | 141.101.120.61 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 406 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 596 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 2 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 43 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 479 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 692 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 721 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 6 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 49 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 240 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 263 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 340 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 425 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 3 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 112 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 252 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 397 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 501 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 675 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 677 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 153 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 154 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 164 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 199 | bestcf.030101.xyz | 104.17.120.229 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 235 | cf.090227.xyz | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 302 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 574 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 604 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 125 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 192 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 277 | freeyx.cloudflare88.eu.org | 141.101.120.23 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 430 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 457 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 558 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 617 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 699 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 222 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 258 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 456 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 549 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 573 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 778 | 172.64.48.175 | 172.64.48.175 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 93 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 204 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 206 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 233 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 251 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 339 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 379 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 500 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 505 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 148 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 161 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 306 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 329 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 459 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 602 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 68 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 107 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 151 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 427 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 559 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 618 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 779 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 130 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 160 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 279 | freeyx.cloudflare88.eu.org | 141.101.121.155 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 490 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 571 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 653 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 752 | 104.17.31.153 | 104.17.31.153 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 29 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 90 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 368 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 410 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 448 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 484 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 577 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 579 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 776 | 162.159.58.15 | 162.159.58.15 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 18 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 61 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 91 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 181 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 299 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 316 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 392 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 493 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 512 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 613 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 743 | 104.26.7.153 | 104.26.7.153 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 210 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 245 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 370 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 507 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 535 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 729 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 78 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 475 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 630 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 194 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 337 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 443 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 683 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 735 | 162.159.40.159 | 162.159.40.159 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 162 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 220 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 416 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 492 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 538 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 31 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 73 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 149 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 211 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 361 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 551 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 667 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 669 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 88 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 129 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 485 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 487 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 534 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 650 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 23 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 45 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 54 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 71 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 37 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 228 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 349 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 528 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 593 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 39 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 56 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 297 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 496 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 55 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 72 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 145 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 176 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 191 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 278 | freeyx.cloudflare88.eu.org | 141.101.120.182 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 398 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 407 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 324 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 486 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 526 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 610 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 625 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 745 | 172.67.73.236 | 172.67.73.236 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 81 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 108 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 123 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 266 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 308 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 356 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 399 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 434 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 508 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 614 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 53 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 239 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 411 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 622 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 623 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 652 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 762 | 172.64.157.187 | 172.64.157.187 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 142 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 315 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 451 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 597 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 670 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 693 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 57 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 250 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 265 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 360 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 488 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 702 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 12 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 367 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 736 | 172.64.48.205 | 172.64.48.205 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 760 | 104.19.153.238 | 104.19.153.238 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 774 | 172.64.42.141 | 172.64.42.141 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 46 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 227 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 378 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 441 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 757 | 104.19.51.31 | 104.19.51.31 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 205 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 285 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 498 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 146 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 246 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 665 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 676 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 58 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 150 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 158 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 268 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 513 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 605 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 82 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 380 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 400 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 458 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 545 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 598 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 671 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 728 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 119 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 133 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 284 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 423 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 525 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 561 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 606 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 621 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 642 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 198 | bestcf.030101.xyz | 104.16.254.88 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 309 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 377 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 414 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 435 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 539 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 582 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 600 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 171 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 616 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 707 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 742 | 172.67.69.156 | 172.67.69.156 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 783 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 62 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 99 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 117 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 272 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 483 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 672 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 698 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 144 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 203 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 480 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 17 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 170 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 450 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 502 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 595 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 48 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 557 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 713 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 739 | 162.159.26.213 | 162.159.26.213 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 780 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 7 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 89 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 215 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 634 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 647 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 16 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 256 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 96 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 273 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 325 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 553 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 709 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 726 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 234 | cf.090227.xyz | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 390 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 401 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 489 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 694 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 172 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 580 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 700 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 298 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 436 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 444 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 601 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 643 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 460 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 552 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 706 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 442 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 494 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 744 | 172.67.79.160 | 172.67.79.160 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 147 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 157 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 143 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 283 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 362 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 396 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 449 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 773 | 172.64.156.31 | 172.64.156.31 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 352 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 740 | 104.20.23.184 | 104.20.23.184 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 281 | freeyx.cloudflare88.eu.org | 141.101.120.7 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 292 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 761 | 172.64.155.187 | 172.64.155.187 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 323 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 537 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 105 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 555 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 69 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 433 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 387 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 651 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 741 | 172.67.70.163 | 172.67.70.163 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 530 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 10 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 238 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 314 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 419 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 531 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 11 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 635 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 603 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 163 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 570 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 418 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 581 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 599 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 351 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 97 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 646 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 341 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 550 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 331 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 547 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 648 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 162 | cloudflare |
| 445 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare |
| 664 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 704 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 527 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare |
| 619 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare |
| 388 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 511 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 540 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 691 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 173 | cloudflare |
| 27 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 178 | cloudflare |
| 165 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 178 | cloudflare |
| 109 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 179 | cloudflare |
| 186 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 180 | cloudflare |
| 131 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 426 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 428 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 190 | cloudflare |
| 746 | 162.159.34.78 | 162.159.34.78 | IPv4 | h3 | ✅ 成功 | 191 | cloudflare |
| 124 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 192 | cloudflare |
| 372 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 193 | cloudflare |
| 594 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 196 | cloudflare |
| 645 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 214 | cloudflare |
| 608 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 217 | cloudflare |
| 453 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 218 | cloudflare |
| 63 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 244 | cloudflare |
| 609 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 319 | cloudflare |
| 567 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 371 | cloudflare |
| 38 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 433 | cloudflare |
| 727 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 487 | cloudflare |
| 366 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 505 | cloudflare |
| 415 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 635 | cloudflare |
| 67 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 790 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 29 条记录
- **快 (50-100ms)**: 298 条记录
- **正常 (100-200ms)**: 216 条记录
- **慢 (200-500ms)**: 8 条记录
- **很慢 (>500ms)**: 3 条记录


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
