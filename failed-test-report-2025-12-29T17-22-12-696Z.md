# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:22:12
- **数据来源**: connectivity_results-20251229-172212.json
- **总测试数**: 494
- **失败测试数**: 181
- **成功测试数**: 313
- **失败率**: 36.64%
- **平均延迟**: 96.77ms
- **最小延迟**: 53ms
- **最大延迟**: 606ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:22:12
- **IP地址**: 20.169.50.34
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4532, -112.0748
- **时区**: America/Phoenix
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
| 2 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 7 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 10 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 11 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 14 | bestcf.030101.xyz | 2606:4700::8d:f082:8938:66d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::8d:f082:8938:66d8]:443: connect: network is unreachable |
| 15 | bestcf.030101.xyz | 2606:4700::fffd:819d:acda | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::fffd:819d:acda]:443: connect: network is unreachable |
| 18 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 19 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 22 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 23 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 26 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 27 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 28 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 35 | xn--b6gac.eu.org | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 36 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 40 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 41 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 42 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 46 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 47 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 48 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 51 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 52 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 57 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 58 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 62 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 63 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 64 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 68 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 69 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 70 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 77 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 78 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 79 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 83 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 84 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 85 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 89 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 90 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 91 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 95 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 96 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 97 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 103 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 104 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 110 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 111 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 112 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 115 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 116 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 120 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 121 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 122 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 125 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 126 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 127 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 130 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 135 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 136 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 137 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 138 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 144 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 145 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 146 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 147 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 151 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 152 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 153 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 157 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 158 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 159 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 160 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 165 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 166 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 171 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 172 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 179 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 180 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 181 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 184 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 185 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 188 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 194 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 195 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 196 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 200 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 201 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 202 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 205 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 213 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 214 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 215 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 218 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 219 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 223 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 224 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 225 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 230 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 231 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 237 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 238 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 239 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 275 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 276 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 279 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 280 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 287 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 288 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 289 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 295 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 296 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 297 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 298 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 299 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 302 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 303 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 306 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 307 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 311 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 312 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 324 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 325 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 330 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 331 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 332 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 340 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 341 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 342 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 343 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 352 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 353 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 354 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 357 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 358 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 360 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4f66:caf5]:443: connect: network is unreachable |
| 367 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 368 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 369 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 399 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 400 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 401 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 409 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 410 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 411 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 416 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 417 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 421 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 422 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 423 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 433 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 434 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 435 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 439 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 440 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 441 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 445 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 446 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 447 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 451 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 452 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 453 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 456 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 457 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 459 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 460 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 461 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 462 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 463 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 464 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 472 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 473 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 474 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 477 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 478 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 482 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 483 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 484 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 488 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 492 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 494 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), www.whatismyip.com (3次), braden.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 176 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 318 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 309 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 313 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 366 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 128 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 338 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 31 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 204 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 217 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 257 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 278 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 54 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 75 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 65 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 73 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 208 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 475 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 485 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 8 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 274 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 317 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 61 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 234 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 402 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 432 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 24 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 333 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 335 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 346 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 415 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 418 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 469 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 293 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 450 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 466 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 37 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 60 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 393 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 397 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 13 | bestcf.030101.xyz | 104.19.147.41 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 25 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 67 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 114 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 150 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 286 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 314 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 256 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 351 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 467 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 337 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 362 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 405 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 43 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 101 | palera.in | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 163 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 220 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 232 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 376 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 489 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 45 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 178 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 378 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 167 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 189 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 266 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 304 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 348 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 419 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 424 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 233 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 379 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 404 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 429 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 133 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 240 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 243 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 292 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 300 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 329 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 380 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 382 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 12 | bestcf.030101.xyz | 198.41.209.230 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 94 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 100 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 244 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 254 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 277 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 285 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 406 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 71 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 102 | palera.in | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 199 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 334 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 365 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 414 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 426 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 226 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 227 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 265 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 344 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 359 | freeyx.cloudflare88.eu.org | 141.101.120.109 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 99 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 174 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 294 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 301 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 408 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 449 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 149 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 269 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 282 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 320 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 412 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 454 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 124 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 193 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 420 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 59 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 161 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 235 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 264 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 273 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 375 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 438 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 72 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 80 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 107 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 386 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 481 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 16 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 50 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 76 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 162 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 175 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 186 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 261 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 283 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 347 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 443 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 44 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 92 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 98 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 216 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 252 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 259 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 437 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 390 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 53 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 177 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 267 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 315 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 336 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 21 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 34 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 38 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 74 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 154 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 241 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 249 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 308 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 155 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 270 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 403 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 3 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 82 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 109 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 183 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 191 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 248 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 253 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 316 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 349 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 407 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 425 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 470 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 132 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 140 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 203 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 319 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 327 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 385 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 389 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 32 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 118 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 148 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 197 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 328 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 355 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 356 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 370 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 395 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 431 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 4 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 173 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 182 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 198 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 284 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 372 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 381 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 458 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 200 条记录
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
