# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/27 02:37:31
- **数据来源**: connectivity_results-20260227-023731.json
- **总测试数**: 1004
- **失败测试数**: 272
- **成功测试数**: 732
- **失败率**: 27.09%
- **平均延迟**: 53.63ms
- **最小延迟**: 23ms
- **最大延迟**: 985ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/27 02:37:31
- **IP地址**: 172.184.211.36
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
| 1 | 2a06:98c1:51:db13:f0d1:44cc:d4a3:582c | 2a06:98c1:51:db13:f0d1:44cc:d4a3:582c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:db13:f0d1:44cc:d4a3:582c]:443: connect: network is unreachable |
| 9 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable |
| 14 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable |
| 15 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable |
| 18 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable |
| 19 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable |
| 24 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable |
| 25 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable |
| 26 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable |
| 29 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable |
| 30 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable |
| 34 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable |
| 35 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable |
| 36 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable |
| 40 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable |
| 41 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable |
| 42 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable |
| 43 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable |
| 51 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable |
| 52 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable |
| 53 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable |
| 54 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable |
| 55 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable |
| 59 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable |
| 60 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable |
| 61 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable |
| 67 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable |
| 68 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable |
| 78 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable |
| 79 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable |
| 80 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable |
| 83 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable |
| 84 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable |
| 90 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable |
| 91 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable |
| 92 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable |
| 98 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable |
| 99 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable |
| 100 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable |
| 107 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable |
| 108 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable |
| 109 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable |
| 113 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:a7e7:26a1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3010:0:f5e8:7af2:a7e7:26a1]:443: connect: network is unreachable |
| 143 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable |
| 144 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable |
| 145 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable |
| 149 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable |
| 150 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable |
| 151 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable |
| 155 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable |
| 156 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable |
| 157 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable |
| 161 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable |
| 162 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable |
| 163 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable |
| 166 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable |
| 167 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable |
| 171 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable |
| 172 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable |
| 173 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable |
| 176 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable |
| 177 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable |
| 180 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable |
| 181 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable |
| 184 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable |
| 185 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable |
| 189 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable |
| 190 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable |
| 201 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable |
| 202 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable |
| 211 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable |
| 212 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable |
| 213 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable |
| 216 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable |
| 217 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable |
| 218 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable |
| 221 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:d9:2acf:b5e0:5a46:4358]:443: connect: network is unreachable |
| 222 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:0:dd:df95:6eb1:ffa4:6779]:443: connect: network is unreachable |
| 228 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 229 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 232 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable |
| 233 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable |
| 235 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable |
| 239 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable |
| 240 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable |
| 241 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable |
| 244 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable |
| 245 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable |
| 250 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable |
| 251 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable |
| 255 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable |
| 256 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable |
| 257 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable |
| 261 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable |
| 262 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable |
| 263 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable |
| 267 | time.is | 2606:4700:20::681a:c36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable |
| 268 | time.is | 2606:4700:20::681a:d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable |
| 269 | time.is | 2606:4700:20::ac43:449d | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable |
| 276 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable |
| 277 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable |
| 278 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable |
| 283 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable |
| 284 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable |
| 285 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable |
| 291 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable |
| 292 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable |
| 293 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable |
| 298 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable |
| 299 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable |
| 303 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable |
| 304 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable |
| 305 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable |
| 309 | ip.gs | 2a06:98c1:3120::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable |
| 310 | ip.gs | 2a06:98c1:3121::3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable |
| 314 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable |
| 315 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable |
| 316 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable |
| 319 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable |
| 320 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable |
| 324 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable |
| 325 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable |
| 326 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable |
| 328 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable |
| 332 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable |
| 333 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable |
| 334 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable |
| 336 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable |
| 341 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable |
| 342 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable |
| 343 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable |
| 348 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable |
| 351 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 352 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 355 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable |
| 356 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable |
| 359 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable |
| 360 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable |
| 361 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable |
| 366 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable |
| 367 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable |
| 368 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable |
| 371 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable |
| 372 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable |
| 377 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable |
| 378 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable |
| 379 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable |
| 385 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable |
| 386 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable |
| 387 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable |
| 391 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable |
| 395 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable |
| 396 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable |
| 397 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable |
| 400 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable |
| 412 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable |
| 413 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable |
| 414 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable |
| 421 | japan.com | 2606:4700:20::681a:43c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable |
| 422 | japan.com | 2606:4700:20::681a:53c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable |
| 423 | japan.com | 2606:4700:20::ac43:465c | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable |
| 425 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable |
| 434 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable |
| 435 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable |
| 439 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable |
| 440 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable |
| 451 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable |
| 452 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable |
| 453 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable |
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
| 521 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable |
| 562 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 563 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable |
| 564 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable |
| 565 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable |
| 566 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable |
| 567 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable |
| 568 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable |
| 569 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 570 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable |
| 571 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable |
| 613 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable |
| 614 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 616 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable |
| 617 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable |
| 618 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 619 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable |
| 620 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable |
| 622 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable |
| 623 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable |
| 663 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3108:0:edda:98f0:da65:4271]:443: connect: network is unreachable |
| 664 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310d:85:ac4c:8137:506:5188]:443: connect: network is unreachable |
| 665 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2]:443: connect: network is unreachable |
| 666 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:fc87:e2d6:88c3:378b]:443: connect: network is unreachable |
| 667 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53]:443: connect: network is unreachable |
| 668 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:6e:e874:db4f:a1d5:2163]:443: connect: network is unreachable |
| 669 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:4f38:5221:f77f:fe11]:443: connect: network is unreachable |
| 670 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::ee:b8fb:877f]:443: connect: network is unreachable |
| 671 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3]:443: connect: network is unreachable |
| 672 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7]:443: connect: network is unreachable |
| 712 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:a594:2926:2b16:6e8d:843e]:443: connect: network is unreachable |
| 713 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:aa:3e22:dd48:6279:eeb9]:443: connect: network is unreachable |
| 714 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea]:443: connect: network is unreachable |
| 716 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:da84:1c63:f149:4d21:b339]:443: connect: network is unreachable |
| 717 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b]:443: connect: network is unreachable |
| 718 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:50:9516:e4a1:4ba9:1c5e:7533]:443: connect: network is unreachable |
| 719 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f702:ebbf:618b:76c:9ba7]:443: connect: network is unreachable |
| 720 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3107:54:2c60:eafc:f14d:ca4b]:443: connect: network is unreachable |
| 721 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106:6a:7ba4:346b:e06c:71c7]:443: connect: network is unreachable |
| 722 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104::f3:8fed:cac0]:443: connect: network is unreachable |
| 772 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable |
| 786 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable |
| 787 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable |
| 788 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable |
| 789 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable |
| 790 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable |
| 792 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 793 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable |
| 794 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable |
| 795 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable |
| 838 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | 2a06:98c1:3102:94:16cd:b988:5dae:1295 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:94:16cd:b988:5dae:1295]:443: connect: network is unreachable |
| 839 | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:ec9e:b468:412c:1558:69cb]:443: connect: network is unreachable |
| 841 | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | 2a06:98c1:3102:94:1604:ebd:f1ec:37be | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:94:1604:ebd:f1ec:37be]:443: connect: network is unreachable |
| 842 | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c]:443: connect: network is unreachable |
| 843 | 2606:4700:59:764d:d406:c823:e52f:4f3a | 2606:4700:59:764d:d406:c823:e52f:4f3a | IPv6 | none | N/A | 0 | N/A | dial tcp [2606:4700:59:764d:d406:c823:e52f:4f3a]:443: connect: network is unreachable |
| 844 | 2a06:98c1:51:8:7944:48b0:1301:5ced | 2a06:98c1:51:8:7944:48b0:1301:5ced | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:8:7944:48b0:1301:5ced]:443: connect: network is unreachable |
| 845 | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f]:443: connect: network is unreachable |
| 846 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:cfd2:7ebe:a043:8535]:443: connect: network is unreachable |
| 847 | 2a06:98c1:310b::fda8:fa9e:4a3e | 2a06:98c1:310b::fda8:fa9e:4a3e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b::fda8:fa9e:4a3e]:443: connect: network is unreachable |
| 848 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:0:e263:6cdc:a8ce:a339]:443: connect: network is unreachable |
| 883 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable |
| 890 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable |
| 891 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable |
| 892 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 893 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable |
| 894 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable |
| 895 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable |
| 896 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 898 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable |
| 899 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable |
| 915 | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3102:8768:b929:7455:f040:5aee]:443: connect: network is unreachable |
| 916 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | 2a06:98c1:310a:0:de:2b25:85a5:8a26 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:0:de:2b25:85a5:8a26]:443: connect: network is unreachable |
| 917 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | 2a06:98c1:3100:22:21ad:d760:d542:16c8 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:22:21ad:d760:d542:16c8]:443: connect: network is unreachable |
| 918 | 2a06:98c1:310c::dd:f399:427e | 2a06:98c1:310c::dd:f399:427e | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310c::dd:f399:427e]:443: connect: network is unreachable |
| 919 | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:e1e7:ae26:af07:44a6:82da]:443: connect: network is unreachable |
| 920 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3]:443: connect: network is unreachable |
| 921 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | 2a06:98c1:3100:b3:af54:9923:e84:af58 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:b3:af54:9923:e84:af58]:443: connect: network is unreachable |
| 924 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | 2a06:98c1:51:878:e123:da31:b2ee:2017 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:51:878:e123:da31:b2ee:2017]:443: connect: network is unreachable |
| 925 | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:22:21cb:7546:1cd8:a79f]:443: connect: network is unreachable |
| 926 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | 2400:cb00:2049:5d:a92a:97f:6fa3:f803 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:5d:a92a:97f:6fa3:f803]:443: connect: network is unreachable |
| 968 | 2a06:98c1:3100:f5b1:2ce9:7430:9027:4c89 | 2a06:98c1:3100:f5b1:2ce9:7430:9027:4c89 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3100:f5b1:2ce9:7430:9027:4c89]:443: connect: network is unreachable |
| 972 | 2803:f800:51:0:7c43:3bc0:435:fd7f | 2803:f800:51:0:7c43:3bc0:435:fd7f | IPv6 | none | N/A | 0 | N/A | dial tcp [2803:f800:51:0:7c43:3bc0:435:fd7f]:443: connect: network is unreachable |
| 973 | 2a06:98c1:3101:923b:5f69:671a:d74d:697b | 2a06:98c1:3101:923b:5f69:671a:d74d:697b | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3101:923b:5f69:671a:d74d:697b]:443: connect: network is unreachable |
| 974 | 2a06:98c1:310f:0:e963:a0b5:a697:c9ac | 2a06:98c1:310f:0:e963:a0b5:a697:c9ac | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310f:0:e963:a0b5:a697:c9ac]:443: connect: network is unreachable |
| 975 | 2a06:98c1:310b:0:e000:14c1:940:2095 | 2a06:98c1:310b:0:e000:14c1:940:2095 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:0:e000:14c1:940:2095]:443: connect: network is unreachable |
| 976 | 2a06:98c1:310b:c5:4f:3c80:c19e:b358 | 2a06:98c1:310b:c5:4f:3c80:c19e:b358 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310b:c5:4f:3c80:c19e:b358]:443: connect: network is unreachable |
| 977 | 2a06:98c1:3103:e8:2985:d043:e227:8130 | 2a06:98c1:3103:e8:2985:d043:e227:8130 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:3103:e8:2985:d043:e227:8130]:443: connect: network is unreachable |
| 978 | 2a06:98c1:310a:d4b7:9608:9cfe:e697:55b9 | 2a06:98c1:310a:d4b7:9608:9cfe:e697:55b9 | IPv6 | none | N/A | 0 | N/A | dial tcp [2a06:98c1:310a:d4b7:9608:9cfe:e697:55b9]:443: connect: network is unreachable |
| 979 | 2400:cb00:2049:d6f5:3f86:37d0:9155:3773 | 2400:cb00:2049:d6f5:3f86:37d0:9155:3773 | IPv6 | none | N/A | 0 | N/A | dial tcp [2400:cb00:2049:d6f5:3f86:37d0:9155:3773]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 988 | 172.67.49.134 | 172.67.49.134 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.67.49.134:443: i/o timeout |
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

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次), wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性



---

## 🚀 延迟最低的 732 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 82 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 23 | cloudflare |
| 329 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 25 | cloudflare |
| 135 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 27 | cloudflare |
| 8 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 28 | cloudflare |
| 729 | 172.67.75.212 | 172.67.75.212 | IPv4 | h3 | ✅ 成功 | 28 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 124 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 196 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 362 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 545 | 104.17.101.37 | 104.17.101.37 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 552 | 104.18.39.15 | 104.18.39.15 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 585 | 104.26.6.171 | 104.26.6.171 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 599 | 104.16.155.230 | 104.16.155.230 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 644 | 104.20.20.192 | 104.20.20.192 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 645 | 104.25.241.198 | 104.25.241.198 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 730 | 104.20.26.221 | 104.20.26.221 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 801 | 172.64.52.90 | 172.64.52.90 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 802 | 104.25.248.93 | 104.25.248.93 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 970 | 172.64.229.68 | 172.64.229.68 | IPv4 | h3 | ✅ 成功 | 29 | cloudflare |
| 39 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 58 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 69 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 132 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 444 | 104.17.139.37 | 104.17.139.37 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 450 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 481 | 162.159.22.29 | 162.159.22.29 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 507 | 104.18.44.148 | 104.18.44.148 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 514 | 172.67.68.252 | 172.67.68.252 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 532 | 172.67.68.211 | 172.67.68.211 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 557 | 172.64.53.0 | 172.64.53.0 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 558 | 172.64.50.51 | 172.64.50.51 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 600 | 104.18.44.25 | 104.18.44.25 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 631 | 172.64.229.134 | 172.64.229.134 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 650 | 104.18.172.20 | 104.18.172.20 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 691 | 104.26.4.135 | 104.26.4.135 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 744 | 104.18.160.38 | 104.18.160.38 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 753 | 104.20.21.147 | 104.20.21.147 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 780 | 104.16.144.235 | 104.16.144.235 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 797 | 172.64.53.181 | 172.64.53.181 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 962 | 104.25.240.182 | 104.25.240.182 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 971 | 104.26.11.160 | 104.26.11.160 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 996 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 30 | cloudflare |
| 72 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 81 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 226 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 258 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 264 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 375 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 393 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 398 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 399 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 456 | 104.19.212.207 | 104.19.212.207 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 487 | 173.245.49.194 | 173.245.49.194 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 499 | 104.17.101.208 | 104.17.101.208 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 549 | 104.16.255.1 | 104.16.255.1 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 673 | 172.64.229.156 | 172.64.229.156 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 699 | 104.25.241.85 | 104.25.241.85 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 754 | 104.26.1.181 | 104.26.1.181 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 770 | 104.17.189.30 | 104.17.189.30 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 826 | 104.25.247.129 | 104.25.247.129 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 827 | 104.17.129.66 | 104.17.129.66 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 849 | 162.159.40.8 | 162.159.40.8 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 866 | 172.67.79.218 | 172.67.79.218 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 888 | 172.64.53.40 | 172.64.53.40 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 913 | 172.64.146.121 | 172.64.146.121 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 951 | 162.159.1.111 | 162.159.1.111 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 980 | 162.159.25.10 | 162.159.25.10 | IPv4 | h3 | ✅ 成功 | 31 | cloudflare |
| 12 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 56 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 95 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 96 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 103 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 106 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 142 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 225 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 254 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 274 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 346 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 350 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 404 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 431 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 433 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 443 | 104.18.151.172 | 104.18.151.172 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 486 | 162.159.24.131 | 162.159.24.131 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 489 | 172.64.229.7 | 172.64.229.7 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 492 | 104.18.40.39 | 104.18.40.39 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 498 | 104.17.100.254 | 104.17.100.254 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 502 | 108.162.195.1 | 108.162.195.1 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 530 | 104.18.45.95 | 104.18.45.95 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 556 | 108.162.198.48 | 108.162.198.48 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 583 | 104.20.18.47 | 104.20.18.47 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 584 | 104.26.12.227 | 104.26.12.227 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 588 | 172.67.78.67 | 172.67.78.67 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 604 | 104.17.214.136 | 104.17.214.136 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 653 | 104.25.245.233 | 104.25.245.233 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 662 | 198.41.222.191 | 198.41.222.191 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 677 | 172.64.52.224 | 172.64.52.224 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 701 | 104.25.240.123 | 104.25.240.123 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 737 | 104.25.254.89 | 104.25.254.89 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 768 | 104.19.153.47 | 104.19.153.47 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 781 | 108.162.198.69 | 108.162.198.69 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 810 | 162.159.6.186 | 162.159.6.186 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 811 | 162.159.21.16 | 162.159.21.16 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 824 | 104.17.97.146 | 104.17.97.146 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 850 | 172.64.152.85 | 172.64.152.85 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 855 | 108.162.198.152 | 108.162.198.152 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 869 | 104.26.4.213 | 104.26.4.213 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 871 | 104.25.240.21 | 104.25.240.21 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 877 | 104.25.252.192 | 104.25.252.192 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 880 | 108.162.198.170 | 108.162.198.170 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 908 | 104.20.17.85 | 104.20.17.85 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 910 | 162.159.27.183 | 162.159.27.183 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 911 | 172.64.229.82 | 172.64.229.82 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 930 | 172.64.52.68 | 172.64.52.68 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 943 | 172.67.76.220 | 172.67.76.220 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 946 | 104.18.41.101 | 104.18.41.101 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 954 | 172.67.73.173 | 172.67.73.173 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 958 | 104.18.146.232 | 104.18.146.232 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 964 | 104.25.251.82 | 104.25.251.82 | IPv4 | h3 | ✅ 成功 | 32 | cloudflare |
| 7 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 13 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 120 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 123 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 197 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 265 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 272 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 281 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 338 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 515 | 104.20.17.233 | 104.20.17.233 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 522 | 104.17.24.232 | 104.17.24.232 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 527 | 162.159.13.51 | 162.159.13.51 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 540 | 104.20.24.107 | 104.20.24.107 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 590 | 172.67.79.150 | 172.67.79.150 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 609 | 104.18.47.46 | 104.18.47.46 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 648 | 104.17.193.113 | 104.17.193.113 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 654 | 104.18.166.232 | 104.18.166.232 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 694 | 104.26.5.194 | 104.26.5.194 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 704 | 104.25.249.225 | 104.25.249.225 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 706 | 172.64.153.183 | 172.64.153.183 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 736 | 104.26.3.117 | 104.26.3.117 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 746 | 172.64.53.93 | 172.64.53.93 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 749 | 104.18.42.61 | 104.18.42.61 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 771 | 104.19.35.242 | 104.19.35.242 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 773 | 104.17.25.241 | 104.17.25.241 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 818 | 104.26.6.238 | 104.26.6.238 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 859 | 172.64.41.47 | 172.64.41.47 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 886 | 172.64.145.242 | 172.64.145.242 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 945 | 104.20.24.177 | 104.20.24.177 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 953 | 172.67.66.16 | 172.67.66.16 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 963 | 104.25.245.153 | 104.25.245.153 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 999 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 1003 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare |
| 5 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 44 | cu.877774.xyz | 104.18.42.54 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 129 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 174 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 205 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 206 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 243 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 275 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 286 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 327 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 347 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 353 | tasteatlas.com | 104.17.37.105 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 402 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 460 | 104.17.167.134 | 104.17.167.134 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 488 | 162.159.136.89 | 162.159.136.89 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 501 | 162.159.34.55 | 162.159.34.55 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 576 | 104.26.8.148 | 104.26.8.148 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 605 | 104.17.187.186 | 104.17.187.186 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 633 | 162.159.36.205 | 162.159.36.205 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 636 | 104.26.14.117 | 104.26.14.117 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 642 | 104.26.8.171 | 104.26.8.171 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 646 | 104.17.97.228 | 104.17.97.228 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 715 | 162.159.43.50 | 162.159.43.50 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 727 | 172.67.79.249 | 172.67.79.249 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 739 | 104.25.240.227 | 104.25.240.227 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 745 | 104.16.245.121 | 104.16.245.121 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 748 | 172.64.146.67 | 172.64.146.67 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 760 | 104.17.211.218 | 104.17.211.218 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 766 | 104.19.34.231 | 104.19.34.231 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 769 | 104.17.209.79 | 104.17.209.79 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 775 | 104.16.251.143 | 104.16.251.143 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 807 | 172.64.52.194 | 172.64.52.194 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 828 | 104.25.241.19 | 104.25.241.19 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 852 | 172.64.52.150 | 172.64.52.150 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 872 | 104.25.243.36 | 104.25.243.36 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 904 | 172.64.41.216 | 172.64.41.216 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 914 | 104.18.47.193 | 104.18.47.193 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 935 | 172.64.49.77 | 172.64.49.77 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 947 | 162.159.16.136 | 162.159.16.136 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 949 | 172.64.42.158 | 172.64.42.158 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 965 | 104.18.47.249 | 104.18.47.249 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 983 | 172.64.153.141 | 172.64.153.141 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 1000 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 1001 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare |
| 11 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 117 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 128 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 175 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 192 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 271 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 317 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 406 | 162.159.128.253 | 162.159.128.253 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 417 | 104.18.81.19 | 104.18.81.19 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 455 | 104.18.255.167 | 104.18.255.167 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 511 | 104.20.16.244 | 104.20.16.244 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 577 | 172.67.67.5 | 172.67.67.5 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 697 | 104.18.148.235 | 104.18.148.235 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 711 | 172.64.151.253 | 172.64.151.253 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 751 | 104.18.40.216 | 104.18.40.216 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 783 | 162.159.19.219 | 162.159.19.219 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 785 | 162.159.7.12 | 162.159.7.12 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 809 | 172.64.229.149 | 172.64.229.149 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 829 | 104.25.253.253 | 104.25.253.253 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 865 | 172.67.65.44 | 172.67.65.44 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 873 | 104.25.250.121 | 104.25.250.121 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 881 | 162.159.41.141 | 162.159.41.141 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 885 | 172.64.151.235 | 172.64.151.235 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 889 | 108.162.198.148 | 108.162.198.148 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 932 | 108.162.198.216 | 108.162.198.216 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 990 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 993 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare |
| 74 | 172.67.243.218 | 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 146 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 186 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 188 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 191 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 210 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 215 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 288 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 294 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 295 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 301 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 344 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 354 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 363 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 419 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 429 | 104.19.148.121 | 104.19.148.121 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 458 | 104.31.16.158 | 104.31.16.158 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 496 | 104.19.50.35 | 104.19.50.35 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 500 | 104.16.157.50 | 104.16.157.50 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 555 | 104.17.105.198 | 104.17.105.198 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 586 | 104.16.148.187 | 104.16.148.187 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 611 | 162.159.36.52 | 162.159.36.52 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 627 | 172.64.52.189 | 172.64.52.189 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 659 | 172.64.53.202 | 172.64.53.202 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 700 | 104.25.247.78 | 104.25.247.78 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 741 | 104.25.242.249 | 104.25.242.249 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 821 | 172.64.146.137 | 172.64.146.137 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 825 | 104.25.252.135 | 104.25.252.135 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 861 | 104.20.20.156 | 104.20.20.156 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 957 | 104.17.58.25 | 104.17.58.25 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 102 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 183 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 187 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 209 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 234 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 296 | palera.in | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 323 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 345 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 373 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 447 | 104.19.220.22 | 104.19.220.22 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 516 | 172.67.70.253 | 172.67.70.253 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 536 | 172.67.64.214 | 172.67.64.214 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 538 | 104.20.22.185 | 104.20.22.185 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 543 | 104.26.3.176 | 104.26.3.176 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 573 | 172.64.52.110 | 172.64.52.110 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 587 | 104.17.60.233 | 104.17.60.233 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 626 | 108.162.198.85 | 108.162.198.85 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 632 | 104.18.42.16 | 104.18.42.16 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 652 | 172.64.153.140 | 172.64.153.140 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 657 | 162.159.11.128 | 162.159.11.128 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 708 | 172.64.144.132 | 172.64.144.132 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 735 | 172.67.77.185 | 172.67.77.185 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 759 | 172.67.72.36 | 172.67.72.36 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 784 | 172.64.52.15 | 172.64.52.15 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 887 | 172.64.145.119 | 172.64.145.119 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 956 | 104.18.229.68 | 104.18.229.68 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 967 | 162.159.26.107 | 162.159.26.107 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 1004 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare |
| 17 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 57 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 71 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 169 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 193 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 195 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 287 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 300 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 312 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 370 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 442 | 172.64.82.114 | 172.64.82.114 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 445 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 477 | 198.41.208.15 | 198.41.208.15 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 479 | 162.159.140.85 | 162.159.140.85 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 529 | 104.18.47.253 | 104.18.47.253 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 547 | 172.64.229.191 | 172.64.229.191 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 592 | 104.20.17.51 | 104.20.17.51 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 656 | 104.25.250.174 | 104.25.250.174 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 676 | 108.162.198.70 | 108.162.198.70 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 686 | 104.20.31.132 | 104.20.31.132 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 796 | 172.64.53.144 | 172.64.53.144 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 814 | 104.20.29.62 | 104.20.29.62 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 815 | 172.67.78.23 | 172.67.78.23 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 820 | 172.67.68.110 | 172.67.68.110 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 868 | 172.67.76.20 | 172.67.76.20 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 929 | 104.18.40.200 | 104.18.40.200 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 940 | 104.26.6.62 | 104.26.6.62 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 994 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 1002 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h3 | ✅ 成功 | 38 | cloudflare |
| 46 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 125 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 165 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 415 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 448 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 461 | 104.18.223.253 | 104.18.223.253 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 537 | 172.67.74.57 | 172.67.74.57 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 591 | 104.20.25.161 | 104.20.25.161 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 594 | 104.19.144.159 | 104.19.144.159 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 635 | 104.26.1.88 | 104.26.1.88 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 738 | 104.25.246.24 | 104.25.246.24 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 831 | 104.17.171.88 | 104.17.171.88 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 922 | 172.64.53.103 | 172.64.53.103 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 941 | 172.67.70.197 | 172.67.70.197 | IPv4 | h3 | ✅ 成功 | 39 | cloudflare |
| 66 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 178 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 606 | 104.18.36.1 | 104.18.36.1 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 621 | 104.18.40.202 | 104.18.40.202 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 689 | 104.20.24.244 | 104.20.24.244 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 695 | 104.20.19.180 | 104.20.19.180 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 742 | 104.17.143.82 | 104.17.143.82 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 756 | 172.67.75.11 | 172.67.75.11 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 777 | 104.17.53.25 | 104.17.53.25 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 782 | 172.64.147.235 | 172.64.147.235 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 864 | 172.67.77.104 | 172.67.77.104 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 878 | 104.25.254.47 | 104.25.254.47 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 948 | 104.18.37.177 | 104.18.37.177 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 10 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 94 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 203 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 357 | www.udemy.com | 104.16.142.237 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 365 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 383 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 438 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 484 | 172.64.91.69 | 172.64.91.69 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 485 | 172.64.48.226 | 172.64.48.226 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 509 | 162.159.62.6 | 162.159.62.6 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 535 | 104.26.15.85 | 104.26.15.85 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 582 | 104.26.4.190 | 104.26.4.190 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 601 | 104.17.25.87 | 104.17.25.87 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 639 | 104.20.25.181 | 104.20.25.181 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 726 | 104.20.24.239 | 104.20.24.239 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 819 | 104.26.12.33 | 104.26.12.33 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 834 | 104.18.44.159 | 104.18.44.159 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare |
| 219 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 494 | 104.17.119.199 | 104.17.119.199 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 682 | 108.162.198.198 | 108.162.198.198 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 725 | 172.64.229.15 | 172.64.229.15 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 733 | 104.20.22.141 | 104.20.22.141 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 734 | 172.67.65.81 | 172.67.65.81 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 767 | 104.17.215.66 | 104.17.215.66 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 774 | 162.159.42.146 | 162.159.42.146 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 270 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 390 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 428 | 198.41.208.224 | 198.41.208.224 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 523 | 104.17.50.237 | 104.17.50.237 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 580 | 104.20.20.42 | 104.20.20.42 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 696 | 172.67.72.212 | 172.67.72.212 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 857 | 172.64.229.106 | 172.64.229.106 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 934 | 172.64.229.202 | 172.64.229.202 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare |
| 134 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 318 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 331 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 420 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 559 | 108.162.194.125 | 108.162.194.125 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 593 | 172.67.67.152 | 172.67.67.152 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 698 | 104.25.245.215 | 104.25.245.215 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 776 | 162.159.21.222 | 162.159.21.222 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare |
| 374 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 409 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 519 | 104.26.4.44 | 104.26.4.44 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 542 | 172.67.72.254 | 172.67.72.254 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 575 | 104.20.26.58 | 104.20.26.58 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 658 | 162.159.12.120 | 162.159.12.120 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 942 | 104.20.25.24 | 104.20.25.24 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 955 | 172.64.41.232 | 172.64.41.232 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare |
| 32 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 89 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 121 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 137 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 236 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 364 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 493 | 104.19.44.238 | 104.19.44.238 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 683 | 162.159.0.41 | 162.159.0.41 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 693 | 104.20.22.91 | 104.20.22.91 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 853 | 172.64.53.195 | 172.64.53.195 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 874 | 104.25.241.235 | 104.25.241.235 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 937 | 172.64.156.202 | 172.64.156.202 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 944 | 172.67.68.156 | 172.67.68.156 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare |
| 136 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 194 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 253 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 426 | 104.18.89.52 | 104.18.89.52 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 561 | 162.159.0.115 | 162.159.0.115 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 690 | 104.26.11.33 | 104.26.11.33 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 836 | 172.64.157.214 | 172.64.157.214 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 882 | 162.159.18.240 | 162.159.18.240 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 950 | 172.64.32.77 | 172.64.32.77 | IPv4 | h3 | ✅ 成功 | 47 | cloudflare |
| 732 | 104.26.5.53 | 104.26.5.53 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 747 | 162.159.44.246 | 162.159.44.246 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 912 | 108.162.198.206 | 108.162.198.206 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare |
| 23 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 114 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 249 | cf.090227.xyz | 104.18.43.174 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 446 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 505 | 162.159.36.26 | 162.159.36.26 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 525 | 162.159.6.115 | 162.159.6.115 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 637 | 104.20.28.239 | 104.20.28.239 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 740 | 104.17.56.177 | 104.17.56.177 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 806 | 108.162.198.232 | 108.162.198.232 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 2 | 172.64.53.220 | 172.64.53.220 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 87 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 119 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 220 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 248 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 437 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 765 | 104.17.115.224 | 104.17.115.224 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 803 | 172.64.34.153 | 172.64.34.153 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 837 | 172.64.53.15 | 172.64.53.15 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 840 | 162.159.0.79 | 162.159.0.79 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 401 | 162.159.137.204 | 162.159.137.204 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 476 | 104.16.65.1 | 104.16.65.1 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 497 | 104.16.155.76 | 104.16.155.76 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 508 | 104.18.39.228 | 104.18.39.228 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 524 | 104.16.153.12 | 104.16.153.12 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 539 | 172.67.65.159 | 172.67.65.159 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 550 | 104.17.156.81 | 104.17.156.81 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 678 | 172.64.53.41 | 172.64.53.41 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 702 | 104.25.244.36 | 104.25.244.36 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 856 | 162.159.39.196 | 162.159.39.196 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare |
| 105 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 139 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 280 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 432 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 436 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 483 | 162.159.61.183 | 162.159.61.183 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 554 | 104.17.170.110 | 104.17.170.110 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 812 | 104.26.1.55 | 104.26.1.55 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 905 | 172.64.154.113 | 172.64.154.113 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare |
| 3 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 64 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 73 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 111 | freeyx.cloudflare88.eu.org | 141.101.120.51 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 130 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 322 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 337 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 369 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 441 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 454 | 104.19.154.200 | 104.19.154.200 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 462 | 104.16.105.166 | 104.16.105.166 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 490 | 162.159.58.65 | 162.159.58.65 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 643 | 104.26.0.210 | 104.26.0.210 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 651 | 104.25.244.239 | 104.25.244.239 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 687 | 162.159.20.46 | 162.159.20.46 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 779 | 104.17.21.106 | 104.17.21.106 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare |
| 4 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 20 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 77 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 101 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 126 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 138 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 198 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 224 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 302 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 358 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 430 | 104.17.162.3 | 104.17.162.3 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 520 | 172.67.76.195 | 172.67.76.195 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 551 | 104.17.16.248 | 104.17.16.248 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 608 | 104.18.32.174 | 104.18.32.174 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 625 | 172.64.154.226 | 172.64.154.226 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 764 | 104.17.170.137 | 104.17.170.137 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 959 | 104.17.107.237 | 104.17.107.237 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 961 | 104.25.255.4 | 104.25.255.4 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |
| 27 | icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 62 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 158 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 208 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 214 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 279 | 104.16.147.114 | 104.16.147.114 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 313 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 380 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 403 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 427 | 104.18.166.129 | 104.18.166.129 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 526 | 104.17.30.164 | 104.17.30.164 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 528 | 108.162.192.66 | 108.162.192.66 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 533 | 172.64.150.30 | 172.64.150.30 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 655 | 104.17.111.150 | 104.17.111.150 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 688 | 172.67.70.56 | 172.67.70.56 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 863 | 172.67.64.123 | 172.67.64.123 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 992 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 70 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 131 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 290 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 306 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 503 | 162.159.42.140 | 162.159.42.140 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 518 | 104.26.8.192 | 104.26.8.192 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 553 | 104.17.168.159 | 104.17.168.159 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 581 | 104.20.21.161 | 104.20.21.161 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 624 | 104.18.42.106 | 104.18.42.106 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 705 | 104.25.254.14 | 104.25.254.14 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 817 | 104.26.14.88 | 104.26.14.88 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 822 | 172.67.76.61 | 172.67.76.61 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 830 | 104.16.247.125 | 104.16.247.125 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 901 | 162.159.61.106 | 162.159.61.106 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare |
| 164 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 231 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 388 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 603 | 172.64.53.165 | 172.64.53.165 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 685 | 162.159.44.202 | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 761 | 104.20.25.82 | 104.20.25.82 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 879 | 104.25.242.137 | 104.25.242.137 | IPv4 | h3 | ✅ 成功 | 57 | cloudflare |
| 389 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 407 | 104.26.8.117 | 104.26.8.117 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 649 | 104.26.13.110 | 104.26.13.110 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 723 | 162.159.45.150 | 162.159.45.150 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 902 | 162.159.45.165 | 162.159.45.165 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare |
| 112 | freeyx.cloudflare88.eu.org | 141.101.121.171 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 141 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 238 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 289 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 335 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 578 | 172.67.77.196 | 172.67.77.196 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 597 | 104.17.53.129 | 104.17.53.129 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 707 | 104.25.250.205 | 104.25.250.205 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 870 | 172.67.73.129 | 172.67.73.129 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 876 | 104.25.245.173 | 104.25.245.173 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare |
| 63 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 340 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 474 | 104.18.189.153 | 104.18.189.153 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 595 | 104.17.153.151 | 104.17.153.151 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 628 | 162.159.45.65 | 162.159.45.65 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare |
| 140 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 179 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 997 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 681 | 162.159.45.67 | 162.159.45.67 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 755 | 104.26.2.2 | 104.26.2.2 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 931 | 172.64.53.223 | 172.64.53.223 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare |
| 884 | 104.25.246.117 | 104.25.246.117 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 969 | 104.17.98.142 | 104.17.98.142 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare |
| 127 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 227 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 246 | saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 382 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 463 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 728 | 162.159.38.83 | 162.159.38.83 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 833 | 104.25.255.103 | 104.25.255.103 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare |
| 16 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 115 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 416 | 162.159.140.116 | 162.159.140.116 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 867 | 104.20.29.234 | 104.20.29.234 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 38 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 50 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 204 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 517 | 104.26.6.159 | 104.26.6.159 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 750 | 104.17.62.194 | 104.17.62.194 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 763 | 104.26.10.239 | 104.26.10.239 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 952 | 172.64.145.108 | 172.64.145.108 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |
| 85 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 86 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 207 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 394 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 757 | 172.67.73.120 | 172.67.73.120 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 875 | 104.20.18.125 | 104.20.18.125 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare |
| 93 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 170 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 512 | 104.26.2.166 | 104.26.2.166 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 596 | 104.26.4.4 | 104.26.4.4 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 692 | 172.67.67.0 | 172.67.67.0 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 709 | 162.159.33.191 | 162.159.33.191 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 778 | 172.64.52.67 | 172.64.52.67 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 823 | 172.67.64.116 | 172.67.64.116 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 900 | 162.159.44.133 | 162.159.44.133 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 28 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 118 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 478 | 162.159.17.243 | 162.159.17.243 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 506 | 104.18.37.110 | 104.18.37.110 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 724 | 172.64.52.181 | 172.64.52.181 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 907 | 162.159.6.44 | 162.159.6.44 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 31 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 110 | freeyx.cloudflare88.eu.org | 141.101.120.153 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 199 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 307 | ip.gs | 188.114.97.3 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 410 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 680 | 162.159.39.20 | 162.159.39.20 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 758 | 104.26.3.120 | 104.26.3.120 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 960 | 172.67.71.178 | 172.67.71.178 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 982 | 162.159.8.49 | 162.159.8.49 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 116 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 147 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 459 | 104.17.69.244 | 104.17.69.244 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 548 | 104.17.169.180 | 104.17.169.180 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 629 | 162.159.38.226 | 162.159.38.226 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 752 | 162.159.39.136 | 162.159.39.136 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 928 | 162.159.44.199 | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 71 | cloudflare |
| 45 | cu.877774.xyz | 172.64.145.202 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 49 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 223 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 321 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 418 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 482 | 172.64.52.127 | 172.64.52.127 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 661 | 162.159.58.17 | 162.159.58.17 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 703 | 104.17.56.208 | 104.17.56.208 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 835 | 172.64.229.185 | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 933 | 162.159.38.9 | 162.159.38.9 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 984 | 108.162.198.127 | 108.162.198.127 | IPv4 | h3 | ✅ 成功 | 72 | cloudflare |
| 589 | 104.20.19.201 | 104.20.19.201 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 607 | 104.16.251.254 | 104.16.251.254 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 610 | 172.64.49.146 | 172.64.49.146 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 641 | 104.26.6.247 | 104.26.6.247 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 906 | 162.159.36.223 | 162.159.36.223 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 909 | 172.64.152.215 | 172.64.152.215 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare |
| 65 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 159 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 230 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 297 | palera.in | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 349 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 531 | 162.159.46.238 | 162.159.46.238 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 679 | 162.159.1.39 | 162.159.1.39 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 731 | 172.67.73.196 | 172.67.73.196 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 743 | 104.25.244.87 | 104.25.244.87 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 854 | 108.162.198.223 | 108.162.198.223 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 897 | 162.159.3.89 | 162.159.3.89 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 986 | 104.17.154.254 | 104.17.154.254 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare |
| 182 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 311 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 513 | 104.26.12.113 | 104.26.12.113 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 598 | 104.26.7.7 | 104.26.7.7 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 660 | 162.159.3.128 | 162.159.3.128 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 710 | 162.159.39.74 | 162.159.39.74 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 200 | steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 308 | ip.gs | 188.114.96.3 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 339 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 424 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 491 | 104.26.4.90 | 104.26.4.90 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 510 | 162.159.19.37 | 162.159.19.37 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 546 | 104.20.30.198 | 104.20.30.198 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 574 | 162.159.45.93 | 162.159.45.93 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 615 | 162.159.1.145 | 162.159.1.145 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare |
| 122 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 154 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 602 | 104.17.110.226 | 104.17.110.226 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 647 | 172.67.65.150 | 172.67.65.150 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 798 | 162.159.44.128 | 162.159.44.128 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 981 | 162.159.44.99 | 162.159.44.99 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 985 | 104.18.41.193 | 104.18.41.193 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 995 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare |
| 133 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 153 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 504 | 104.18.35.166 | 104.18.35.166 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare |
| 76 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 475 | 104.17.142.212 | 104.17.142.212 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 544 | 104.26.1.194 | 104.26.1.194 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 862 | 172.64.40.68 | 172.64.40.68 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 966 | 104.17.215.219 | 104.17.215.219 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare |
| 408 | 104.26.3.162 | 104.26.3.162 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 791 | 162.159.45.176 | 162.159.45.176 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare |
| 381 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 480 | 162.159.21.116 | 162.159.21.116 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 534 | 173.245.58.237 | 173.245.58.237 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 927 | 162.159.33.28 | 162.159.33.28 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare |
| 37 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 152 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare |
| 160 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare |
| 634 | 162.159.44.101 | 162.159.44.101 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 260 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 266 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 457 | 198.41.194.162 | 198.41.194.162 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |
| 330 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 495 | 104.26.5.134 | 104.26.5.134 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 560 | 162.159.44.176 | 162.159.44.176 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 923 | 162.159.49.244 | 162.159.49.244 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 247 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 612 | 172.64.40.196 | 172.64.40.196 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 674 | 162.159.39.180 | 162.159.39.180 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare |
| 816 | 162.159.39.26 | 162.159.39.26 | IPv4 | h3 | ✅ 成功 | 89 | cloudflare |
| 168 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 800 | 162.159.38.35 | 162.159.38.35 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare |
| 579 | 162.159.39.62 | 162.159.39.62 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 638 | 162.159.39.146 | 162.159.39.146 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 805 | 162.159.39.177 | 162.159.39.177 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare |
| 449 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare |
| 104 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare |
| 97 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 860 | 162.159.44.60 | 162.159.44.60 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 936 | 162.159.45.180 | 162.159.45.180 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare |
| 242 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 813 | 162.159.38.52 | 162.159.38.52 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare |
| 851 | 162.159.39.156 | 162.159.39.156 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare |
| 6 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 75 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 392 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 939 | 162.159.44.16 | 162.159.44.16 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare |
| 411 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 804 | 162.159.38.171 | 162.159.38.171 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare |
| 88 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 252 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 799 | 162.159.39.99 | 162.159.39.99 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 938 | 162.159.39.203 | 162.159.39.203 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare |
| 33 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 148 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 237 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 630 | 162.159.45.121 | 162.159.45.121 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 572 | 162.159.38.192 | 162.159.38.192 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 903 | 162.159.38.67 | 162.159.38.67 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare |
| 684 | 162.159.38.45 | 162.159.38.45 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 808 | 162.159.45.145 | 162.159.45.145 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare |
| 273 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 675 | 162.159.38.134 | 162.159.38.134 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 858 | 162.159.45.237 | 162.159.45.237 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare |
| 259 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 282 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare |
| 384 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 762 | 104.26.5.101 | 104.26.5.101 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare |
| 47 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 832 | 172.67.72.250 | 172.67.72.250 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare |
| 48 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h3 | ✅ 成功 | 172 | cloudflare |
| 640 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 282 | cloudflare |
| 987 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 405 | cloudflare |
| 541 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 415 | cloudflare |
| 405 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 624 | cloudflare |
| 376 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 985 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 420 条记录
- **快 (50-100ms)**: 289 条记录
- **正常 (100-200ms)**: 18 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 2 条记录


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
