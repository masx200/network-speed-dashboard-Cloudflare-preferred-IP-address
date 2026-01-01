# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/1 21:43:11
- **数据来源**: connectivity_results-20260101-214310.json
- **总测试数**: 723
- **失败测试数**: 221
- **成功测试数**: 502
- **失败率**: 30.57%
- **平均延迟**: 100.43ms
- **最小延迟**: 42ms
- **最大延迟**: 534ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/1 21:43:11
- **IP地址**: 135.232.201.224
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

- **网络不可达: 网络不可达**: 218 次 (98.6%)
- **连接超时: I/O超时**: 3 次 (1.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (218 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 5 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 6 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 9 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 10 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 13 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 14 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 19 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 20 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 21 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 24 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 35 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 38 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 39 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 42 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 43 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 48 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 49 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 50 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 54 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 55 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 56 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 60 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 61 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 62 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 65 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 71 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 72 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 73 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 74 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 75 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 79 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 80 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 93 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 94 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 95 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 105 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 106 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 155 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 156 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 158 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 159 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 160 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 161 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 162 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 163 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 164 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 165 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 207 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 208 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 209 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 210 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 211 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 212 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 213 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 215 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 216 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 217 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 255 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 256 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 263 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 264 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 265 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 268 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 271 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 272 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 275 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 276 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 287 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 288 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 289 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 292 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 293 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 299 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 300 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 301 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 304 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 305 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 306 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 310 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 311 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 312 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 321 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 322 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 323 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 325 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 332 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 333 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 334 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 338 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 339 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 340 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 343 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 344 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 345 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 352 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 355 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 356 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 360 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 361 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 362 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 366 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 367 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 368 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 369 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 373 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 374 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 379 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 380 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 383 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 389 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 390 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 393 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 394 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 398 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 399 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 400 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 404 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 405 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 406 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 410 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 411 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 412 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 419 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 420 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 429 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 430 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 431 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 435 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 436 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 437 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 441 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 442 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 446 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 447 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 448 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 452 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 453 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 457 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 458 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 459 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 463 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 464 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 465 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 469 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 470 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 471 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 473 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 475 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 481 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 482 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 483 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 487 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 488 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 492 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 493 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 494 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 501 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 502 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 503 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 511 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 512 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 513 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 514 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 515 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 516 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 517 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 532 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 533 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 534 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 543 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa73:82e3:b098:2d2c:5ebd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3009:aa73:82e3:b098:2d2c:5ebd]:443: connect: network is unreachable |
| 559 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 560 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 561 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 573 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 574 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 575 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 578 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 581 | bestcf.030101.xyz | 2606:4700::ba:70f2:6c0c:d9d5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::ba:70f2:6c0c:d9d5]:443: connect: network is unreachable |
| 582 | bestcf.030101.xyz | 2606:4700:0:fdd3:e8b9:88ce:79cf:94fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:fdd3:e8b9:88ce:79cf:94fc]:443: connect: network is unreachable |
| 585 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 586 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 587 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 591 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 592 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 593 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 604 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 605 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 606 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 640 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 641 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 645 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 646 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 647 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 648 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 649 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 650 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 651 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 652 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 653 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 654 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 678 | 2a06:98c1:3100:0:60e2:56b1:6eda:c6ea | 2a06:98c1:3100:0:60e2:56b1:6eda:c6ea | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:60e2:56b1:6eda:c6ea]:443: connect: network is unreachable |
| 682 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 683 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 684 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 696 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 697 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 703 | 2a06:98c1:50:0:d9:7833:1957:3fc | 2a06:98c1:50:0:d9:7833:1957:3fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:0:d9:7833:1957:3fc]:443: connect: network is unreachable |
| 704 | 2a06:98c1:51:b3:4cf9:6626:e825:c1f | 2a06:98c1:51:b3:4cf9:6626:e825:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:b3:4cf9:6626:e825:c1f]:443: connect: network is unreachable |
| 705 | 2803:f800:50:28a5:72ab:6ef3:5155:dd72 | 2803:f800:50:28a5:72ab:6ef3:5155:dd72 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:28a5:72ab:6ef3:5155:dd72]:443: connect: network is unreachable |
| 706 | 2a06:98c1:3102:0:9825:9875:90b7:a534 | 2a06:98c1:3102:0:9825:9875:90b7:a534 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:9825:9875:90b7:a534]:443: connect: network is unreachable |
| 707 | 2803:f800:50:97d1:dc8a:8fcb:203c:fae | 2803:f800:50:97d1:dc8a:8fcb:203c:fae | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:97d1:dc8a:8fcb:203c:fae]:443: connect: network is unreachable |
| 708 | 2a06:98c1:3105:6cc4:39a7:c497:41bb:be15 | 2a06:98c1:3105:6cc4:39a7:c497:41bb:be15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:6cc4:39a7:c497:41bb:be15]:443: connect: network is unreachable |
| 709 | 2803:f800:51:0:3:2a1d:b1e8:5d9e | 2803:f800:51:0:3:2a1d:b1e8:5d9e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:3:2a1d:b1e8:5d9e]:443: connect: network is unreachable |
| 710 | 2a06:98c1:310f:c0:8a19:7249:907f:2272 | 2a06:98c1:310f:c0:8a19:7249:907f:2272 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:c0:8a19:7249:907f:2272]:443: connect: network is unreachable |
| 711 | 2a06:98c1:310e:3e59:a058:1508:400f:78b5 | 2a06:98c1:310e:3e59:a058:1508:400f:78b5 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:3e59:a058:1508:400f:78b5]:443: connect: network is unreachable |
| 721 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 722 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 424 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 659 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 723 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 218 次 (98.6%)
- **连接超时**: 3 次 (1.4%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 221 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 218 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), wilson.ns.cloudflare.com (3次), japan.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 502 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 377 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 273 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 290 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 183 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 416 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 316 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 600 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 692 | 104.18.40.146 | 104.18.40.146 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 77 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 395 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 519 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 603 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 101 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 291 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 687 | 104.19.40.149 | 104.19.40.149 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 699 | 162.159.20.118 | 162.159.20.118 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 719 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 371 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 686 | 104.18.46.2 | 104.18.46.2 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 245 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 425 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 450 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 460 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 281 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 324 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 712 | 172.64.229.99 | 172.64.229.99 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 196 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 497 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 548 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 123 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 168 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 270 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 278 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 327 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 454 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 596 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 661 | 104.18.38.6 | 104.18.38.6 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 314 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 455 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 677 | 104.20.25.112 | 104.20.25.112 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 63 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 219 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 428 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 598 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 628 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 146 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 624 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 232 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 357 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 413 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 119 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 127 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 178 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 583 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 154 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 182 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 222 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 642 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 666 | 162.159.24.112 | 162.159.24.112 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 672 | 172.67.75.98 | 172.67.75.98 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 121 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 151 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 551 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 639 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 97 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 528 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 608 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 673 | 104.26.10.222 | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 280 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 309 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 351 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 537 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 11 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 570 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 669 | 172.67.69.91 | 172.67.69.91 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 64 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 90 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 602 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 623 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 694 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 670 | 172.67.65.105 | 172.67.65.105 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 103 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 221 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 391 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 115 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 246 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 260 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 302 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 376 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 599 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 15 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 128 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 467 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 34 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 78 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 353 | cf.090227.xyz | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 139 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 179 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 228 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 350 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 358 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 675 | 172.67.73.113 | 172.67.73.113 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 109 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 177 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 189 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 240 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 347 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 509 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 660 | 162.159.1.158 | 162.159.1.158 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 67 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 185 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 186 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 234 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 326 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 382 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 536 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 541 | freeyx.cloudflare88.eu.org | 141.101.121.37 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 595 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 612 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 188 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 242 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 298 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 341 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 486 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 552 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 613 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 663 | 162.159.33.230 | 162.159.33.230 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 69 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 87 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 170 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 173 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 303 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 401 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 629 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 32 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 51 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 68 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 111 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 114 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 124 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 235 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 563 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 681 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 33 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 110 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 126 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 201 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 676 | 104.26.6.217 | 104.26.6.217 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 679 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 4 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 135 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 200 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 237 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 279 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 315 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 451 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 522 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 577 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 31 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 144 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 346 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 504 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 544 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 644 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 83 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 107 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 166 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 192 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 372 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 542 | freeyx.cloudflare88.eu.org | 141.101.120.101 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 576 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 665 | 162.159.6.173 | 162.159.6.173 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 57 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 81 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 85 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 91 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 120 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 140 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 142 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 313 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 378 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 388 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 611 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 626 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 136 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 137 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 248 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 277 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 433 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 693 | 104.17.56.81 | 104.17.56.81 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 715 | 172.64.53.60 | 172.64.53.60 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 17 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 29 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 41 | comicabc.com | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 113 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 134 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 141 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 167 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 230 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 249 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 258 | www.7749tv.com | 104.19.114.190 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 295 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 414 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 423 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 432 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 440 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 521 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 564 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 565 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 580 | bestcf.030101.xyz | 172.64.155.243 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 630 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 633 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 720 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 152 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 231 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 252 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 257 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 285 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 427 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 620 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 36 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 116 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 130 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 180 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 181 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 197 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 253 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 320 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 550 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 572 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 22 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 44 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 47 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 53 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 102 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 132 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 421 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 456 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 535 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 16 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 254 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 259 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 700 | 172.64.52.0 | 172.64.52.0 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 12 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 112 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 553 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 615 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 27 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 392 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 545 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 547 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 610 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 634 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 171 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 269 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 359 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 479 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 524 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 601 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 689 | 104.17.168.35 | 104.17.168.35 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 691 | 104.17.159.241 | 104.17.159.241 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 138 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 439 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 472 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 508 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 525 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 713 | 108.162.198.90 | 108.162.198.90 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 84 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 187 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 194 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 229 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 365 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 499 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 507 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 510 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 674 | 104.26.12.204 | 104.26.12.204 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 701 | 162.159.39.149 | 162.159.39.149 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 3 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 108 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 125 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 417 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 506 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 530 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 617 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 680 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 86 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 100 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 296 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 330 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 195 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 238 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 282 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 422 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 443 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 498 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 668 | 173.245.58.165 | 173.245.58.165 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 7 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 37 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 145 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 385 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 554 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 627 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 157 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 184 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 198 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 202 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 520 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 531 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 631 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 133 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 239 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 588 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 636 | 162.159.46.211 | 162.159.46.211 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 76 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 150 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 206 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 247 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 415 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 622 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 23 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 98 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 118 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 402 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 26 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 148 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 224 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 251 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 375 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 526 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 579 | bestcf.030101.xyz | 104.17.155.61 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 614 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 621 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 698 | 104.19.59.26 | 104.19.59.26 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 717 | 162.159.44.213 | 162.159.44.213 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 131 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 261 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 317 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 337 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 364 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 418 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 476 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 496 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 618 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 656 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 671 | 162.159.9.88 | 162.159.9.88 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 319 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 329 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 381 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 407 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 523 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 635 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 655 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 716 | 172.64.41.135 | 172.64.41.135 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 241 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 266 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 307 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 485 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 99 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 147 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 571 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 45 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 175 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 193 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 205 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 225 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 262 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 335 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 354 | cf.090227.xyz | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 387 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 445 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 584 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 149 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 477 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 637 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 176 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 267 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 286 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 434 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 28 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 174 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 243 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 348 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 384 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 461 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 557 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 480 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 555 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 568 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 685 | 104.17.182.156 | 104.17.182.156 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 690 | 104.19.43.96 | 104.19.43.96 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 695 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 8 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 52 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 199 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 233 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 349 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 462 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 597 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 662 | 104.20.22.198 | 104.20.22.198 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 342 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 527 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 590 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare |
| 191 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 236 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 540 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 117 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 449 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 466 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 562 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 616 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 657 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare |
| 2 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 96 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 129 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 426 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 539 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 556 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 589 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 397 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 538 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 632 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 664 | 162.159.25.69 | 162.159.25.69 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare |
| 153 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 214 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 609 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 702 | 172.64.154.89 | 172.64.154.89 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 331 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 566 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 123 | cloudflare |
| 567 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare |
| 220 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 403 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 500 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 125 | cloudflare |
| 40 | comicabc.com | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 126 | cloudflare |
| 226 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 468 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 625 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 89 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 518 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 129 | cloudflare |
| 244 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 505 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 130 | cloudflare |
| 88 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 122 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 131 | cloudflare |
| 529 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 546 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare |
| 558 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare |
| 638 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare |
| 203 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 489 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 688 | 104.26.0.42 | 104.26.0.42 | IPv4 | h3 | ✅ 成功 | 135 | cloudflare |
| 190 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare |
| 25 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 409 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 396 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare |
| 386 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 718 | 162.159.45.222 | 162.159.45.222 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 218 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 223 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare |
| 169 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 444 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 142 | cloudflare |
| 438 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 484 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 569 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 143 | cloudflare |
| 308 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 145 | cloudflare |
| 204 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 607 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare |
| 18 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 172 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 294 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 148 | cloudflare |
| 478 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 149 | cloudflare |
| 92 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 474 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare |
| 284 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 297 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 250 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 154 | cloudflare |
| 336 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 159 | cloudflare |
| 714 | 162.159.38.167 | 162.159.38.167 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare |
| 46 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare |
| 58 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 658 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 165 | cloudflare |
| 30 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 166 | cloudflare |
| 227 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 167 | cloudflare |
| 59 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 318 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 168 | cloudflare |
| 363 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 178 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 491 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 182 | cloudflare |
| 549 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 143 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 191 | cloudflare |
| 70 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 208 | cloudflare |
| 643 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 213 | cloudflare |
| 490 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare |
| 82 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 235 | cloudflare |
| 594 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 238 | cloudflare |
| 667 | 162.159.48.186 | 162.159.48.186 | IPv4 | h3 | ✅ 成功 | 245 | cloudflare |
| 408 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 251 | cloudflare |
| 619 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 255 | cloudflare |
| 274 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 269 | cloudflare |
| 104 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 273 | cloudflare |
| 495 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 342 | cloudflare |
| 370 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 487 | cloudflare |
| 283 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 519 | cloudflare |
| 328 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 534 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 23 条记录
- **快 (50-100ms)**: 273 条记录
- **正常 (100-200ms)**: 192 条记录
- **慢 (200-500ms)**: 12 条记录
- **很慢 (>500ms)**: 2 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 218 次

### 按协议统计

- **none**: 221 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
