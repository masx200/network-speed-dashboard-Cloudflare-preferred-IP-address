# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/23 02:38:14
- **数据来源**: connectivity_results-20260123-023813.json
- **总测试数**: 770
- **失败测试数**: 230
- **成功测试数**: 540
- **失败率**: 29.87%
- **平均延迟**: 105.42ms
- **最小延迟**: 54ms
- **最大延迟**: 1730ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/23 02:38:14
- **IP地址**: 172.182.213.3
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

- **网络不可达: 网络不可达**: 228 次 (99.1%)
- **连接超时: I/O超时**: 2 次 (0.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (228 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 7 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 8 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 12 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 13 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 14 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 17 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 18 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 22 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 23 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 24 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 36 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 37 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 38 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 42 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 43 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 47 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 48 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 53 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 54 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 57 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 58 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 61 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 62 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 63 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 71 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 72 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 73 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 74 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 75 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 82 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 83 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 84 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 87 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 88 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 92 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 93 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 94 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 130 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 131 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 132 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 136 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 137 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 138 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 141 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 142 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 146 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 147 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 148 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 154 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 155 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 156 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 159 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:100d:4f66:caf5]:443: connect: network is unreachable |
| 165 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 166 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 167 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 171 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 172 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 173 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 179 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 180 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 181 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 184 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 185 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 188 | bestcf.030101.xyz | 2606:4700:0:67b9:b065:a90a:a0a2:2889 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:67b9:b065:a90a:a0a2:2889]:443: connect: network is unreachable |
| 189 | bestcf.030101.xyz | 2606:4700:0:6704:6c5a:ab4f:1bd5:b002 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:6704:6c5a:ab4f:1bd5:b002]:443: connect: network is unreachable |
| 190 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 194 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 195 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 196 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 203 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 204 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 205 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 208 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 209 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 211 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 214 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 215 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 218 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 219 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 223 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 224 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 225 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 229 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 230 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 231 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 238 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 239 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 243 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 244 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 245 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 249 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 250 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 251 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 255 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 256 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 257 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 263 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 264 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 265 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 269 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 270 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 271 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 279 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 280 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 284 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 285 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 291 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 292 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 293 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 297 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 298 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 299 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 301 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 303 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 307 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 308 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 309 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 314 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 315 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 316 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 320 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 321 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 322 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 327 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 328 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 329 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 332 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 333 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 334 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 339 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 340 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 341 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 344 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 345 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 349 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 350 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 356 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 357 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 361 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 364 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 365 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 368 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 374 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 375 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 376 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 380 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 381 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 382 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 384 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 389 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 390 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 391 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 395 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 396 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 400 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 401 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 402 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 424 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 425 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 432 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 433 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 434 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 457 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 458 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 459 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 460 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 461 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 462 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 463 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 464 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 465 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 466 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 507 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 508 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 509 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 510 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 511 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 512 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 513 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 514 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 516 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 517 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 564 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 576 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 577 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 578 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 579 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 580 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 581 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 582 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 583 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 584 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 585 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 636 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 637 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 639 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 640 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 641 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 642 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 643 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 644 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 645 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 685 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | 2a06:98c1:3103:335e:fabe:8b11:3107:3685 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:335e:fabe:8b11:3107:3685]:443: connect: network is unreachable |
| 696 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 697 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 698 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 699 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 700 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 701 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 702 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 703 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 704 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 705 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 741 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 742 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 748 | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | 2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:a7:4a3f:d3c5:7b8f:bb9d]:443: connect: network is unreachable |
| 749 | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | 2a06:98c1:50:37c9:e79d:f57f:5979:246b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:37c9:e79d:f57f:5979:246b]:443: connect: network is unreachable |
| 750 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | 2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:6fd:f3b9:2a1f:92f9:837]:443: connect: network is unreachable |
| 751 | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | 2a06:98c1:51:0:9437:4b55:9f2c:c3a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:0:9437:4b55:9f2c:c3a]:443: connect: network is unreachable |
| 752 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | 2a06:98c1:3105:0:1f7b:99bd:a6f2:b984 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:1f7b:99bd:a6f2:b984]:443: connect: network is unreachable |
| 753 | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | 2803:f800:51:c7:ea87:3a0c:9a73:a94a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:c7:ea87:3a0c:9a73:a94a]:443: connect: network is unreachable |
| 754 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | 2a06:98c1:3101:0:b1cc:b849:730:3d20 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:0:b1cc:b849:730:3d20]:443: connect: network is unreachable |
| 755 | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | 2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:51df:ad9d:e0d1:7ca:494b]:443: connect: network is unreachable |
| 756 | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | 2400:cb00:f00e:0:9fb9:570f:6b2a:983f | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:0:9fb9:570f:6b2a:983f]:443: connect: network is unreachable |
| 763 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 764 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 765 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 483 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 770 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 228 次 (99.1%)
- **连接超时**: 2 次 (0.9%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.67（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 230 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 2 次，IPv6失败 228 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 540 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 744 | 162.159.48.112 | 162.159.48.112 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 600 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 653 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 594 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 687 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 418 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 482 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 604 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 694 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 739 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 78 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 326 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 543 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 711 | 172.64.52.80 | 172.64.52.80 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 306 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 656 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 86 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 556 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 596 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 654 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 740 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 128 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 498 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 638 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 668 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 367 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 549 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 281 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 394 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 471 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 601 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 70 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 417 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 452 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 477 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 544 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 609 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 635 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 715 | 172.64.229.233 | 172.64.229.233 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 721 | 172.67.71.177 | 172.67.71.177 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 236 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 406 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 683 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 104 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 126 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 300 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 378 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 505 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 528 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 546 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 672 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 757 | 172.64.53.110 | 172.64.53.110 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 16 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 102 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 410 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 566 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 670 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 96 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 440 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 502 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 555 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 568 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 586 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 32 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 65 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 99 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 120 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 287 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 405 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 631 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 709 | 162.159.44.17 | 162.159.44.17 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 108 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 174 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 207 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 497 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 533 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 536 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 175 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 342 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 362 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 480 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 500 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 565 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 123 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 149 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 275 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 467 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 469 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 575 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 331 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 403 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 447 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 518 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 547 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 625 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 674 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 676 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 679 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 4 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 456 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 468 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 475 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 572 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 734 | 104.17.177.91 | 104.17.177.91 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 100 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 114 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 119 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 288 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 385 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 539 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 733 | 104.17.58.98 | 104.17.58.98 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 232 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 233 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 444 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 551 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 665 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 669 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 680 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 731 | 104.18.94.137 | 104.18.94.137 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 113 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 186 | bestcf.030101.xyz | 104.17.182.228 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 304 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 370 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 372 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 495 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 563 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 724 | 104.26.13.215 | 104.26.13.215 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 29 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 112 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 504 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 716 | 162.159.40.162 | 162.159.40.162 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 30 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 105 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 492 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 718 | 104.20.23.55 | 104.20.23.55 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 353 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 632 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 162 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 254 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 330 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 441 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 747 | 162.159.38.113 | 162.159.38.113 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 134 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 140 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 348 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 442 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 91 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 151 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 323 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 404 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 478 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 490 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 519 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 649 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 659 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 31 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 192 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 282 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 324 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 557 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 684 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 767 | 198.41.223.250 | 198.41.223.250 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 412 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 451 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 526 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 573 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 624 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 650 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 681 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 729 | 104.19.38.110 | 104.19.38.110 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 21 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 97 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 111 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 445 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 537 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 607 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 64 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 260 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 261 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 363 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 411 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 472 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 496 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 522 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 603 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 626 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 730 | 104.17.181.90 | 104.17.181.90 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 76 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 157 | freeyx.cloudflare88.eu.org | 141.101.121.145 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 319 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 493 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 525 | www.7749tv.com | 104.25.196.30 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 540 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 542 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 717 | 162.159.36.94 | 162.159.36.94 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 35 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 139 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 144 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 199 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 272 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 343 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 358 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 473 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 488 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 523 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 527 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 553 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 559 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 673 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 686 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 735 | 104.18.92.250 | 104.18.92.250 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 736 | 104.17.221.233 | 104.17.221.233 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 762 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 52 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 101 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 164 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 202 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 210 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 346 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 414 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 438 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 450 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 545 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 587 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 133 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 177 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 234 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 240 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 313 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 470 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 503 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 535 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 40 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 46 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 206 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 430 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 627 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 628 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 723 | 104.26.3.200 | 104.26.3.200 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 728 | 104.16.155.40 | 104.16.155.40 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 121 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 193 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 312 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 388 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 449 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 591 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 662 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 689 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 759 | 162.159.45.140 | 162.159.45.140 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 15 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 79 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 103 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 109 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 178 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 283 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 416 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 454 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 550 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 611 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 667 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 89 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 176 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 217 | cloudflare-ip.mofashi.ltd | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 610 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 612 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 710 | 108.162.198.183 | 108.162.198.183 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 90 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 226 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 267 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 296 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 310 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 360 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 393 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 415 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 548 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 567 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 682 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 707 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 45 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 77 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 182 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 187 | bestcf.030101.xyz | 104.18.40.176 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 387 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 437 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 590 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 720 | 104.20.27.188 | 104.20.27.188 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 27 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 56 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 198 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 335 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 562 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 150 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 200 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 274 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 407 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 529 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 677 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 737 | 108.162.198.146 | 108.162.198.146 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 41 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 266 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 337 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 355 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 588 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 743 | 162.159.39.194 | 162.159.39.194 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 3 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 161 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 366 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 436 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 688 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 726 | 104.26.15.207 | 104.26.15.207 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 222 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 276 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 336 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 481 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 531 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 554 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 719 | 104.26.5.25 | 104.26.5.25 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 50 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 55 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 163 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 213 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 216 | cloudflare-ip.mofashi.ltd | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 247 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 259 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 317 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 678 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 169 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 201 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 338 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 354 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 431 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 486 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 617 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 618 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 708 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 26 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 386 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 552 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 80 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 116 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 212 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 268 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 371 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 499 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 506 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 521 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 652 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 722 | 104.26.10.159 | 104.26.10.159 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 124 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 237 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 318 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 325 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 413 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 558 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 570 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 614 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 690 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 713 | 162.159.39.95 | 162.159.39.95 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 6 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 158 | freeyx.cloudflare88.eu.org | 141.101.120.120 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 168 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 352 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 530 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 714 | 162.159.38.179 | 162.159.38.179 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 44 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 98 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 115 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 383 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 426 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 606 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 615 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 630 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 646 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 692 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 761 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 419 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 423 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 446 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 494 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 541 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 660 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 738 | 104.17.57.66 | 104.17.57.66 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 129 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 277 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 453 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 520 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 571 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 622 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 629 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 693 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 695 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 768 | 172.64.52.204 | 172.64.52.204 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 145 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 170 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 302 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 443 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 491 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 538 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 661 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 725 | 104.26.8.60 | 104.26.8.60 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 117 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 227 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 448 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 524 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 648 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 657 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 95 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 359 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 422 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 592 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 623 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 655 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 60 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 248 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 427 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 574 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 616 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 633 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 727 | 172.67.73.198 | 172.67.73.198 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 69 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 143 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 294 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 377 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 51 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 106 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 286 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 399 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 597 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 608 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 621 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 127 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 135 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 153 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 246 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 435 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 33 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 49 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 59 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 107 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 258 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 351 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 439 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 561 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 766 | 162.159.44.249 | 162.159.44.249 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 2 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 273 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 455 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 474 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 598 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 758 | 172.64.229.3 | 172.64.229.3 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 39 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 118 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 589 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 242 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 398 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 479 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 9 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 595 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 620 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 671 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 122 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 152 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 484 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 712 | 162.159.45.138 | 162.159.45.138 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 347 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 515 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 593 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 599 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 663 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 5 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 10 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 429 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 532 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 560 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 651 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 706 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 110 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 428 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 485 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 732 | 172.64.158.41 | 172.64.158.41 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 295 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 311 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 569 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 605 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 691 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 290 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 664 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 11 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 420 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 85 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 160 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 305 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 34 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 197 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 602 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 658 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 489 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 191 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 220 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 534 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 20 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 25 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 183 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 289 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 408 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 28 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 409 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 501 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 760 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 221 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 81 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 228 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 278 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 392 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 675 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 613 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 487 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 147 | cloudflare |
| 647 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 476 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 68 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 153 | cloudflare |
| 421 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 252 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare |
| 397 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare |
| 253 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 373 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 379 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 666 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 619 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare |
| 67 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 745 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare |
| 634 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 171 | cloudflare |
| 241 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 125 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 174 | cloudflare |
| 262 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 176 | cloudflare |
| 19 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 178 | cloudflare |
| 369 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 287 | cloudflare |
| 769 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 465 | cloudflare |
| 746 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 491 | cloudflare |
| 235 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 1730 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 278 条记录
- **正常 (100-200ms)**: 258 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 1 条记录


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
