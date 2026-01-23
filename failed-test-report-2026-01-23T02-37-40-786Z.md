# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/23 02:37:40
- **数据来源**: connectivity_results-20260123-023740.json
- **总测试数**: 778
- **失败测试数**: 230
- **成功测试数**: 548
- **失败率**: 29.56%
- **平均延迟**: 79.11ms
- **最小延迟**: 34ms
- **最大延迟**: 652ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/23 02:37:40
- **IP地址**: 172.184.210.214
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

- **网络不可达: 网络不可达**: 228 次 (99.1%)
- **连接超时: I/O超时**: 2 次 (0.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (228 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 2 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 8 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 9 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 10 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 17 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 18 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 19 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 24 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 25 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 26 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 29 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 30 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 34 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 35 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 36 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 39 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 40 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 44 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 45 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 46 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 50 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 51 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 52 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 55 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 56 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 65 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 66 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 67 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 71 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 72 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 73 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 77 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 78 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 79 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 107 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 110 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 111 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 113 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 116 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 117 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 123 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 124 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 125 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 127 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 133 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 134 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 135 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 136 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 137 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 141 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 142 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 143 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 147 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 148 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 149 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 150 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 154 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 155 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 156 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 159 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 160 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 161 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 164 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 165 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 177 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 178 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 179 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 183 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 187 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 188 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 189 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 193 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 194 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 195 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 199 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 200 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 203 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 204 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 210 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 214 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 215 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 216 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 220 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 221 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 222 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 225 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 226 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 230 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 231 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 232 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 235 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 236 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 239 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 240 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 243 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 244 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 264 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 265 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 267 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 271 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 272 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 277 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 278 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 279 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 285 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 286 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 287 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 290 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 291 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 294 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 295 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 296 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 300 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 301 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 302 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 317 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 318 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 326 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 327 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 332 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 333 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 334 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 339 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 340 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 346 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 347 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 348 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 355 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 356 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 357 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 361 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 362 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 363 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 369 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 370 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 371 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 376 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 377 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 378 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 382 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 383 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 384 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 387 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4f66:caf5]:443: connect: network is unreachable |
| 401 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 402 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 403 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 421 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 422 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 423 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 430 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 431 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 440 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 452 | bestcf.030101.xyz | 2606:4700:0:67b9:b065:a90a:a0a2:2889 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:67b9:b065:a90a:a0a2:2889]:443: connect: network is unreachable |
| 453 | bestcf.030101.xyz | 2606:4700:0:6704:6c5a:ab4f:1bd5:b002 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:6704:6c5a:ab4f:1bd5:b002]:443: connect: network is unreachable |
| 456 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 457 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 461 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 462 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 463 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 464 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 480 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 481 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 488 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 489 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 508 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 509 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 510 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 512 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 513 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 514 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 515 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 516 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 517 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 519 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 520 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 521 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 525 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 526 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 527 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 528 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 529 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 530 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 531 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 559 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 560 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 561 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 592 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 599 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 600 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 601 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 602 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 603 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 604 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 605 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 606 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 608 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 659 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 660 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 661 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 662 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 663 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 664 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 665 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 666 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 667 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 668 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 688 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:335e:fabe:8b11:3107:3685]:443: connect: network is unreachable |
| 695 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 696 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 730 | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d]:443: connect: network is unreachable |
| 731 | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:37c9:e79d:f57f:5979:246b]:443: connect: network is unreachable |
| 732 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837]:443: connect: network is unreachable |
| 733 | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:0:9437:4b55:9f2c:c3a]:443: connect: network is unreachable |
| 734 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:1f7b:99bd:a6f2:b984]:443: connect: network is unreachable |
| 735 | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:c7:ea87:3a0c:9a73:a94a]:443: connect: network is unreachable |
| 736 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:0:b1cc:b849:730:3d20]:443: connect: network is unreachable |
| 737 | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b]:443: connect: network is unreachable |
| 738 | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:0:9fb9:570f:6b2a:983f]:443: connect: network is unreachable |
| 739 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 752 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 753 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 755 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 756 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 757 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 759 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 760 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 761 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 762 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 726 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 778 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 228 次 (99.1%)
- **连接超时**: 2 次 (0.9%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 230 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 2 次，IPv6失败 228 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), time.is (3次), moura.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 548 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 539 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 84 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 246 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 404 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 585 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 672 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 289 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 446 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 684 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 704 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 769 | 162.159.40.162 | 162.159.40.162 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 198 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 426 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 490 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 552 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 590 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 651 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 765 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 315 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 379 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 424 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 533 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 534 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 713 | 104.19.38.110 | 104.19.38.110 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 749 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 758 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 311 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 385 | freeyx.cloudflare88.eu.org | 141.101.121.145 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 458 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 478 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 610 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 622 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 649 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 699 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 190 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 257 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 386 | freeyx.cloudflare88.eu.org | 141.101.120.120 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 460 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 469 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 493 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 579 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 582 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 621 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 625 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 690 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 715 | 104.17.181.90 | 104.17.181.90 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 747 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 89 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 112 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 180 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 253 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 328 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 417 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 427 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 472 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 540 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 686 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 728 | 172.64.229.3 | 172.64.229.3 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 307 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 455 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 467 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 548 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 630 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 748 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 408 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 441 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 474 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 566 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 571 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 573 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 581 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 642 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 652 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 716 | 104.18.94.137 | 104.18.94.137 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 341 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 365 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 583 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 640 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 767 | 172.64.52.80 | 172.64.52.80 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 118 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 620 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 646 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 698 | 104.26.10.159 | 104.26.10.159 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 705 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 205 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 366 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 484 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 524 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 565 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 707 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 330 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 624 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 675 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 711 | 104.26.8.60 | 104.26.8.60 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 750 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 754 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 656 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 416 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 678 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 343 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 612 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 400 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 550 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 658 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 368 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 549 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 207 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 439 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 483 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 637 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 173 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 202 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 217 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 251 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 319 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 352 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 415 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 468 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 505 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 537 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 557 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 673 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 682 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 23 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 60 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 64 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 131 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 151 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 176 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 375 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 473 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 535 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 644 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 693 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 82 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 99 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 249 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 392 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 411 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 591 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 54 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 63 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 174 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 228 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 288 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 325 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 547 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 702 | 104.26.3.200 | 104.26.3.200 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 53 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 92 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 101 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 209 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 238 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 241 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 242 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 250 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 256 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 258 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 269 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 314 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 393 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 395 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 410 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 475 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 477 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 532 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 546 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 631 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 647 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 676 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 681 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 722 | 104.17.57.66 | 104.17.57.66 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 770 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 158 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 166 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 223 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 268 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 305 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 316 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 351 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 367 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 372 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 442 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 491 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 496 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 538 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 542 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 578 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 595 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 720 | 104.17.58.98 | 104.17.58.98 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 744 | 172.64.52.204 | 172.64.52.204 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 766 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 94 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 171 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 175 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 282 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 324 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 499 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 506 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 554 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 569 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 587 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 634 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 12 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 80 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 90 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 97 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 132 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 237 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 259 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 306 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 338 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 394 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 406 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 407 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 432 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 495 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 639 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 725 | 104.17.221.233 | 104.17.221.233 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 28 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 41 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 49 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 93 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 103 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 114 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 182 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 398 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 425 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 437 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 586 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 607 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 609 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 629 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 632 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 700 | 104.20.27.188 | 104.20.27.188 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 721 | 104.18.92.250 | 104.18.92.250 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 37 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 57 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 62 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 233 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 234 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 280 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 344 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 420 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 429 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 570 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 648 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 687 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 710 | 172.67.71.177 | 172.67.71.177 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 729 | 162.159.44.249 | 162.159.44.249 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 7 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 33 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 70 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 86 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 168 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 196 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 312 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 390 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 445 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 492 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 523 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 545 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 575 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 594 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 655 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 689 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 717 | 104.16.155.40 | 104.16.155.40 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 87 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 108 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 119 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 126 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 172 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 273 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 284 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 342 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 414 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 448 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 536 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 98 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 197 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 281 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 299 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 303 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 309 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 433 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 476 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 562 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 650 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 679 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 706 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 751 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 206 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 329 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 413 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 447 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 449 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 593 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 638 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 685 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 74 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 201 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 498 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 589 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 633 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 692 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 69 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 109 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 146 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 335 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 337 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 359 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 412 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 511 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 14 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 122 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 320 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 336 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 680 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 106 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 208 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 556 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 614 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 645 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 701 | 104.20.23.55 | 104.20.23.55 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 43 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 58 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 61 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 270 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 292 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 293 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 321 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 555 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 88 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 224 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 297 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 466 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 577 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 654 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 718 | 172.67.73.198 | 172.67.73.198 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 771 | 108.162.198.183 | 108.162.198.183 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 772 | 172.64.229.233 | 172.64.229.233 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 95 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 138 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 170 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 381 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 388 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 574 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 691 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 81 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 96 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 261 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 436 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 471 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 479 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 486 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 553 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 712 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 323 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 353 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 677 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 59 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 83 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 163 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 212 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 262 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 276 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 434 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 584 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 626 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 742 | 162.159.45.140 | 162.159.45.140 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 153 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 186 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 247 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 248 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 266 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 443 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 451 | bestcf.030101.xyz | 172.64.159.47 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 543 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 551 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 746 | 198.41.223.250 | 198.41.223.250 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 162 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 350 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 450 | bestcf.030101.xyz | 104.17.176.62 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 482 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 576 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 623 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 636 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 643 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 708 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 13 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 91 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 121 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 503 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 641 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 697 | 162.159.36.94 | 162.159.36.94 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 21 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 27 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 47 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 169 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 349 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 391 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 497 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 588 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 635 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 694 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 773 | 162.159.44.17 | 162.159.44.17 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 75 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 184 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 218 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 263 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 723 | 104.17.177.91 | 104.17.177.91 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 260 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 397 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 465 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 741 | 162.159.38.113 | 162.159.38.113 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 764 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 22 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 157 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 275 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 405 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 409 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 616 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 670 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 709 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 774 | 162.159.39.95 | 162.159.39.95 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 11 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 31 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 100 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 219 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 254 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 360 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 435 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 470 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 580 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 768 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 115 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 252 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 454 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 564 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 628 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 653 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 120 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 298 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 304 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 438 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 494 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 714 | 104.26.13.215 | 104.26.13.215 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 727 | 108.162.198.146 | 108.162.198.146 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 743 | 172.64.53.110 | 172.64.53.110 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 48 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 255 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 274 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 444 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 563 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 617 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 5 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 144 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 308 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 502 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 596 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 68 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 213 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 313 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 399 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 724 | 172.64.158.41 | 172.64.158.41 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 763 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 191 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 389 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 619 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 16 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 522 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 568 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 572 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 683 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 128 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 139 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 373 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 485 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 500 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 703 | 104.26.5.25 | 104.26.5.25 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 740 | 162.159.48.112 | 162.159.48.112 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 167 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 544 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 6 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 76 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 102 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 358 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 396 | www.7749tv.com | 104.25.196.30 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 428 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 211 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 541 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 322 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 657 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 613 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 615 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 719 | 104.26.15.207 | 104.26.15.207 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 310 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 245 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 611 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 145 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 185 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 345 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 507 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 674 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 380 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 627 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 671 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 597 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 518 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 38 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 42 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 229 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 669 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 192 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 85 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 558 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 283 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 418 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 354 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 501 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 504 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 745 | 162.159.39.194 | 162.159.39.194 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 567 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 598 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 775 | 162.159.38.179 | 162.159.38.179 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 140 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 227 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 374 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 32 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 459 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 618 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 152 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 331 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 15 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 487 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 419 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 776 | 162.159.45.138 | 162.159.45.138 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 194 | cloudflare |
| 3 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 204 | cloudflare |
| 4 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 204 | cloudflare |
| 130 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 206 | cloudflare |
| 129 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 390 | cloudflare |
| 364 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 450 | cloudflare |
| 181 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 623 | cloudflare |
| 777 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 644 | cloudflare |
| 20 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 652 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 97 条记录
- **快 (50-100ms)**: 385 条记录
- **正常 (100-200ms)**: 58 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 3 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 2 次
- **IPv6 失败**: 228 次

### 按协议统计

- **none**: 230 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
