# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 03:46:45
- **数据来源**: connectivity_results-20251211-034645.json
- **总测试数**: 475
- **失败测试数**: 3
- **成功测试数**: 472
- **失败率**: 0.63%
- **平均延迟**: 78.27ms
- **最小延迟**: 50ms
- **最大延迟**: 857ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (66.7%)
- **连接超时: 上下文超时**: 1 次 (33.3%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 10   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 289  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名    | 目标IP       | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | ------------ | ------------ | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 80   | 104.26.6.112 | 104.26.6.112 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 121 |
fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare | | 25 |
ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 51 |
cloudflare | | 216 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 51 |
cloudflare | | 55 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅
成功 | 52 | cloudflare | | 140 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功
| 52 | cloudflare | | 141 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 |
52 | cloudflare | | 167 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 |
IPv6 | h2 | ✅ 成功 | 52 | cloudflare | | 272 | rustam.ns.cloudflare.com |
2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare | | 39 |
www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | |
222 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 299 | yx-auto.pages.dev | 104.21.9.230 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare
| | 416 | freeyx.cloudflare88.eu.org | 141.101.120.51 | IPv4 | h2 | ✅ 成功 | 53
| cloudflare | | 424 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功
| 53 | cloudflare | | 457 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功
| 53 | cloudflare | | 463 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 |
✅ 成功 | 53 | cloudflare | | 464 | iplocation.io | 2606:4700:20::681a:bde |
IPv6 | h2 | ✅ 成功 | 53 | cloudflare | | 31 | huxley.ns.cloudflare.com |
2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 54 | cloudflare | | 60 |
japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare | | 122 |
fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare | | 192 |
craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 54 |
cloudflare | | 231 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 54 |
cloudflare | | 304 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功
| 54 | cloudflare | | 306 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6
| h2 | ✅ 成功 | 54 | cloudflare | | 367 | 456.cloudflare.182682.xyz |
2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare | | 384 |
www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare | | 451
| steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare
| | 460 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare
| | 23 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 55 |
cloudflare | | 68 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 |
✅ 成功 | 55 | cloudflare | | 170 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 |
✅ 成功 | 55 | cloudflare | | 183 | singapore.com | 2606:4700:20::681a:c8c |
IPv6 | h2 | ✅ 成功 | 55 | cloudflare | | 205 | decker.ns.cloudflare.com |
2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 55 | cloudflare | | 225 |
cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 244 |
104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 251 |
moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 55 |
cloudflare | | 277 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 55 |
cloudflare | | 372 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 55 |
cloudflare | | 374 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅
成功 | 55 | cloudflare | | 415 | freeyx.cloudflare88.eu.org | 141.101.120.153 |
IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 425 | cf.zhetengsha.eu.org |
172.64.145.158 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 439 | www.4chan.org |
104.16.229.229 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 466 | ifconfig.co |
104.21.54.91 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 64 | japan.com |
2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare | | 77 |
local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h2 | ✅ 成功 | 56
| cloudflare | | 104 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 56
| cloudflare | | 235 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 |
56 | cloudflare | | 292 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 |
✅ 成功 | 56 | cloudflare | | 332 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅
成功 | 56 | cloudflare | | 368 | 456.cloudflare.182682.xyz |
2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare | | 431 |
stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 56 |
cloudflare | | 456 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 56 |
cloudflare | | 468 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅
成功 | 56 | cloudflare | | 13 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅
成功 | 57 | cloudflare | | 18 | yx-auto.pages.dev | 2606:4700:3031::ac43:868b |
IPv6 | h2 | ✅ 成功 | 57 | cloudflare | | 20 | 172.64.157.120 | 172.64.157.120 |
IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 100 | [2606:4700:440f::53aa:4126] |
2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare | | 117 |
dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 57 |
cloudflare | | 123 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 57 |
cloudflare | | 147 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 57 |
cloudflare | | 274 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 57
| cloudflare | | 303 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅
成功 | 57 | cloudflare | | 338 | bestcf.030101.xyz |
2606:4700:0:ea:d24d:88c4:939:e08e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare | |
379 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 380 |
ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 467 |
ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 14 |
www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 58 | cloudflare
| | 16 | yx-auto.pages.dev | 104.21.6.60 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare
| | 101 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare | | 125 |
time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare | | 126 |
time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare | | 136 |
abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 58
| cloudflare | | 145 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 58 |
cloudflare | | 161 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅
成功 | 58 | cloudflare | | 177 | benedict.ns.cloudflare.com |
2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 58 | cloudflare | | 238 |
uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 58 |
cloudflare | | 243 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 |
h2 | ✅ 成功 | 58 | cloudflare | | 298 | [2606:4700:83bd::7d8:2b47] |
2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare | | 321 |
icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare | | 436 |
cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 7 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 15
| www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 59 |
cloudflare | | 26 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 |
h2 | ✅ 成功 | 59 | cloudflare | | 37 | www.hugedomains.com | 172.67.70.191 |
IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 44 | comicabc.com | 172.67.174.21 |
IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 49 | 104.16.45.84 | 104.16.45.84 |
IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 67 | yx-auto.pages.dev |
2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 89 |
www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 148 |
cu.877774.xyz | 104.26.4.112 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 152 |
104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 185 |
singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 188 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare | |
193 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 59 |
cloudflare | | 200 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 59 |
cloudflare | | 215 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 59 |
cloudflare | | 267 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功
| 59 | cloudflare | | 281 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 |
✅ 成功 | 59 | cloudflare | | 282 | palera.in | 2606:4700:3035::6815:3a48 | IPv6
| h2 | ✅ 成功 | 59 | cloudflare | | 288 | pranab.ns.cloudflare.com |
2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 293 |
wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 59 |
cloudflare | | 296 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 59
| cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
