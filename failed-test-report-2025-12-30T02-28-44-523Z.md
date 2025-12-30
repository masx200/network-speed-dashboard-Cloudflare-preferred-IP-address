# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:28:44
- **数据来源**: connectivity_results-20251230-022843.json
- **总测试数**: 590
- **失败测试数**: 191
- **成功测试数**: 399
- **失败率**: 32.37%
- **平均延迟**: 107.39ms
- **最小延迟**: 39ms
- **最大延迟**: 519ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:28:44
- **IP地址**: 64.236.145.64
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

- **网络不可达: 网络不可达**: 188 次 (98.4%)
- **连接超时: I/O超时**: 3 次 (1.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (188 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 29 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 30 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 38 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 39 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 40 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 47 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 48 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 49 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 50 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 51 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 52 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 53 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 54 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 55 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 56 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 79 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 80 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 81 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 82 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 83 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 102 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 103 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 108 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 109 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 113 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 114 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 115 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 116 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 117 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 118 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 119 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 120 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 121 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 122 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 179 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 180 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 181 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 210 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 211 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 215 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 216 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 217 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 223 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 224 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 225 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 229 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 230 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 231 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 235 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 236 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 237 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 241 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 242 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 243 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 247 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 248 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 249 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 252 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 255 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 256 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 262 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cd47:f6a2:e5d6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa59:4b67:cd47:f6a2:e5d6]:443: connect: network is unreachable |
| 265 | bestcf.030101.xyz | 2606:4700:0:7809:f54e:ab56:90d6:b7ab | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:7809:f54e:ab56:90d6:b7ab]:443: connect: network is unreachable |
| 266 | bestcf.030101.xyz | 2606:4700:0:7809:f54e:bd32:7275:fc25 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:7809:f54e:bd32:7275:fc25]:443: connect: network is unreachable |
| 270 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 271 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 272 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 280 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 281 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 282 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 286 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 287 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 288 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 289 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 292 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 293 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 297 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 298 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 299 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 305 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 306 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 311 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 312 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 313 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 316 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 317 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 321 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 322 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 323 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 326 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable |
| 327 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable |
| 331 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 332 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 333 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 337 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 338 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 339 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 344 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 345 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 349 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 350 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 351 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 354 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 355 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 359 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 360 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 361 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 364 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 368 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 369 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 370 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 373 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 379 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 380 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 381 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 383 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 387 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 388 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 389 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 394 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 398 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 399 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 400 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 403 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 404 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 407 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 408 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 409 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 412 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 413 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 424 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 425 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 426 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 430 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 431 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 435 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 436 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 437 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 442 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 443 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 444 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 448 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 449 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 450 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 454 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 455 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 456 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 458 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 466 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 467 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 468 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 474 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 475 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 481 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 482 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 488 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 489 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 490 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 492 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 493 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 496 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 497 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 504 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 505 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 508 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 509 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 519 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 520 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 521 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 525 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 526 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 531 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 532 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 533 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 546 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 549 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 550 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 554 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 555 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 556 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 560 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 561 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 562 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 563 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 567 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 568 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 572 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 573 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 574 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 543 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 589 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 590 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 188 次 (98.4%)
- **连接超时**: 3 次 (1.6%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 191 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 188 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 399 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 511 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 366 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 291 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 37 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 340 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 214 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 479 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 163 | 108.162.196.137 | 108.162.196.137 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 390 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 411 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 91 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 296 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 544 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 378 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 406 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 422 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 12 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 391 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 524 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 112 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 173 | 172.64.48.164 | 172.64.48.164 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 438 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 527 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 301 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 308 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 309 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 362 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 428 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 551 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 166 | 172.64.53.139 | 172.64.53.139 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 502 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 477 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 385 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 372 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 584 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 64 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 371 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 457 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 228 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 19 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 94 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 100 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 200 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 290 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 402 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 139 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 191 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 365 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 483 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 498 | www.7749tv.com | 104.16.115.36 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 522 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 87 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 202 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 357 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 35 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 453 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 23 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 32 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 59 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 347 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 393 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 569 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 20 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 127 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 304 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 319 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 476 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 27 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 3 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 150 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 325 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 518 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 4 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 140 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 307 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 315 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 348 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 395 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 577 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 5 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 77 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 261 | freeyx.cloudflare88.eu.org | 141.101.121.253 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 541 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 99 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 154 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 285 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 314 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 45 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 257 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 418 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 61 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 244 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 250 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 353 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 513 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 515 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 157 | 162.159.19.204 | 162.159.19.204 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 420 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 429 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 462 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 538 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 66 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 95 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 137 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 145 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 277 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 465 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 10 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 65 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 172 | 162.159.16.71 | 162.159.16.71 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 189 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 201 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 276 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 440 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 542 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 44 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 553 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 84 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 416 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 535 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 222 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 356 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 445 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 575 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 160 | 172.64.49.198 | 172.64.49.198 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 382 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 459 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 512 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 534 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 571 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 15 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 209 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 396 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 421 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 523 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 548 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 21 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 26 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 43 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 76 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 89 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 374 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 470 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 506 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 63 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 104 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 187 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 263 | bestcf.030101.xyz | 104.17.55.198 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 300 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 432 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 143 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 196 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 239 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 461 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 578 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 88 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 111 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 138 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 375 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 478 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 557 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 18 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 75 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 141 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 188 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 335 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 336 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 341 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 401 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 485 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 13 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 170 | 108.162.198.104 | 108.162.198.104 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 184 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 251 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 570 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 22 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 60 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 268 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 274 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 414 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 581 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 17 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 57 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 92 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 186 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 221 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 233 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 484 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 552 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 93 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 144 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 174 | 162.159.44.8 | 162.159.44.8 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 176 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 238 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 464 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 472 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 566 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 71 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 151 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 183 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 185 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 195 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 302 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 434 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 439 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 446 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 486 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 528 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 11 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 156 | 162.159.40.228 | 162.159.40.228 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 165 | 162.159.48.95 | 162.159.48.95 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 194 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 329 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 427 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 463 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 25 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 98 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 503 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 559 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 7 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 41 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 62 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 107 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 218 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 269 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 324 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 352 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 419 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 46 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 73 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 105 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 168 | 162.159.7.116 | 162.159.7.116 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 283 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 284 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 334 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 415 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 68 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 96 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 212 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 397 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 423 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 85 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 245 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 433 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 495 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 514 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 529 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 583 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 586 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 33 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 72 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 133 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 204 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 279 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 487 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 579 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 9 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 24 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 101 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 135 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 177 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 192 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 343 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 363 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 460 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 1 | 172.64.52.136 | 172.64.52.136 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 69 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 136 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 149 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 260 | freeyx.cloudflare88.eu.org | 141.101.120.101 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 494 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 501 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 530 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 565 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 130 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 152 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 162 | 104.18.45.32 | 104.18.45.32 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 219 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 258 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 310 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 499 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 582 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 16 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 318 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 367 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 42 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 131 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 199 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 564 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 14 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 178 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 303 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 516 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 536 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 34 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 410 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 500 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 576 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 227 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 142 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 155 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 6 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 134 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 346 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 507 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 86 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 480 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 126 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 164 | 108.162.193.41 | 108.162.193.41 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 197 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 264 | bestcf.030101.xyz | 104.17.60.95 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 358 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 125 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 246 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 2 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 330 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 473 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 110 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 259 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 78 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 208 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 537 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 220 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 31 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 153 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 294 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 123 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 213 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 193 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 384 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 129 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 132 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 182 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 469 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 8 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 70 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 128 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 161 | 173.245.59.46 | 173.245.59.46 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 254 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 74 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 328 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 540 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 234 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 386 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 167 | 162.159.45.96 | 162.159.45.96 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 171 | 162.159.38.49 | 162.159.38.49 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 376 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 441 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 253 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 392 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 159 | 162.159.62.214 | 162.159.62.214 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 58 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 278 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 539 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 320 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 491 | 162.159.39.15 | 162.159.39.15 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 517 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 147 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 267 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 275 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 295 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 144 | cloudflare |
| 28 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 198 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 148 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 447 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 342 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 146 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 405 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 152 | cloudflare |
| 451 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 155 | cloudflare |
| 97 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 156 | cloudflare |
| 169 | 162.159.58.5 | 162.159.58.5 | IPv4 | h3 | ✅ 成功 | 157 | cloudflare |
| 158 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 106 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 226 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 90 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 124 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 180 | cloudflare |
| 175 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 180 | cloudflare |
| 67 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare |
| 452 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 188 | cloudflare |
| 545 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 190 | cloudflare |
| 377 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 200 | cloudflare |
| 240 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare |
| 273 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare |
| 190 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 212 | cloudflare |
| 585 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 214 | cloudflare |
| 547 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare |
| 232 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 220 | cloudflare |
| 558 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 238 | cloudflare |
| 36 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 268 | cloudflare |
| 510 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 299 | cloudflare |
| 471 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 351 | cloudflare |
| 417 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 356 | cloudflare |
| 580 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 498 | cloudflare |
| 588 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 516 | cloudflare |
| 587 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 519 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 10 条记录
- **快 (50-100ms)**: 180 条记录
- **正常 (100-200ms)**: 194 条记录
- **慢 (200-500ms)**: 13 条记录
- **很慢 (>500ms)**: 2 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 188 次

### 按协议统计

- **none**: 191 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
