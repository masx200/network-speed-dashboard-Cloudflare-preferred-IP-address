# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 04:56:30
- **数据来源**: connectivity_results-20251211-045630.json
- **总测试数**: 483
- **失败测试数**: 2
- **成功测试数**: 481
- **失败率**: 0.41%
- **平均延迟**: 62.80ms
- **最小延迟**: 41ms
- **最大延迟**: 687ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 204  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 450  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 278 |
cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 41 |
cloudflare | | 69 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 42 |
cloudflare | | 84 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 42
| cloudflare | | 253 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 42 |
cloudflare | | 275 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 42 |
cloudflare | | 98 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 43 |
cloudflare | | 276 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 43 |
cloudflare | | 324 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac |
IPv6 | h2 | ✅ 成功 | 43 | cloudflare | | 352 | cf.zhetengsha.eu.org |
2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 43 | cloudflare | | 70 |
ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare | | 132 |
toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare
| | 239 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 44 |
cloudflare | | 277 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅
成功 | 44 | cloudflare | | 282 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅
成功 | 44 | cloudflare | | 287 | 456.cloudflare.182682.xyz |
2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare | | 385 |
104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare | | 398 |
yx-auto.pages.dev | 2606:4700:3031::ac43:868b | IPv6 | h2 | ✅ 成功 | 44 |
cloudflare | | 53 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 |
✅ 成功 | 45 | cloudflare | | 81 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功
| 45 | cloudflare | | 121 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功
| 45 | cloudflare | | 214 | yx-auto.pages.dev | 104.21.9.230 | IPv4 | h2 | ✅
成功 | 45 | cloudflare | | 235 | [2606:4700:440b::3e6e:5f06] |
2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare | | 305 |
172.64.229.249 | 172.64.229.249 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare | | 336
| www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare | | 338
| www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 356 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare | |
394 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 45 |
cloudflare | | 60 | 172.64.153.172 | 172.64.153.172 | IPv4 | h2 | ✅ 成功 | 46 |
cloudflare | | 65 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 46 |
cloudflare | | 181 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功
| 46 | cloudflare | | 191 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅
成功 | 46 | cloudflare | | 192 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅
成功 | 46 | cloudflare | | 212 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅
成功 | 46 | cloudflare | | 237 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功
| 46 | cloudflare | | 238 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅
成功 | 46 | cloudflare | | 252 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6
| h2 | ✅ 成功 | 46 | cloudflare | | 304 | dnschecker.org |
2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare | | 317 |
cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 46 |
cloudflare | | 340 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 46 |
cloudflare | | 346 | ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 46 |
cloudflare | | 358 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功
| 46 | cloudflare | | 383 | ae8a9c24-83de.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare | | 395 |
iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare
| | 408 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 46 |
cloudflare | | 423 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 |
46 | cloudflare | | 430 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 |
h2 | ✅ 成功 | 46 | cloudflare | | 459 | cmcc.877774.xyz | 104.16.149.5 | IPv4 |
h2 | ✅ 成功 | 46 | cloudflare | | 461 | cmcc.877774.xyz | 104.16.149.7 | IPv4 |
h2 | ✅ 成功 | 46 | cloudflare | | 464 | cmcc.877774.xyz | 104.16.149.10 | IPv4
| h2 | ✅ 成功 | 46 | cloudflare | | 476 | cmcc.877774.xyz | 104.16.148.9 | IPv4
| h2 | ✅ 成功 | 46 | cloudflare | | 39 | ip.sb | 2606:4700:20::ac43:4bac | IPv6
| h2 | ✅ 成功 | 47 | cloudflare | | 66 | fbi.gov | 2606:4700::6810:94f4 | IPv6
| h2 | ✅ 成功 | 47 | cloudflare | | 67 | fbi.gov | 2606:4700::6810:95f4 | IPv6
| h2 | ✅ 成功 | 47 | cloudflare | | 86 | time.is | 2606:4700:20::ac43:449d |
IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 101 | www.gov.ua |
2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 110 |
172.64.159.6 | 172.64.159.6 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare | | 166 |
moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 47 |
cloudflare | | 209 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 |
h2 | ✅ 成功 | 47 | cloudflare | | 219 | whatismyipaddress.com |
2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 251 |
www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare
| | 259 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅
成功 | 47 | cloudflare | | 263 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 |
✅ 成功 | 47 | cloudflare | | 266 | bestcf.030101.xyz | 104.19.153.123 | IPv4 |
h2 | ✅ 成功 | 47 | cloudflare | | 272 | www.pcmag.com | 104.16.21.118 | IPv4 |
h2 | ✅ 成功 | 47 | cloudflare | | 280 | [2606:4700:4403::7357:544f] |
2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 288 |
456.cloudflare.182682.xyz | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 47 |
cloudflare | | 289 | 456.cloudflare.182682.xyz | 2606:4700:20::681a:8a0 | IPv6 |
h2 | ✅ 成功 | 47 | cloudflare | | 302 | dnschecker.org |
2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 326 |
asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare | | 349
| ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 47 |
cloudflare | | 362 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2
| ✅ 成功 | 47 | cloudflare | | 366 | cf.0sm.com | 2606:4700:3037::ac43:bb91 |
IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 378 | steamdb.info |
2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 380 |
ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 47 |
cloudflare | | 391 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 47 |
cloudflare | | 392 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 47 |
cloudflare | | 428 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 |
47 | cloudflare | | 443 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅
成功 | 47 | cloudflare | | 446 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 |
✅ 成功 | 47 | cloudflare | | 463 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 |
✅ 成功 | 47 | cloudflare | | 10 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 |
✅ 成功 | 48 | cloudflare | | 13 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70
| IPv6 | h2 | ✅ 成功 | 48 | cloudflare | | 37 | ip.sb | 104.26.13.31 | IPv4 |
h2 | ✅ 成功 | 48 | cloudflare | | 38 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅
成功 | 48 | cloudflare | | 77 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅
成功 | 48 | cloudflare | | 120 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅
成功 | 48 | cloudflare | | 127 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 |
✅ 成功 | 48 | cloudflare | | 158 | uriah.ns.cloudflare.com |
2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare | | 159 |
104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare | | 170 |
julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 48 |
cloudflare | | 193 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 48
| cloudflare | | 233 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 |
h2 | ✅ 成功 | 48 | cloudflare | | 236 | icook.tw | 104.20.28.74 | IPv4 | h2 |
✅ 成功 | 48 | cloudflare | | 246 | trevor.ns.cloudflare.com |
2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare | | 250 |
www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare | | 273 |
www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare | |
281 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare | |
285 | 456.cloudflare.182682.xyz | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 48 |
cloudflare | | 291 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 48 |
cloudflare | | 319 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 48
| cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 100 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

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

_此报告由 HTTP/3 连接测试报告生成器自动生成_
