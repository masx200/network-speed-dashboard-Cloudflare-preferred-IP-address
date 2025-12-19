# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:10:45
- **数据来源**: connectivity_results-20251219-021045.json
- **总测试数**: 457
- **失败测试数**: 172
- **成功测试数**: 285
- **失败率**: 37.64%
- **平均延迟**: 91.42ms
- **最小延迟**: 35ms
- **最大延迟**: 1240ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:10:45
- **IP地址**: 172.184.213.226
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

- **网络不可达: 网络不可达**: 169 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (169 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 5 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 6 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 7 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 11 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 12 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 13 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 17 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 18 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 19 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 49 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 50 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 59 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 60 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 63 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 64 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 68 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 69 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 70 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 81 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 82 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 83 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 87 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 88 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 89 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 96 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 97 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 98 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 102 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 103 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 104 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 108 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 109 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 110 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 114 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:12d8:5d82 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:f5e8:7af2:12d8:5d82]:443: connect: network is unreachable |
| 117 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 118 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 121 | bestcf.030101.xyz | 2606:4700:0:3701:6875:ca63:41c9:a311 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:3701:6875:ca63:41c9:a311]:443: connect: network is unreachable |
| 122 | bestcf.030101.xyz | 2606:4700:0:57:503f:ebaa:486e:53f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:57:503f:ebaa:486e:53f7]:443: connect: network is unreachable |
| 123 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 129 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 130 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 131 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 132 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 133 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 137 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 138 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 139 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 145 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 146 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 149 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 150 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 151 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 156 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 157 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 158 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 161 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 162 | xn--b6gac.eu.org | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 166 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 167 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 168 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 171 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 172 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 175 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 176 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 180 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 181 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 182 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 186 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 187 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 188 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 192 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 193 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 194 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 203 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 204 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 205 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 209 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 210 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 211 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 218 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 219 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 220 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 225 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 226 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 230 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 231 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 232 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 235 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 236 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 240 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 241 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 242 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 245 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 246 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 248 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 249 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 256 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 257 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 258 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 262 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 263 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 264 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 265 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 270 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 271 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 272 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 275 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 279 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 280 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 281 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 284 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 285 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 286 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 289 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 290 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 299 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 300 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 303 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 307 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 308 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 310 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 315 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 316 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 320 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 321 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 322 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 326 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 327 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 328 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 333 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 334 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable |
| 338 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 339 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 340 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 346 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 347 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 348 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 352 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 353 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 354 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 357 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 358 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 363 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 364 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 370 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 371 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 382 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 383 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 392 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 393 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 397 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 398 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 403 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 404 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 405 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 406 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 412 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 413 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 416 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 417 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 421 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 422 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 423 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 427 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 428 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 429 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 433 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 434 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 435 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 302 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 448 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 453 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 169 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.67（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 172 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 169 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), sullivan.ns.cloudflare.com (3次), craig.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 293 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 400 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 369 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 442 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 216 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 408 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 402 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 437 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 184 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 368 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 450 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 351 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 287 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 451 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 75 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 372 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 200 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 388 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 269 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 395 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 80 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 76 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 430 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 343 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 445 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 221 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 431 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 73 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 375 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 304 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 185 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 291 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 224 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 30 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 39 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 55 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 174 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 325 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 362 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 379 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 140 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 212 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 278 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 3 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 115 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 294 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 391 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 440 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 24 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 27 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 135 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 201 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 214 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 350 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 365 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 407 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 93 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 410 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 454 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 142 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 266 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 288 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 438 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 439 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 62 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 77 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 95 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 126 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 148 | cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 152 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 173 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 274 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 314 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 331 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 386 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 394 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 396 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 409 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 84 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 94 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 170 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 215 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 217 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 234 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 319 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 341 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 381 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 22 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 38 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 78 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 344 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 367 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 385 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 99 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 183 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 199 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 228 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 283 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 345 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 360 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 377 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 455 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 21 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 41 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 46 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 58 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 72 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 105 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 376 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 384 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 414 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 4 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 125 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 28 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 61 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 119 | bestcf.030101.xyz | 162.159.133.251 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 196 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 295 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 312 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 349 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 389 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 113 | freeyx.cloudflare88.eu.org | 141.101.120.95 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 154 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 177 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 207 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 251 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 259 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 335 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 390 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 457 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 34 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 56 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 159 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 198 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 227 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 238 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 255 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 366 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 26 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 134 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 155 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 233 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 298 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 9 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 250 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 254 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 292 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 420 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 425 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 456 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 31 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 33 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 66 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 74 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 229 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 296 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 297 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 411 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 444 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 67 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 309 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 323 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 44 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 160 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 169 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 267 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 399 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 143 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 29 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 35 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 51 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 276 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 318 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 359 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 374 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 443 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 16 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 37 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 147 | cf.zhetengsha.eu.org | 172.66.44.77 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 329 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 36 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 144 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 208 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 361 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 378 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 415 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 441 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 45 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 223 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 324 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 336 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 53 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 54 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 112 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 136 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 141 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 213 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 243 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 20 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 65 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 21 条记录
- **快 (50-100ms)**: 179 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 169 次

### 按协议统计

- **none**: 172 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
