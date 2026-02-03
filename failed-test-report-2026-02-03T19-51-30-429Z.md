# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/3 19:51:30
- **数据来源**: connectivity_results-20260203-195129.json
- **总测试数**: 792
- **失败测试数**: 2
- **成功测试数**: 790
- **失败率**: 0.25%
- **平均延迟**: 96.45ms
- **最小延迟**: 53ms
- **最大延迟**: 906ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/3 19:51:30
- **IP地址**: 2a09:bac5:7971:8c::e:2bd
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8874, -87.6318
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 114 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 389 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 790 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 789 | 2a06:98c1:310d:f860:89a7:7bc8:11ef:4685 | 2a06:98c1:310d:f860:89a7:7bc8:11ef:4685 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 127 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 98 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 761 | 172.67.76.81 | 172.67.76.81 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 768 | 172.64.154.63 | 172.64.154.63 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 93 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 193 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 57 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 60 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 268 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 322 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 326 | ip.gs | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 413 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 730 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 756 | 172.67.67.52 | 172.67.67.52 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 64 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 90 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 161 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 327 | ip.gs | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 764 | 104.17.30.217 | 104.17.30.217 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 138 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 315 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 766 | 104.17.117.119 | 104.17.117.119 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 777 | 162.159.38.59 | 162.159.38.59 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 18 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 43 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 47 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 70 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 89 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 160 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 243 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 265 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 270 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 288 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 481 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 605 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 680 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 709 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 716 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 743 | 162.159.45.120 | 162.159.45.120 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 66 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare |
| 91 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 118 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 204 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 353 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 493 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 536 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 612 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 659 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 678 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 733 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 765 | 104.17.147.230 | 104.17.147.230 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 769 | 104.17.51.240 | 104.17.51.240 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 318 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 355 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 468 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 525 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 640 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 723 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 102 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 112 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 124 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 165 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 258 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 263 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 337 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 339 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 368 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 530 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 602 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 613 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 660 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 714 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 4 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 58 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 75 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 271 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 289 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 313 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 334 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 425 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 665 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 713 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 734 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 738 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 742 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 751 | 172.64.229.246 | 172.64.229.246 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 778 | 108.162.198.248 | 108.162.198.248 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 45 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 130 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 172 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 176 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 203 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 238 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 401 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 578 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 651 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 679 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 694 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 706 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 724 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 759 | 172.67.69.253 | 172.67.69.253 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 2 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 68 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 88 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 110 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 117 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 120 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 151 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 157 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 242 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 287 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 311 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 356 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 370 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 418 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 427 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 518 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 570 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 572 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 703 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 715 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 754 | 172.67.65.106 | 172.67.65.106 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 80 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 136 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 153 | cu.877774.xyz | 104.26.4.113 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 197 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 336 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 367 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 394 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 423 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 609 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 620 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 668 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 674 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 677 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 690 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 710 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 711 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 719 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 725 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 727 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 741 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 758 | 172.67.72.74 | 172.67.72.74 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 763 | 172.67.71.67 | 172.67.71.67 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 779 | 162.159.44.173 | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 780 | 162.159.45.21 | 162.159.45.21 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 14 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 42 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 53 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 146 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 149 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 235 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 244 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 252 | bestcf.030101.xyz | 172.64.159.213 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 254 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 259 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 283 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 312 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 361 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 369 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 383 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 471 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 520 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 551 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 642 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 644 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 667 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 707 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 735 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 739 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 6 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 76 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 123 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 129 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 155 | cu.877774.xyz | 104.26.4.115 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 156 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 170 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 233 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 237 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 266 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 296 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 422 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 475 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 566 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 591 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 772 | 104.17.149.8 | 104.17.149.8 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 10 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 44 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 113 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 134 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 158 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 182 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 290 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 332 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 341 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 440 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 460 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 483 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 557 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 588 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 597 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 658 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 689 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 695 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 37 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 51 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 83 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 104 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 147 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 168 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 192 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 196 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 240 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 251 | bestcf.030101.xyz | 104.17.144.235 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 255 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 300 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 365 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 366 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 398 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 502 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 528 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 549 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 670 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 671 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 705 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 712 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 736 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 783 | 198.41.223.173 | 198.41.223.173 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 8 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 12 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 39 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 52 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 72 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 125 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 154 | cu.877774.xyz | 104.26.4.114 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 188 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 246 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 297 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 376 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 397 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 449 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 512 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 636 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 672 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 731 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 752 | 172.67.78.158 | 172.67.78.158 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 792 | 2a06:98c1:3100:0:92d7:326:fa62:11cf | 2a06:98c1:3100:0:92d7:326:fa62:11cf | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 78 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 86 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 194 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 247 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 345 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 354 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 382 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 390 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 415 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 477 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 501 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 569 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 590 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 708 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 784 | 2a06:98c1:3108:f98c:209d:50fc:b788:b65c | 2a06:98c1:3108:f98c:209d:50fc:b788:b65c | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 69 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 77 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 105 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 121 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 191 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 232 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 276 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 295 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 335 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 375 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 476 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 532 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 563 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 564 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 611 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 666 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 55 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 178 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 253 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 277 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 302 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 316 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 331 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 377 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 384 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 387 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 459 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 494 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 580 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 581 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 589 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 740 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 33 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 73 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 108 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 152 | cu.877774.xyz | 104.26.4.112 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 269 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 310 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 323 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 404 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 610 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 737 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 757 | 172.67.79.70 | 172.67.79.70 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 791 | 2a06:98c1:3108:f969:876d:b1a7:497d:57dd | 2a06:98c1:3108:f969:876d:b1a7:497d:57dd | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 107 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 177 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 181 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 274 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 329 | ip.gs | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 349 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 373 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 441 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 524 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 604 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 628 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 634 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 704 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 748 | 162.159.44.59 | 162.159.44.59 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 767 | 104.17.215.215 | 104.17.215.215 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 785 | 2a06:98c1:310c:96ca:8a3d:395:146c:c376 | 2a06:98c1:310c:96ca:8a3d:395:146c:c376 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 65 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 87 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 97 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 144 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 245 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 343 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 363 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 455 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 457 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 473 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 573 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 631 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 639 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 657 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 56 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 84 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 163 | freeyx.cloudflare88.eu.org | 141.101.120.232 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 261 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 386 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 474 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 656 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 661 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 669 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 693 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 745 | 172.64.53.51 | 172.64.53.51 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 3 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 62 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 74 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 142 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 262 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 342 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 347 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 542 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 565 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 614 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 131 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 286 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 293 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 340 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 499 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 527 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 567 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 582 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 618 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 626 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 633 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 635 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 643 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 700 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 298 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 321 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 505 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 652 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 7 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 17 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 38 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 41 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 106 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 128 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 180 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 278 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 333 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 364 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 395 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 400 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 402 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 472 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 498 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 517 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 579 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 607 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 623 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 648 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 650 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 747 | 172.64.52.17 | 172.64.52.17 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 790 | 2a06:98c1:3107:1c8c:c953:32f0:877e:d809 | 2a06:98c1:3107:1c8c:c953:32f0:877e:d809 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 13 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 15 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 48 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 85 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 201 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 309 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 409 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 464 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 529 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 653 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 692 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 698 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 699 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 755 | 172.67.73.69 | 172.67.73.69 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 109 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 186 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 189 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 412 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 595 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 603 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 717 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 770 | 104.17.155.181 | 104.17.155.181 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 63 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 71 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 111 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 325 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 403 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 568 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 697 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 718 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 728 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 746 | 108.162.198.121 | 108.162.198.121 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 762 | 172.67.75.58 | 172.67.75.58 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 79 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 328 | ip.gs | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 351 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 414 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 438 | www.7749tv.com | 104.24.211.67 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 462 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 115 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 150 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 285 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 346 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 362 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 391 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 428 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 469 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 629 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 732 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 11 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 40 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 50 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 280 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 574 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 676 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 683 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 696 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 122 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 264 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 279 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 479 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 103 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 260 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 431 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 433 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 486 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 575 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 116 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 195 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 352 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 450 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 544 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 584 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 621 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 773 | 104.17.148.211 | 104.17.148.211 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 31 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 378 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 488 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 691 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 139 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 294 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 419 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 463 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 513 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 599 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 647 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 774 | 172.64.53.118 | 172.64.53.118 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 16 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 35 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 338 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 531 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 558 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 571 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 649 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 788 | 2a06:98c1:310b:636a:5598:d530:8b94:e23c | 2a06:98c1:310b:636a:5598:d530:8b94:e23c | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 24 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 49 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 61 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 538 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 554 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 585 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 34 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 95 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 330 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 411 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 442 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 526 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 630 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 29 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 101 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 461 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 489 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 596 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 96 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 317 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 371 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 405 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 497 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 663 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 685 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 782 | 162.159.39.132 | 162.159.39.132 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 67 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 199 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 234 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 250 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 447 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 456 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 507 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 523 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 598 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 359 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 393 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 534 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 720 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 744 | 172.64.50.250 | 172.64.50.250 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 169 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 187 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 257 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 284 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 385 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 444 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 500 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 533 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 535 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 682 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 729 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 20 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 23 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 133 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 162 | freeyx.cloudflare88.eu.org | 141.101.120.150 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 303 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 360 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 407 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 106 | cloudflare |
| 664 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 675 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 126 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 159 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 174 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 183 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 396 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 491 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 548 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 608 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 627 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 637 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 687 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 107 | cloudflare |
| 99 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 239 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 256 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 408 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 655 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 776 | 172.64.52.178 | 172.64.52.178 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 787 | 2400:cb00:f00e:d69d:66dd:1321:64a4:5076 | 2400:cb00:f00e:d69d:66dd:1321:64a4:5076 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 140 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 145 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 109 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 231 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 291 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 421 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 522 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 619 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 622 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 760 | 162.159.18.13 | 162.159.18.13 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 9 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 26 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 82 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 275 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 399 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 406 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 110 | cloudflare |
| 490 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 92 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 299 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 388 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 482 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 111 | cloudflare |
| 638 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 645 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 775 | 162.159.46.137 | 162.159.46.137 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 5 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 94 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 148 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 198 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 306 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 350 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 358 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 506 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 641 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 662 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 446 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 701 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 726 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 1 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 429 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 451 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 624 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 114 | cloudflare |
| 646 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 319 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 410 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 448 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 115 | cloudflare |
| 487 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 519 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 576 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 617 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 753 | 172.64.34.185 | 172.64.34.185 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 185 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 305 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 116 | cloudflare |
| 374 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 484 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 516 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 539 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 561 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 681 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 786 | 2400:cb00:f00e:d382:fa2e:2623:6358:92de | 2400:cb00:f00e:d382:fa2e:2623:6358:92de | IPv6 | h2 | ✅ 成功 | 116 | cloudflare |
| 143 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 117 | cloudflare |
| 304 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 750 | 162.159.39.252 | 162.159.39.252 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 101 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 416 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 702 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 28 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 119 | cloudflare |
| 36 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 119 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 432 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 119 | cloudflare |
| 540 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 30 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 120 | cloudflare |
| 458 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 749 | 162.159.38.203 | 162.159.38.203 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 273 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 420 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 470 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 537 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 562 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 121 | cloudflare |
| 601 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 722 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 32 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 282 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 453 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 771 | 104.17.211.235 | 104.17.211.235 | IPv4 | h2 | ✅ 成功 | 122 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 392 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 123 | cloudflare |
| 437 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 46 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 124 | cloudflare |
| 236 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 124 | cloudflare |
| 308 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 439 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 541 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 632 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 175 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 125 | cloudflare |
| 184 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 301 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 125 | cloudflare |
| 545 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 550 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 688 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 125 | cloudflare |
| 141 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 249 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 281 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 547 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 673 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 167 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 272 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 292 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 314 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 27 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 241 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 128 | cloudflare |
| 307 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 128 | cloudflare |
| 426 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 128 | cloudflare |
| 19 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 511 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 129 | cloudflare |
| 593 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 625 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 129 | cloudflare |
| 515 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 130 | cloudflare |
| 190 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 131 | cloudflare |
| 616 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 131 | cloudflare |
| 781 | 2a06:98c1:3100:f7e3:cbcf:b6c7:6124:2fe2 | 2a06:98c1:3100:f7e3:cbcf:b6c7:6124:2fe2 | IPv6 | h2 | ✅ 成功 | 131 | cloudflare |
| 166 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 324 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 132 | cloudflare |
| 430 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 467 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 503 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 543 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 555 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 132 | cloudflare |
| 344 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 514 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 133 | cloudflare |
| 485 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 134 | cloudflare |
| 135 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 135 | cloudflare |
| 248 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 135 | cloudflare |
| 552 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 135 | cloudflare |
| 171 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 495 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 556 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 136 | cloudflare |
| 100 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 380 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 137 | cloudflare |
| 381 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 443 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 137 | cloudflare |
| 686 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 137 | cloudflare |
| 492 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare |
| 592 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare |
| 445 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 139 | cloudflare |
| 320 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 594 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 577 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 141 | cloudflare |
| 615 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare |
| 435 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 143 | cloudflare |
| 466 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 143 | cloudflare |
| 137 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 144 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare |
| 654 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare |
| 521 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 146 | cloudflare |
| 478 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 149 | cloudflare |
| 583 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 150 | cloudflare |
| 508 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 152 | cloudflare |
| 606 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 152 | cloudflare |
| 452 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 153 | cloudflare |
| 600 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 154 | cloudflare |
| 684 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 154 | cloudflare |
| 721 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 154 | cloudflare |
| 132 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 155 | cloudflare |
| 480 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 155 | cloudflare |
| 465 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 156 | cloudflare |
| 372 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 158 | cloudflare |
| 434 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 159 | cloudflare |
| 586 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 163 | cloudflare |
| 348 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 164 | cloudflare |
| 379 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 169 | cloudflare |
| 559 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 169 | cloudflare |
| 200 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 175 | cloudflare |
| 202 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 180 | cloudflare |
| 454 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 182 | cloudflare |
| 553 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 183 | cloudflare |
| 546 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 189 | cloudflare |
| 510 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 193 | cloudflare |
| 164 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cd47:f6f7:aaa7 | IPv6 | h2 | ✅ 成功 | 194 | cloudflare |
| 509 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 196 | cloudflare |
| 560 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 196 | cloudflare |
| 424 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 200 | cloudflare |
| 357 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 202 | cloudflare |
| 496 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 206 | cloudflare |
| 587 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 213 | cloudflare |
| 436 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 214 | cloudflare |
| 504 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 214 | cloudflare |
| 173 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 229 | cloudflare |
| 179 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 235 | cloudflare |
| 267 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 537 | cloudflare |
| 81 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 697 | cloudflare |
| 417 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 867 | cloudflare |
| 54 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 906 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 521 条记录
- **正常 (100-200ms)**: 257 条记录
- **慢 (200-500ms)**: 8 条记录
- **很慢 (>500ms)**: 4 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
