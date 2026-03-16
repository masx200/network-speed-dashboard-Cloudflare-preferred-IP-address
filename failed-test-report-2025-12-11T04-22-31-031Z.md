# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 04:22:31
- **数据来源**: connectivity_results-20251211-042230.json
- **总测试数**: 483
- **失败测试数**: 4
- **成功测试数**: 479
- **失败率**: 0.83%
- **平均延迟**: 80.50ms
- **最小延迟**: 44ms
- **最大延迟**: 2002ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 4 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | -------------------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 45   | huxley.ns.cloudflare.com   | 108.162.195.188 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.188:443: i/o timeout |
| 95   | cfip.xxxxxxxx.tk           | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 201  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.205:443: i/o timeout |
| 311  | 172.64.201.25              | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 4 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 108.162（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 4 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 162 |
fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare | | 35 |
172.64.157.120 | 172.64.157.120 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare | | 58 |
www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 46 |
cloudflare | | 138 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 |
✅ 成功 | 46 | cloudflare | | 447 | cf.0sm.com | 2606:4700:3037::ac43:bb91 |
IPv6 | h2 | ✅ 成功 | 46 | cloudflare | | 120 | ip.sb | 2606:4700:20::ac43:4bac
| IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 158 | time.is |
2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 47 | cloudflare | | 292 |
www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare | | 413
| cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 |
47 | cloudflare | | 31 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅
成功 | 48 | cloudflare | | 39 | www.digitalocean.com | 2606:4700::6813:ad44 |
IPv6 | h2 | ✅ 成功 | 48 | cloudflare | | 56 | www.whatismyip.com |
2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare | | 186 |
eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare | | 338 |
icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare | |
346 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare | |
384 | 456.cloudflare.182682.xyz | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 48 |
cloudflare | | 460 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 |
h2 | ✅ 成功 | 48 | cloudflare | | 13 | ifconfig.co | 2606:4700:3037::6815:365b
| IPv6 | h2 | ✅ 成功 | 49 | cloudflare | | 21 | yx-auto.pages.dev |
2606:4700:3031::ac43:868b | IPv6 | h2 | ✅ 成功 | 49 | cloudflare | | 83 |
www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 90 |
www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 159 |
ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 336 |
icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 372 |
gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 373 |
cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 405 |
www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare | | 420
| tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare
| | 440 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 49 |
cloudflare | | 456 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 49 |
cloudflare | | 458 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅
成功 | 49 | cloudflare | | 466 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅
成功 | 49 | cloudflare | | 42 | comicabc.com | 2606:4700:3036::6815:400a | IPv6
| h2 | ✅ 成功 | 50 | cloudflare | | 57 | www.whatismyip.com |
2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare | | 65 |
japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare | |
89 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 50 |
cloudflare | | 157 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 50
| cloudflare | | 166 | 172.64.146.16 | 172.64.146.16 | IPv4 | h2 | ✅ 成功 | 50
| cloudflare | | 184 | 172.64.159.6 | 172.64.159.6 | IPv4 | h2 | ✅ 成功 | 50 |
cloudflare | | 195 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 50 |
cloudflare | | 197 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 50 |
cloudflare | | 213 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 50 |
cloudflare | | 253 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 50 |
cloudflare | | 322 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 |
✅ 成功 | 50 | cloudflare | | 334 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅
成功 | 50 | cloudflare | | 350 | bestcf.030101.xyz | 104.19.149.213 | IPv4 | h2
| ✅ 成功 | 50 | cloudflare | | 375 | cf.877774.xyz | 2606:4700:4406::ac40:9242
| IPv6 | h2 | ✅ 成功 | 50 | cloudflare | | 396 | ip.gs |
2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare | | 406 |
www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare | | 419
| tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare
| | 441 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 50 |
cloudflare | | 446 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功
| 50 | cloudflare | | 468 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功
| 50 | cloudflare | | 472 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功
| 50 | cloudflare | | 18 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 |
✅ 成功 | 51 | cloudflare | | 29 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2
| ✅ 成功 | 51 | cloudflare | | 34 | www.hugedomains.com |
2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 51 | cloudflare | | 38 |
www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 51 |
cloudflare | | 100 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 |
51 | cloudflare | | 106 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 145 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 146 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 161 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 170 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 223 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 226 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 231 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅
成功 | 51 | cloudflare | | 255 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4
| h2 | ✅ 成功 | 51 | cloudflare | | 270 | icook.hk | 2606:4700:3031::6815:5ad2
| IPv6 | h2 | ✅ 成功 | 51 | cloudflare | | 364 | 172.64.151.55 | 172.64.151.55
| IPv4 | h2 | ✅ 成功 | 51 | cloudflare | | 371 | gamer.com.tw | 104.18.2.197 |
IPv4 | h2 | ✅ 成功 | 51 | cloudflare | | 404 | cf.090227.xyz |
2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare | | 431 |
198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare | | 453 |
na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare | | 465 |
ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare | | 17 |
www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 52 | cloudflare
| | 20 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h2 | ✅ 成功 | 52 |
cloudflare | | 76 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 |
h2 | ✅ 成功 | 52 | cloudflare | | 82 | local-aria2-webui.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare | | 93 |
www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare
| | 103 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 52 |
cloudflare | | 176 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅
成功 | 52 | cloudflare | | 194 | singapore.com | 2606:4700:20::681a:d8c | IPv6 |
h2 | ✅ 成功 | 52 | cloudflare | | 196 | toy-people.com | 172.67.72.18 | IPv4 |
h2 | ✅ 成功 | 52 | cloudflare | | 207 | cmcc.877774.xyz | 104.16.149.11 | IPv4
| h2 | ✅ 成功 | 52 | cloudflare | | 214 | cmcc.877774.xyz | 104.16.148.5 | IPv4
| h2 | ✅ 成功 | 52 | cloudflare | | 229 | cmcc.877774.xyz | 104.16.149.7 | IPv4
| h2 | ✅ 成功 | 52 | cloudflare | | 370 | [2606:4700:4403::7357:544f] |
2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 52 | cloudflare | | 409 |
asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare | | 430
| www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 444 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare | |
461 | ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 |
52 | cloudflare | | 462 | ae8a9c24-83de.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare | | 12 |
ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | | 14 |
ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare
| | 15 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | |
60 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | | 107 |
cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | |
134 | 172.64.153.172 | 172.64.153.172 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 163 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare | | 174
| www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 31 条记录
- **快 (50-100ms)**: 69 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 4 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
