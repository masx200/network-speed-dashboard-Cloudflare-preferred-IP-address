# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:53:13
- **数据来源**: connectivity_results-20251210-185313.json
- **总测试数**: 483
- **失败测试数**: 6
- **成功测试数**: 477
- **失败率**: 1.24%
- **平均延迟**: 112.19ms
- **最小延迟**: 59ms
- **最大延迟**: 1981ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 6 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 71   | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 189  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 214  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 245  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 449  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 450  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 6 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 121.188（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 6 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 154 |
ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅ 成功 | 59 | cloudflare
| | 155 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 63 |
cloudflare | | 39 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅
成功 | 65 | cloudflare | | 206 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2
| ✅ 成功 | 65 | cloudflare | | 302 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅
成功 | 65 | cloudflare | | 294 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 |
✅ 成功 | 66 | cloudflare | | 7 | [2606:4700:4409::5b5b:7758] |
2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 14 |
www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 67 | cloudflare
| | 43 | 456.cloudflare.182682.xyz | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅
成功 | 67 | cloudflare | | 218 | bowen.ns.cloudflare.com |
2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 474 |
icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 25 |
172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 62 |
www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 81 |
tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 103 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | |
138 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | |
223 | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h2 | ✅ 成功
| 68 | cloudflare | | 293 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅
成功 | 68 | cloudflare | | 18 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2
| ✅ 成功 | 69 | cloudflare | | 80 | tasteatlas.com | 2606:4700::6811:2469 |
IPv6 | h2 | ✅ 成功 | 69 | cloudflare | | 82 | freeyx.cloudflare88.eu.org |
141.101.120.51 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 99 |
cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
124 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
144 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
171 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 |
69 | cloudflare | | 198 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 |
h2 | ✅ 成功 | 69 | cloudflare | | 209 | yx-auto.pages.dev | 172.66.47.112 |
IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 211 | yx-auto.pages.dev |
2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare | | 283 |
cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 320 |
sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 69
| cloudflare | | 477 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功
| 69 | cloudflare | | 29 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2
| ✅ 成功 | 70 | cloudflare | | 33 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 |
✅ 成功 | 70 | cloudflare | | 69 | cloudflare-ip.mofashi.ltd |
2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 70 | cloudflare | | 83 |
freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6 | h2
| ✅ 成功 | 70 | cloudflare | | 90 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅
成功 | 70 | cloudflare | | 101 | cf.zhetengsha.eu.org |
2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 70 | cloudflare | | 114 |
cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 121 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h2 | ✅ 成功 |
70 | cloudflare | | 134 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 |
70 | cloudflare | | 304 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 70
| cloudflare | | 308 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 70
| cloudflare | | 314 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅
成功 | 70 | cloudflare | | 386 | decker.ns.cloudflare.com |
2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 70 | cloudflare | | 430 |
palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 463 | yx-auto.pages.dev | 2606:4700:3033::ac43:a162 | IPv6 | h2 | ✅ 成功 | 70
| cloudflare | | 36 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 71
| cloudflare | | 59 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 |
71 | cloudflare | | 84 | freeyx.cloudflare88.eu.org |
2606:4700:3009::83da:e4f:b8a5 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare | | 91 |
zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 95 |
saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 118 |
na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 145 |
iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 188 |
cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 257
| ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 71 | cloudflare | | 284
| cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 291 |
time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 303 |
ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 323 |
eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 372 |
toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare
| | 374 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 71 |
cloudflare | | 427 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 71 |
cloudflare | | 461 | yx-auto.pages.dev | 172.67.161.98 | IPv4 | h2 | ✅ 成功 |
71 | cloudflare | | 10 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功
| 72 | cloudflare | | 12 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 |
72 | cloudflare | | 37 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 |
✅ 成功 | 72 | cloudflare | | 44 | 456.cloudflare.182682.xyz |
2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 47 |
172.64.148.15 | 172.64.148.15 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 54 |
172.64.229.249 | 172.64.229.249 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 68 |
cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 72 |
cloudflare | | 70 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6
| h2 | ✅ 成功 | 72 | cloudflare | | 74 | asia.877774.xyz | 104.17.142.146 |
IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 87 | www.udemy.com | 104.16.142.237 |
IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 112 | cf.0sm.com | 104.21.7.133 | IPv4
| h2 | ✅ 成功 | 72 | cloudflare | | 128 | steamdb.info |
2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 136 |
ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 149 |
[2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 |
72 | cloudflare | | 174 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅
成功 | 72 | cloudflare | | 193 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 |
✅ 成功 | 72 | cloudflare | | 199 | www.whatismyip.com | 2606:4700:20::681a:d17
| IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 212 | yx-auto.pages.dev |
2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 237 |
www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 72 |
cloudflare | | 252 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 72 |
cloudflare | | 298 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 72 |
cloudflare | | 324 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 72 |
cloudflare | | 442 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 |
h2 | ✅ 成功 | 72 | cloudflare | | 446 | wilson.ns.cloudflare.com |
2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 473 |
otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 72 |
cloudflare | | 30 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅
成功 | 73 | cloudflare | | 73 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅
成功 | 73 | cloudflare | | 105 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 |
h2 | ✅ 成功 | 73 | cloudflare | | 148 | iplocation.io | 2606:4700:20::ac43:4664
| IPv6 | h2 | ✅ 成功 | 73 | cloudflare | | 202 | japan.com | 104.26.4.60 | IPv4
| h2 | ✅ 成功 | 73 | cloudflare | | 203 | japan.com | 104.26.5.60 | IPv4 | h2 |
✅ 成功 | 73 | cloudflare | | 271 | [2606:4700:440f::53aa:4126] |
2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare | | 273 |
104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare | | 307
| 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare | | 335 |
icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare | |
342 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 73 |
cloudflare | | 366 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 73
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

- **IPv4 失败**: 6 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 6 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
