# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:11:59
- **数据来源**: connectivity_results-20251219-021158.json
- **总测试数**: 455
- **失败测试数**: 173
- **成功测试数**: 282
- **失败率**: 38.02%
- **平均延迟**: 116.22ms
- **最小延迟**: 45ms
- **最大延迟**: 600ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:12:00
- **IP地址**: 68.154.116.69
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 170 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (170 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 5 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 6 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 9 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 10 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 16 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 17 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 18 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 19 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 20 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 24 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 25 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 26 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 30 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 31 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 32 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 47 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 48 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 49 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 53 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 54 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 55 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 90 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 91 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 92 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 96 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 97 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 98 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 102 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 103 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 104 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 107 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 108 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 110 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 113 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 114 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 116 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 119 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 120 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 124 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 125 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 126 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 130 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 134 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 135 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 136 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 140 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 141 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 142 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 145 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 150 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 151 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 152 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 153 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 156 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 157 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 166 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 167 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 170 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 171 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 173 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 177 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 178 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 180 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 184 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 185 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 186 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 189 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 190 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 195 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 196 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 197 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 200 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 201 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable |
| 207 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 208 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 209 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 213 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 214 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 215 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 220 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 221 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 224 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 225 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 228 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 229 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 232 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 233 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 239 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 240 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 243 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 244 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 254 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 255 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 256 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 263 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 264 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 265 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 269 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 270 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 271 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 278 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 279 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 283 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 284 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 285 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 291 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 292 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 293 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 294 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 298 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 299 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 300 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 308 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 309 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 325 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 326 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 327 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 330 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 331 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 338 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 339 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 340 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 346 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa:66d2:154e:f17c:9632 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa:66d2:154e:f17c:9632]:443: connect: network is unreachable |
| 347 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa73:2243:9add:8c5a:8a20 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa73:2243:9add:8c5a:8a20]:443: connect: network is unreachable |
| 351 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 352 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 353 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 357 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 358 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 359 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 363 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 364 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 365 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 369 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 370 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 371 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 375 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 376 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 377 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 381 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 382 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 383 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 386 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 387 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 390 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 391 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 396 | bestcf.030101.xyz | 2606:4700:0:57:503f:ebaa:486e:53f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:57:503f:ebaa:486e:53f7]:443: connect: network is unreachable |
| 397 | bestcf.030101.xyz | 2606:4700:0:3701:6875:ca63:41c9:a311 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:3701:6875:ca63:41c9:a311]:443: connect: network is unreachable |
| 401 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 404 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 405 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 408 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 409 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 413 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 414 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 415 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 419 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 420 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 421 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 423 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 426 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 427 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 434 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 435 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 436 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 440 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 441 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 442 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 445 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 446 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 451 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 454 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 455 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 170 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 173 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 170 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), benedict.ns.cloudflare.com (3次), moura.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 430 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 384 | cloudflare-ip.mofashi.ltd | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 250 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 354 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 406 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 317 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 342 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 321 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 172 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 203 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 147 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 179 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 63 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 230 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 199 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 315 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 345 | freeyx.cloudflare88.eu.org | 141.101.120.207 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 65 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 437 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 163 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 282 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 276 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 416 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 80 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 160 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 234 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 257 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 56 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 111 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 372 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 95 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 223 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 267 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 144 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 248 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 374 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 35 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 50 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 72 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 226 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 275 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 133 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 181 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 332 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 205 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 62 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 335 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 350 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 37 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 82 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 274 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 129 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 211 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 222 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 433 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 11 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 57 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 227 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 122 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 182 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 36 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 40 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 51 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 64 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 93 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 314 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 77 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 343 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 328 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 4 | www.ipget.net | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 83 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 22 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 39 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 68 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 302 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 319 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 261 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 304 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 164 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 85 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 204 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 281 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 301 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 165 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 210 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 260 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 266 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 287 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 305 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 373 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 115 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 443 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 449 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 89 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 236 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 3 | www.ipget.net | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 43 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 392 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 117 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 289 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 322 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 329 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 69 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 78 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 237 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 259 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 297 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 337 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 15 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 46 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 118 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 306 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 402 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 34 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 45 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 58 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 101 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 169 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 252 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 86 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 188 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 432 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 450 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 21 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 81 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 94 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 191 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 295 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 368 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 394 | bestcf.030101.xyz | 104.19.146.144 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 410 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 453 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 79 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 217 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 238 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 312 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 333 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 407 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 431 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 12 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 128 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 242 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 14 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 41 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 75 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 84 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 109 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 324 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 360 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 425 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 42 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 106 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 174 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 272 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 366 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 378 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 74 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 137 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 362 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 33 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 127 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 155 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 218 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 241 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 246 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 355 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 403 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 452 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 52 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 100 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 143 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 235 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 286 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 8 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 273 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 334 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 341 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 168 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 198 | ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 60 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 76 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 245 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 288 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 303 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 399 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 412 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 66 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 71 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 162 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 310 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 313 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 29 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 361 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 400 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 121 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 154 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 175 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 194 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 253 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 95 条记录
- **正常 (100-200ms)**: 104 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 170 次

### 按协议统计

- **none**: 173 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
