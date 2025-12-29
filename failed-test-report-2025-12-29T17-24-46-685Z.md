# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:24:46
- **数据来源**: connectivity_results-20251229-172446.json
- **总测试数**: 501
- **失败测试数**: 180
- **成功测试数**: 321
- **失败率**: 35.93%
- **平均延迟**: 114.45ms
- **最小延迟**: 50ms
- **最大延迟**: 1520ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:24:46
- **IP地址**: 68.154.38.24
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

- **网络不可达: 网络不可达**: 177 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (177 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 1 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable |
| 8 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 9 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 15 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 16 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 20 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 21 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 36 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 37 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 38 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 39 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 40 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 56 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 57 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 58 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable |
| 59 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 60 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable |
| 61 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable |
| 62 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 63 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable |
| 64 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 82 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 83 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 84 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 92 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 93 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 94 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 97 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 98 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 100 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 103 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 104 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 109 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 110 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 114 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 115 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 122 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 123 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 124 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 151 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 155 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 156 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 157 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 160 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 161 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 173 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 174 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 175 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 179 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 180 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 181 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 196 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 197 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 203 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 204 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 205 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 208 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:fb:e00f:f254:e667 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:fb:e00f:f254:e667]:443: connect: network is unreachable |
| 215 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 216 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 217 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 221 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 222 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 223 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 226 | zread.ai | 2606:4700:130:436c:6f75:6466:6c61:7265 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:130:436c:6f75:6466:6c61:7265]:443: connect: network is unreachable |
| 230 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 231 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 232 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 236 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 237 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 238 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 241 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 242 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 248 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 249 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 250 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 254 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 258 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 259 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 260 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 263 | bestcf.030101.xyz | 2606:4700:0:c5:4803:8845:8bde:1897 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:c5:4803:8845:8bde:1897]:443: connect: network is unreachable |
| 264 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:e7ac:854f:c15c:d3b1:fc6a]:443: connect: network is unreachable |
| 267 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 268 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 271 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 272 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 276 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 277 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 278 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 279 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 283 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable |
| 284 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable |
| 291 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 292 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 293 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 297 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 298 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 299 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 302 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 303 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 307 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 308 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 309 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 313 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 314 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 315 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 322 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 323 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 324 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 328 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 329 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 330 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 337 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 338 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 344 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 345 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 346 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 350 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 351 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 352 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 356 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 357 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 358 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 361 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 362 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 365 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable |
| 366 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable |
| 368 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 369 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 375 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 378 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 379 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 384 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 385 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 386 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 390 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 391 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 392 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 395 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 399 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 400 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 401 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 405 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 406 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 411 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 412 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 415 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 416 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 420 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 421 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 424 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 425 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 427 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 432 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 433 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 434 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 436 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 440 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 441 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 442 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 449 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 454 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 455 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 456 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 460 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 461 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 462 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 466 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 467 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 472 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 473 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 474 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 479 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 480 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 481 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
| 490 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 491 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 492 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 496 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 499 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
| 501 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 177 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 180 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 177 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), dnschecker.org (3次), huxley.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 163 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 265 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 409 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 22 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 408 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 46 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 275 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 167 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 360 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 191 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 310 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 435 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 138 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 186 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 286 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 397 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 185 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 342 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 261 | bestcf.030101.xyz | 104.17.99.183 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 476 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 318 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 343 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 164 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 168 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 333 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 341 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 11 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 146 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 316 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 468 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 266 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 325 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 353 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 376 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 243 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 327 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 402 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 418 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 213 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 165 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 144 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 374 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 469 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 111 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 154 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 200 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 354 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 71 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 209 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 419 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 377 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 106 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 141 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 332 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 349 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 367 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 102 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 380 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 446 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 483 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 136 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 187 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 280 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 6 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 7 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 256 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 478 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 192 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 457 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 35 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 34 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 257 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 465 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 43 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 45 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 49 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 130 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 188 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 394 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 4 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 80 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 101 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 340 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 413 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 52 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 79 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 269 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 339 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 166 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 487 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 42 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 116 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 235 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 437 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 19 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 50 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 143 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 262 | bestcf.030101.xyz | 104.17.27.231 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 285 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 206 | freeyx.cloudflare88.eu.org | 141.101.121.146 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 251 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 273 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 120 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 176 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 184 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 301 | cf.090227.xyz | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 404 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 497 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 75 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 76 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 129 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 134 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 140 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 245 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 252 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 363 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 13 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 51 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 53 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 119 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 464 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 304 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 26 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 31 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 85 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 239 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 244 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 305 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 306 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 96 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 121 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 126 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 201 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 450 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 14 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 135 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 193 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 212 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 224 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 240 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 370 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 68 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 295 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 423 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare |
| 5 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 77 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 108 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 183 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 489 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 32 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 70 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 348 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare |
| 47 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 169 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 335 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 414 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 27 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 99 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 117 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 128 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 270 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 445 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 494 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare |
| 10 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 81 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 383 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 431 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 463 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 498 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 17 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 86 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 158 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 199 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 300 | cf.090227.xyz | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 393 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 198 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 229 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 444 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 470 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare |
| 66 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 87 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 211 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 228 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 326 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 336 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 387 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare |
| 44 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 145 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 172 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 177 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 189 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 220 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 282 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 486 | www.7749tv.com | 104.17.196.161 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare |
| 30 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 132 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 359 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |
| 55 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 118 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 162 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 122 条记录
- **正常 (100-200ms)**: 78 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 177 次

### 按协议统计

- **none**: 180 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
